use hex;
use zerocopy::{FromBytes, Unaligned, Immutable, KnownLayout};

pub enum VocType {
    Old = 0,
    Resistance = 1,
    Ppm = 2,
    Iaq = 3,
}

#[derive(Debug, Clone, PartialEq, Copy)]
pub enum HibouAirType {
    Unknown = 0x00,
    TempHumSensor = 0x02,
    PmSensor = 0x03,
    Co2Sensor = 0x04,
    No2OutdoorWifi = 0x05,
    Co2Battery = 0x06,
    No2OutdoorLtemNbiot = 0x07,
    PirSensor = 0x08,
    Co2Noise = 0x09,
    DuoMaster = 0x0A,
    DuoSlave = 0x0B,
    Matrix = 0x14,
}

impl TryFrom<i64> for HibouAirType {
    type Error = &'static str;

    fn try_from(value: i64) -> std::result::Result<Self, Self::Error> {
        match value {
            0x00 => Ok(HibouAirType::Unknown),
            0x02 => Ok(HibouAirType::TempHumSensor),
            0x03 => Ok(HibouAirType::PmSensor),
            0x04 => Ok(HibouAirType::Co2Sensor),
            0x05 => Ok(HibouAirType::No2OutdoorWifi),
            0x06 => Ok(HibouAirType::Co2Battery),
            0x07 => Ok(HibouAirType::No2OutdoorLtemNbiot),
            0x08 => Ok(HibouAirType::PirSensor),
            0x09 => Ok(HibouAirType::Co2Noise),
            0x0A => Ok(HibouAirType::DuoMaster),
            0x0B => Ok(HibouAirType::DuoSlave),
            0x14 => Ok(HibouAirType::Matrix),
            _ => Err("Unkown sensor type"),
        }
    }
}

// #[derive(Debug, Clone, PartialEq, Copy)]
#[repr(C, packed)]
#[derive(FromBytes, Unaligned, Immutable, KnownLayout, Debug, Clone, Copy, PartialEq)]
pub struct HibouAir {
    mfid: u16,          // the manufacturer id of the device
    beacon_nr: u8,      // type of beacon
    board_type: u8,     // type of device
    board_id: [u8;3],   // unique board id
    als: u16,           // ambient light sensor
    bar: u16,           // pressure
    temp: u16,          // temperature
    hum: u16,           // humidity
    voc: u16,           // volatile organic compounds
    pm1_0: u16,         // particle matter PM1.0
    pm2_5: u16,         // particle matter PM2.5
    pm10: u16,          // particle matter PM10.0
    co2: u16,           // carbon dioxide
    voc_type: u8,       // 0 = old, 1 = resistance, 2 = ppm, 3 = IAQ
}

impl HibouAir {
    /// Tar en hex-sträng och försöker konvertera den till en HibouAir-struct
    pub fn from_hex(hex_str: &str) -> std::result::Result<Self, String> {
        let bytes = hex::decode(hex_str)
            .map_err(|e| format!("Invalid hex string: {e}"))?;

        // Walk through BLE AD structures:
        // [len][type][payload...]
        let mut i = 0usize;
        while i < bytes.len() {
            let len = bytes[i] as usize;
            if len == 0 { break; }

            let end = i + 1 + len;
            if end > bytes.len() { break; }

            let ad_type = bytes[i + 1];

            // Manufacturer specific data
            if ad_type == 0xFF && len >= 3 {
                // Company ID is 2 bytes little-endian
                let company_lo = bytes[i + 2];
                let company_hi = bytes[i + 3];

                // 0x075B => bytes = 5B 07
                if company_lo == 0x5B && company_hi == 0x07 {
                    // Start struct at company id (same as your old bytes[5..] assumption)
                    let mfg = &bytes[i + 2..end];

                    let (data, _rest) = Self::read_from_prefix(mfg)
                        .map_err(|_| "Manufacturer data too short for HibouAir format".to_string())?;

                    return Ok(data);
                }
            }

            i = end;
        }

        Err("No HibouAir manufacturer data (0xFF, company 0x075B) found in advertisement".to_string())
    }

    #[cfg(feature = "using_ble")]
    /// Tar en byte-slice från en BLE-annons och försöker konvertera den till en HibouAir-struct
    pub fn from_ble(data: &[u8]) -> std::result::Result<Self, String> {
        if data.len() < std::mem::size_of::<HibouAir>() {
            return Err("Datan är för kort för att matcha HibouAir-formatet".to_string());
        }

        let (hibouair, _rest) = Self::read_from_prefix(data)
            .map_err(|_| "Misslyckades med att läsa HibouAir från BLE-data")?;

        Ok(hibouair)
    }

    // Return a string representation of the HibouAir struct.
    pub fn to_string(&self) -> String {
        format!(
            "HibouAir(mfid: {}, beacon_nr: {}, board_type: {}, board_id: {:02X?}, als: {}, bar: {}, temp: {}, hum: {}, voc: {}, pm1_0: {}, pm2_5: {}, pm10: {}, co2: {}, voc_type: {})",
            {self.mfid},
            self.beacon_nr,
            self.board_type,
            self.board_id,
            {self.als},
            {self.bar},
            {self.temp},
            {self.hum},
            {self.voc},
            {self.pm1_0},
            {self.pm2_5},
            {self.pm10},
            {self.co2},
            self.voc_type
        )
    }

    // Getter methods for each field.

    // Return MFID of device.
    pub fn get_mfid(&self) -> u16 {
        self.mfid
    }

    // Return beacon number.
    pub fn get_beacon_nr(&self) -> u8 {
        self.beacon_nr
    }

    // Return board type as SensorType.
    pub fn get_board_type(&self) -> HibouAirType {
        HibouAirType::try_from(self.board_type as i64).unwrap_or(HibouAirType::Unknown) 
    }

    // Return board type as string.
    pub fn get_board_type_string(&self) -> String {
        match self.get_board_type() {
            HibouAirType::PmSensor => "PM".to_string(),
            HibouAirType::Co2Sensor => "CO2".to_string(),
            HibouAirType::TempHumSensor => "Temp/Hum".to_string(),
            HibouAirType::No2OutdoorWifi => "NO2 Outdoor WiFi".to_string(),
            HibouAirType::Co2Battery => "CO2 Battery".to_string(),
            HibouAirType::No2OutdoorLtemNbiot => "NO2 Outdoor LTEM NBIOT".to_string(),
            HibouAirType::PirSensor => "PIR".to_string(),
            HibouAirType::Co2Noise => "CO2 Noise".to_string(),
            HibouAirType::DuoMaster => "Duo Master".to_string(),
            HibouAirType::DuoSlave => "Duo Slave".to_string(),
            HibouAirType::Matrix => "Matrix".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    // Return board ID as u32.
    pub fn get_id(&self) -> u32 {
        ((self.board_id[0] as u32) << 16) | ((self.board_id[1] as u32) << 8) | (self.board_id[2] as u32)
    }

    // Return board ID as hex string.
    pub fn get_board_id_string(&self) -> String {
        format!("{:06X}", self.get_id())
    }

    // Return ambient light sensor value.
    pub fn get_als(&self) -> u16 {
        self.als
    }

    // Return barometric pressure value.
    pub fn get_bar(&self) -> f64 {
        self.bar as f64 / 10.0
    }

    // Return temperature value.
    pub fn get_temp(&self) -> f64 {
        (self.temp as i16) as f64 / 10.0
    }

    // Return humidity value.
    pub fn get_hum(&self) -> f64 {
        self.hum as f64 / 10.0
    }

    // Return CO2 value.
    pub fn get_co2(&self) -> u16 {
        self.co2.swap_bytes()
    }

    // Return VOC value.
    pub fn get_voc(&self) -> f64 {
        let mut v: f64 = self.voc as f64 ;
        if self.voc_type == 2 {
            v = v / 100.0;
        }
        v
    }

    // Return VOC type.
    pub fn get_voc_type(&self) -> u8 {
        self.voc_type
    }

    // Return VOC unit as string.
    pub fn get_voc_unit(&self) -> String {
        // println!("Voc type: {}", self.voc_type);
        match self.voc_type {
            0 => "".to_string(),
            1 => "".to_string(),
            2 => "ppm".to_string(),
            3 => "IAQ".to_string(),
            _ => "".to_string(),
        }
    }

    // Return VOC value with unit as string.
    pub fn get_voc_view(&self) -> String {
        match self.voc_type {
            0 => "".to_string(),
            1 => "".to_string(),
            2 => format!("{:.1} {}",self.get_voc(), self.get_voc_unit()),
            3 => format!("{:.1} {}",self.get_voc(), self.get_voc_unit()),
            _ => "".to_string(),
        }
    }

    // Return PM1.0 value.
    pub fn get_pm1_0(&self) -> f64 {
        self.pm1_0 as f64 / 10.0
    }

    // Return PM2.5 value.
    pub fn get_pm2_5(&self) -> f64 {
        self.pm2_5 as f64 / 10.0
    }

    // Return PM10 value.
    pub fn get_pm10(&self) -> f64 {
        self.pm10 as f64 / 10.0
    }
}

// 0201061BFF5B07050422005A0000BA27C60017013E0000000000000001C002
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_hibouair_from_hex() {
        let hex_str = "0201061BFF5B07050422005A0000BA27C60017013E0000000000000001C002";
        match HibouAir::from_hex(hex_str) {
            Ok(hibouair) => {
                assert_eq!(hibouair.get_mfid(), 0x075B, "Checking MFID value");
                assert_eq!(hibouair.get_beacon_nr(), 0x05, "Checking beacon number");
                assert_eq!(hibouair.get_board_type(), HibouAirType::Co2Sensor, "Checking board type");
                assert_eq!(hibouair.get_id(), 0x22005A, "Checking board ID value");
                assert_eq!(hibouair.get_als(), 0x0000, );
                assert_eq!(hibouair.get_bar(), 1017.0, "Checking pressure value");
                assert_eq!(hibouair.get_temp(), 19.8, "Checking temperature value");
                assert_eq!(hibouair.get_hum(), 27.9, "Checking humidity value");
                assert_eq!(hibouair.get_voc(), 0.62, "Checking VOC value");
                assert_eq!(hibouair.get_pm1_0(), 0.0, "Checking PM1.0 value");
                assert_eq!(hibouair.get_pm2_5(), 0.0, "Checking PM2.5 value");
                assert_eq!(hibouair.get_pm10(), 0.0, "Checking PM10 value");
                assert_eq!(hibouair.get_co2(), 448, "Checking CO2 value");
                assert_eq!(hibouair.get_voc_type(), 2, "Checking VOC type");
            },
            Err(_) => panic!("Unexpected error parsing HibouAir from hex"),
        }
    }

    #[cfg(feature = "using_ble")]
    #[test]
    fn test_hibouair_from_ble() {
        todo!();
    }
}
