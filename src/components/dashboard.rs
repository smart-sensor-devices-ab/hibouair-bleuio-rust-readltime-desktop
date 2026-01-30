use dioxus::prelude::*;
use std::collections::HashMap;

use crate::models::hibouair::*;
// use crate::models::bleuio::*; 
use crate::components::sensor_panel::*;
use crate::hooks::use_bleuio::*;

#[component]
pub fn Dashboard(port_name: String) -> Element {
    let sensor_hash: HashMap<u32, HibouAir> = HashMap::new();
    let hibs = use_signal(|| sensor_hash.clone());
    let mut log = use_signal(|| String::new());
    
    let _serial_task = use_bleuio(port_name, hibs);

    rsx! {
        div {
            // img { src: HEADER_SVG, id: "header" }
            // style: "font-family: monospace; padding: 20px;",
            // h1 { "HibouAIR Monitor" }
            if cfg!(feature = "logging") {
                div { style: "background: rgb(128, 128, 128); height: 300px; overflow-y: scroll; margin-bottom: 10px;",
                    pre { "{log}" }
                }

                button {
                    class: "border p-1 rounded-md bg-gray-500",
                    onclick: move |_| log.set(String::new()),
                    "Clear log"
                }
            }

            div {
                // Horizontal container for all panel groups
                class: "flex flex-col gap-8 p-4",
                // Note: flex-row is the default for 'flex', but explicit is fine.
                // gap-8 (2rem/32px) adds space between each group of 3.
                for sensor in hibs.read().values() {
                    {
                        rsx! {
                            div {
                                SensorPanel { sensor: sensor.clone() }
                            }
                        }
                    }
                }
            }
        }
    }
}