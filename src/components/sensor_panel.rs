#![allow(non_snake_case)]
use dioxus::prelude::*;
use crate::models::hibouair::*;
use crate::hooks::use_bleuio::LAST_TIME_STR;

fn header_title(sensor: &HibouAir) -> String {
    match sensor.get_board_type() {
        HibouAirType::Co2Sensor => "CO2 Sensor".to_string(),
        HibouAirType::PmSensor => "PM Sensor".to_string(),
        _ => "Sensor".to_string(),
    }
}

#[component]
fn Metric(label: String, value: String) -> Element {
    rsx! {
        div { class: "flex flex-col gap-1",
            div { class: "text-sm font-semibold text-gray-700", "{label}" }
            div { class: "text-lg font-bold text-gray-900", "{value}" }
        }
    }
}

#[component]
fn SensorCard(header: String, id: String, children: Element) -> Element {
    rsx! {
        div {
            class: "rounded-xl overflow-hidden shadow-md border border-green-800/30",
            title: "Last seen: {LAST_TIME_STR}",

            // Header bar
            div { class: "bg-green-700 text-white px-6 py-4 flex items-center gap-10",
                div { class: "text-2xl font-bold", "{header}" }
                div { class: "text-xl font-semibold", "ID: {id}" }
            }

            // White body
            div { class: "bg-white px-6 py-5",
                {children}
            }
        }
    }
}

#[component]
fn SensorPanelCO2(sensor: HibouAir) -> Element {
    rsx! {
        SensorCard {
            header: header_title(&sensor),
            id: sensor.get_board_id_string(),

            div {
                class: "grid gap-8",
                style: "grid-template-columns: repeat(6, minmax(0, 1fr));",

                Metric { label: "CO2".to_string(),      value: format!("{} ppm", sensor.get_co2()) }
                //Metric { label: "VOC".to_string(),      value: if sensor.get_voc_view().is_empty() { "-".to_string() } else { sensor.get_voc_view() } }
                Metric { label: "Humidity".to_string(), value: format!("{:.0} %rh", sensor.get_hum()) }
                Metric { label: "Temp".to_string(),     value: format!("{:.1} °C", sensor.get_temp()) }
                Metric { label: "Pressure".to_string(), value: format!("{:.0} hPA", sensor.get_bar()) }
                Metric { label: "Light".to_string(),    value: format!("{} Lux", sensor.get_als()) }
            }
        }
    }
}

#[component]
fn SensorPanelCO2Noise(sensor: HibouAir) -> Element {
    rsx! {
        SensorCard {
            header: header_title(&sensor),
            id: sensor.get_board_id_string(),

            div {
                class: "grid gap-8",
                style: "grid-template-columns: repeat(6, minmax(0, 1fr));",

                Metric { label: "CO2".to_string(),      value: format!("{} ppm", sensor.get_co2()) }
                Metric { label: "VOC".to_string(),      value: if sensor.get_voc_view().is_empty() { "-".to_string() } else { sensor.get_voc_view() } }
                Metric { label: "VOC type".to_string(), value: if sensor.get_voc_type() == 0 { "Old".to_string() } else { "Resistance".to_string() } }
                Metric { label: "PM1.0".to_string(),    value: format!("{:.1} μg/m³", sensor.get_pm1_0()) }
                Metric { label: "PM2.5".to_string(),    value: format!("{:.1} μg/m³", sensor.get_pm2_5()) }
                Metric { label: "PM10".to_string(),     value: format!("{:.1} μg/m³", sensor.get_pm10()) }
                Metric { label: "Bar".to_string(),      value: format!("{} mbar", sensor.get_bar()) }
                Metric { label: "Humidity".to_string(), value: format!("{:.0} %rh", sensor.get_hum()) }
                Metric { label: "Temp".to_string(),     value: format!("{:.1} °C", sensor.get_temp()) }
            }
        }
    }
}

#[component]
fn SensorPanelPM(sensor: HibouAir) -> Element {
    rsx! {
        SensorCard {
            header: header_title(&sensor),
            id: sensor.get_board_id_string(),

            div {
                class: "grid gap-8",
                style: "grid-template-columns: repeat(6, minmax(0, 1fr));",

                Metric { label: "PM10".to_string(),     value: format!("{:.1} μg/m³", sensor.get_pm10()) }
                Metric { label: "PM2.5".to_string(),    value: format!("{:.1} μg/m³", sensor.get_pm2_5()) }
                Metric { label: "PM1.0".to_string(),    value: format!("{:.1} μg/m³", sensor.get_pm1_0()) }
                Metric { label: "Humidity".to_string(), value: format!("{:.0} %rh", sensor.get_hum()) }
                Metric { label: "Temp".to_string(),     value: format!("{:.1} °C", sensor.get_temp()) }
                Metric { label: "Pressure".to_string(), value: format!("{:.0} hPA", sensor.get_bar()) }
            }

            // Second row (optional) for VOC + Light (keeps 6-col grid clean)
            div {
                class: "grid gap-8 mt-6",
                style: "grid-template-columns: repeat(6, minmax(0, 1fr));",

                //Metric { label: "VOC".to_string(),   value: if sensor.get_voc_view().is_empty() { "-".to_string() } else { sensor.get_voc_view() } }
                //Metric { label: "Light".to_string(), value: format!("{} Lux", sensor.get_als()) }
                div {}
                div {}
                div {}
                div {}
            }
        }
    }
}

#[component]
fn SensorPanelUnknown(sensor: HibouAir) -> Element {
    rsx! {
        SensorCard {
            header: "Unknown Sensor".to_string(),
            id: sensor.get_board_id_string(),

            div { class: "text-gray-800",
                "Unknown sensor type for board ID: {sensor.get_board_id_string()}"
            }
        }
    }
}

#[component]
pub fn SensorPanel(sensor: HibouAir) -> Element {
    match sensor.get_board_type() {
        HibouAirType::Co2Sensor => rsx! { SensorPanelCO2 { sensor: sensor.clone() } },
        HibouAirType::PmSensor  => rsx! { SensorPanelPM  { sensor: sensor.clone() } },
        HibouAirType::Co2Noise  => rsx! { SensorPanelCO2Noise { sensor: sensor.clone() } },
        _ => rsx! { SensorPanelUnknown { sensor: sensor.clone() } },
    }
}
