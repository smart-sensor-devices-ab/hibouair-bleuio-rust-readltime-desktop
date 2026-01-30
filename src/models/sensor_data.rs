use crate::models::hibouair::HibouAir;

pub enum SensorType {
    HibouAir,
    // Other sensor types can be added here in the future.
}

pub union SensorData {
    pub hibouair: HibouAir,
    // Other sensor types can be added here in the future.
}

pub struct Sensor {
    pub location: String,
    pub sensor_type: SensorType,
    pub data: SensorData,
}