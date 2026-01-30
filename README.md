# Example Dioxus/Rust application monitoring sensor devices using the [BleuIO](https://bleuio.com) dongle

This is my first try to develop a desktop application in Rust using the [Dioxus](https://dioxuslabs.com) framework.

The kinds of sensor that could be monitored by this application are the ones from [Smart Sensor Devices AB](https://smartsensordevices.com), specifically the [HibouAIR](https://smartsensordevices.com/our-products-and-solutions/) sensors.

## How it works
1. The app starts by trying to find a USB device with the Vendor ID and Product ID of the BlueIO dongle.
2. If valid device found, it tries to open the corresponding USB Serial port.
3. If open succeeded, turns echo off with the 'ATE0' command.
4. Enables verbose mode with the 'ATV1' command.
5. Starts scanning for sensor advertisment with the 'AT+FINDSCANDATA=FF5B07' command.

## Screenshot
![Screenshot](/img/SCR-20260126-lahq.png)

![Screenshot](/img/SCR-20260120-lqgp.png)

The application has been tested on macos and on my Raspberry Pi 5 with the latest Trixie distro.

## Project Structure
The src/ directory is organized into functional modules:

- src/components/: Contains UI components.
- dashboard.rs: The main dashboard view (formerly Hero component).
- sensor_panel.rs: The sensor display component.
- src/models/: Contains data models and parsing logic.
- bleuio.rs: BleuIO dongle definitions and response parsing.
- hibouair.rs: HibouAir sensor data structure and parsing.
- src/hooks/: Contains custom Dioxus hooks.
- use_bleuio.rs: Encapsulates the serial port communication logic.
- src/main.rs: Handles window configuration and mounting the App.

```
src/
├── components/
│   ├── dashboard.rs
│   ├── mod.rs
│   └── sensor_panel.rs
├── hooks/
│   ├── mod.rs
│   └── use_bleuio.rs
├── models/
│   ├── bleuio.rs
│   ├── hibouair.rs
│   └── mod.rs
└── main.rs
````
