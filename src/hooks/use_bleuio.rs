use dioxus::prelude::*;
use std::collections::HashMap;
use serial2_tokio::SerialPort;
use tokio::io::{BufReader, AsyncBufReadExt, AsyncWriteExt};
use tokio::time::{timeout, Duration};
use futures_util::StreamExt;
use chrono::{DateTime, Local};

use crate::models::bleuio::*;
use crate::models::hibouair::*;

pub static COUNT: GlobalSignal<i32> = Signal::global(|| 0);
pub static LAST_TIME: GlobalSignal<DateTime<Local>> = Signal::global(|| Local::now());
pub static LAST_TIME_STR: GlobalSignal<String> = Signal::global(|| String::new());

const ATE0: &[u8; 6] = b"ATE0\r\n";
const ATV1: &[u8; 6] = b"ATV1\r\n";
const AT_FINDSCANDATA: &[u8;  24] = b"AT+FINDSCANDATA=FF5B07\r\n";

// Utility function for adding a sensor
fn add_sensor(mut sens: Signal<HashMap<u32, HibouAir>>, sensor: HibouAir) {
    sens.with_mut(|s| {
        s.insert(sensor.get_id(), sensor);
    });
}

// Utility function for logging (commented out in original, but might be useful)
#[cfg(feature = "logging")]
fn logga(mut log: Signal<String>, msg: &str) {
    // println!("{}", msg); // Debug print
    // log.with_mut(|l| l.push_str(msg));
}

pub fn use_bleuio(
    port_name: String,
    hibs: Signal<HashMap<u32, HibouAir>>,
    // mut log: Signal<String>, // Log removed or optional? Original code passed it but commented out usage mostly.
) -> Coroutine<BleuIOCommand> {
    
    use_coroutine(move |mut external_rx: UnboundedReceiver<BleuIOCommand>| {
        let port_name_for_async = port_name.clone();
        
        async move {
            // let mut app_state: AppState = AppState::OpenPort;

            // logga(log_handle, &format!("Försöker öppna {}\n", port_name_for_async));
            let port = match SerialPort::open(port_name_for_async, 115200) {
                Ok(p) => {
                    p.set_dtr(true).ok();
                    p.set_rts(true).ok();
                    p},
                Err(_e) => {
                    // logga(log_handle, &format!("Error: {}\n", e));
                    return;
                }
            };

            // Dela upp porten i läsare och skrivare för att kunna använda båda i select!
            let (reader, mut writer) = tokio::io::split(port);
            let mut buffered_reader = BufReader::new(reader);
            let mut read_buffer = String::new();

            // Current coomunicating state with the BleuIO dongle.
            let mut last_cmd: &[u8];

            // logga(log_handle, "Port öppen. Väntar...\n");

            // Skapa en intern kanal
            let (internal_tx, mut internal_rx) = futures_channel::mpsc::unbounded::<BleuIOCommand>();
            // let initial_tx = internal_tx.clone();

            // 1. Skicka initialt kommando direkt
            // initial_tx.unbounded_send(BleuIOCommand::At).ok();
            // writer.write_all(b"ATE0\r\n").await.ok();
            writer.write_all(ATE0).await.ok();
            last_cmd = ATE0;
            let mut last_error: i64 = 0;

            loop {
                tokio::select! {
                    // GREN 1: Läs inkommande data från USB (fram till LF)
                    res = timeout(Duration::from_secs(5), buffered_reader.read_line(&mut read_buffer)) => {
                        match res {
                            Ok(Ok(0)) => break, // Porten stängdes
                            Ok(Ok(_)) => {
                                let clean_line = read_buffer.trim_end_matches(['\r', '\n']).to_string();
                                read_buffer.clear();
                                // logga(log_handle, &format!("{}\n", clean_line));
                                match parse_bleuio_result(&clean_line) {
                                    Ok(v) => {
                                        let t = get_bleuio_result_type(&v);
                                        match &t {
                                            BleuIOResponseType::AcknowledgementResponse => {
                                                // Received line with possible error code - let's hope it is success!
                                                // In any case, save it for later.
                                                last_error = v["err"].as_i64().unwrap_or(-1); 
                                                // let ec = BleuIOErrorCode::try_from(last_error);
                                                // logga(log_handle, &format!("Error code: {}, msg: {}, ec: {:?}\n", last_error, &v["errMsg"], &ec));
                                            },
                                            BleuIOResponseType::EndResponse => {
                                                // Last line of response received.
                                                if last_error == 0 {
                                                    // logga(log_handle, "Operation slutförd utan fel.\n");
                                                    if last_cmd == ATE0 {
                                                        // Echo off successful
                                                        // logga(log_handle, "Echo avstängt\n");
                                                        writer.write_all(ATV1).await.ok();
                                                        last_cmd = ATV1;
                                                    } else if last_cmd == ATV1 {
                                                        // logga(log_handle, "Verbose läge aktiverat\n");
                                                        writer.write_all(AT_FINDSCANDATA).await.ok();
                                                        last_cmd = AT_FINDSCANDATA;
                                                    }
                                                } else {
                                                    // logga(log_handle, &format!("Operation slutförd med felkod {}\n", last_error));
                                                }
                                            },
                                            BleuIOResponseType::ScanFindDataResponse => {
                                                // Scan completed.
                                                // logga(log_handle, &format!("address: {} data: {}\n", &v["addr"], &v["data"]));
                                                let data = &v["data"].as_str().unwrap_or("");
                                                if data.len() > 60 {
                                                    match HibouAir::from_hex(data) {
                                                        Ok(hibou) => {
                                                            // ✅ Only accept the "full" HibouAir payload (Beacon 0x05)
                                                            if hibou.get_beacon_nr() != 0x05 {
                                                                continue;
                                                            }

                                                            add_sensor(hibs, hibou);
                                                            *LAST_TIME.write() = Local::now();
                                                            *LAST_TIME_STR
                                                                .write() = LAST_TIME.read().format("%Y-%m-%d %H:%M:%S").to_string();
                                                            COUNT.with_mut(|c| *c += 1);
                                                        }
                                                        Err(_e) => {
                                                            // ignore
                                                        }
                                                    }

                                                }
                                            },
                                            _ => {}
                                        }
                                    }
                                    Err(_e) => {
                                        // We may end up here for a couple of reasons:
                                        // 1. The line is not JSON (e.g. "OK" or "ERROR")
                                        // 2. The line is malformed JSON
                                        // logga(log_handle, &format!("JSON error: {}\n", e));
                                        // logga(log_handle, &format!("Rådata: {}\n", clean_line));
                                        if last_cmd == ATE0 {
                                            if clean_line == "ECHO OFF" {
                                                // Echo off successful
                                                // logga(log_handle, "Echo avstängt\n");
                                                writer.write_all(ATV1).await.ok();
                                                last_cmd = ATV1;
                                            // } else {
                                            //     logga(log_handle, "Fel vid avstängning av echo\n");
                                            }
                                        } else if last_cmd == ATV1 {
                                            if clean_line == "VERBOSE ON" {
                                                // logga(log_handle, "Verbose läge aktiverat\n");
                                                writer.write_all(AT_FINDSCANDATA).await.ok();
                                                last_cmd = AT_FINDSCANDATA;
                                            // } else {
                                            //     logga(log_handle, "Fel vid aktivering av verbose läge\n");
                                            }
                                        }
                                    }
                                }
                            }
                            Ok(Err(_e)) => {
                                // logga(log_handle, &format!("Läsfel: {}\n", e));
                                break;
                            }
                            Err(_) => {
                                // Detta händer om 5 sekunder går utan att read_line blir klar
                                // logga(log_handle, "Timeout.\n");
                            }
                        }
                    }

                    // GREN 2: Lyssna på kommandon från Dioxus UI (rx)
                    ext_msg = external_rx.next() => {
                        if let Some(cmd) = ext_msg {
                            internal_tx.unbounded_send(cmd).ok();
                        } else {
                            break; // Avsluta om UI-kanalen dör
                        }
                    }

                    // GREN 3: Här körs ALL logik (både från UI och interna triggers)
                    cmd_to_exec = internal_rx.next() => {
                        if let Some(cmd) = cmd_to_exec {
                            // logga(log_handle, &format!("Kör kommando: {:?}", cmd));
                            match cmd {
                                BleuIOCommand::At => {writer.write_all(b"AT\r\n").await.ok();},
                                BleuIOCommand::AtI=> {writer.write_all(b"ATI\r\n").await.ok();},
                                BleuIOCommand::AtCentral => {writer.write_all(b"AT+CENTRAL\r\n").await.ok();},
                                BleuIOCommand::AtFindscandata => {writer.write_all(b"AT+FINDSCANDATA=FF5B07\r\n").await.ok();},
                            }
                        }
                    }
                }
            }
        }
    })
}