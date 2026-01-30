# HibouAir Desktop Air Quality Dashboard  
**Rust · Dioxus · BLE · BleuIO**

A cross-platform **desktop application built with Rust and Dioxus** that connects to a **BleuIO USB BLE dongle**, scans nearby **HibouAir air-quality sensors**, decodes their BLE advertisement data, and displays real-time environmental measurements in a clean desktop dashboard.

The application runs as a **native desktop window** and does **not require a browser**.

---

## Features

- Native **Rust desktop application** (macOS, Linux, Windows)
- Built with **Dioxus Desktop**
- Uses **BleuIO USB dongle** for BLE scanning via serial port
- Real-time scanning of **HibouAir sensors**
- Decodes BLE advertisement (Manufacturer Data – Company ID `0x075B`)
- Supports **CO₂** and **PM** HibouAir devices
- Stable decoding by accepting **Beacon Type `0x05` only**
- Clean UI with device-type header and metric panels

---

## Supported Sensors

- **HibouAir CO₂ sensors**
- **HibouAir PM sensors**

Each device advertises sensor data over BLE, which is decoded locally without cloud dependencies.

---

## How It Works

1. The app searches for a **BleuIO USB dongle** using its **VID/PID**
2. Opens the corresponding **serial port**
3. Sends initialization commands:
   - `ATE0` → disable echo  
   - `ATV1` → enable verbose mode
4. Starts BLE scanning using:  
   `AT+FINDSCANDATA=FF5B07`
5. BleuIO returns BLE advertisement packets as JSON
6. The app:
   - Extracts Manufacturer Specific Data (`0xFF`)
   - Verifies company ID `0x075B` (HibouAir)
   - Decodes **Beacon Type `0x05`** (full payload only)
7. Sensor values are parsed and rendered in the UI

---

## Project Structure

```
src/
├── components/
│   ├── dashboard.rs
│   ├── sensor_panel.rs
│   └── mod.rs
├── hooks/
│   ├── use_bleuio.rs
│   └── mod.rs
├── models/
│   ├── bleuio.rs
│   ├── hibouair.rs
│   ├── sensor_data.rs
│   └── mod.rs
├── main.rs
assets/
├── main.css
├── tailwind.css
└── favicon.ico
```

---

## Requirements

### Hardware
- **BleuIO USB BLE dongle**
- One or more **HibouAir sensors**

### Software
- Rust (stable)
- Dioxus CLI

---

## Installation

### Install Rust
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Install Dioxus CLI
```
cargo install dioxus-cli
```

### Clone repository
```
git clone <repo-url>
cd hibouair-desktop-dashboard
```

---

## Running the App

### Development (hot reload)
```
dx serve
```

### Using Cargo
```
cargo run
```

---

## Notes

- Only **Beacon Type `0x05`** is processed for stability
- No cloud dependency – all decoding is local
- Designed for real-time monitoring

---

## Credits

- HibouAir – Air quality sensors  
- BleuIO – USB BLE dongle  
- Dioxus – Rust UI framework  

---

Built with **Rust** for reliable, real-time air quality monitoring.
