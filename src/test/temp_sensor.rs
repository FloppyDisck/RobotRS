use crate::modules::output::{OutputType, Reading};
use crate::modules::sensor::{Features, SensorBuilder};
use crate::modules::{Descriptor, Metadata, Module};
use crate::Robot;
use bevy_ecs::prelude::*;
pub struct TemperatureComponent {
    sensors: Vec<(String, u8)>,
}

impl TemperatureComponent {
    pub fn new() -> Self {
        Self { sensors: vec![] }
    }

    pub fn add(mut self, name: &str, port: u8) -> Self {
        self.sensors.push((name.to_string(), port));
        self
    }
}

pub struct InitializedTemperatureSensors {
    pub sensors: Vec<(Entity, String, u8)>,
}

impl Module<InitializedTemperatureSensors> for TemperatureComponent {
    fn init(self, robot: &mut Robot) -> InitializedTemperatureSensors {
        let mut initialized = vec![];

        for (s_name, port) in self.sensors.iter() {
            let sensor_type = TemperatureSensor::new(*port);

            let s = SensorBuilder::new(&format!("Temperature - {}", s_name), robot)
                .with_type(&sensor_type)
                .with_output(OutputType::Temperature)
                .with_output(OutputType::Humidity)
                .with_component(sensor_type)
                .with_system(temperature_reading)
                .build();

            initialized.push((s, s_name.clone(), *port));
        }

        InitializedTemperatureSensors {
            sensors: initialized,
        }
    }
}

fn temperature_reading(
    mut query: Query<(&mut TemperatureSensor, &Features)>,
    mut reading: Query<(&mut Reading, &Metadata)>,
) {
    for (mut sensor, features) in query.iter_mut() {
        let sensor_reading = sensor.read();
        for feature in features.0.iter() {
            let (mut read, meta) = reading.get_mut(*feature).unwrap();

            let mut r = sensor_reading;
            if OutputType::Temperature.id() == meta.id {
                r *= 2.0;
            }

            read.0 = r;
        }
    }
}

// Sensor
#[derive(Component, Clone)]
pub struct TemperatureSensor {
    // Assume the moisture sensor only needs the port to read
    port: u8,

    // We use this as a counter
    last_read: f64,
}

impl TemperatureSensor {
    fn new(port: u8) -> Self {
        Self {
            port,
            last_read: 0.0,
        }
    }

    fn read(&mut self) -> f64 {
        self.last_read += 2.0;
        self.last_read
    }
}

impl Descriptor for TemperatureSensor {
    fn id(&self) -> u8 {
        101
    }

    fn name(&self) -> String {
        "Temperature Sensor".to_string()
    }

    fn description(&self) -> String {
        "Sensor used to log the temperature and humidity".to_string()
    }
}
