use crate::modules::output::{OutputType, Reading};
use crate::modules::sensor::{Features, SensorBuilder};
use crate::modules::{Descriptor, Module};
use crate::Robot;
use bevy_ecs::prelude::*;
// Component
pub struct MoistureComponent {
    sensors: Vec<(String, u8)>,
}

impl MoistureComponent {
    pub fn new() -> Self {
        Self { sensors: vec![] }
    }

    pub fn add(mut self, name: &str, port: u8) -> Self {
        self.sensors.push((name.to_string(), port));
        self
    }
}

pub struct InitializedMoistureSensors {
    pub sensors: Vec<(Entity, String, u8)>,
}

impl Module<InitializedMoistureSensors> for MoistureComponent {
    fn init(self, robot: &mut Robot) -> InitializedMoistureSensors {
        let mut initialized = vec![];

        for (s_name, port) in self.sensors.iter() {
            let sensor_type = MoistureSensor::new(*port);

            let s = SensorBuilder::new(&format!("Moisture - {}", s_name), robot)
                .with_type(&sensor_type)
                .with_output(OutputType::Moisture)
                .with_system(moisture_reading)
                .with_component(sensor_type)
                .build();

            initialized.push((s, s_name.clone(), *port));
        }

        InitializedMoistureSensors {
            sensors: initialized,
        }
    }
}

fn moisture_reading(
    mut query: Query<(&mut MoistureSensor, &Features)>,
    mut reading: Query<&mut Reading>,
) {
    for (mut sensor, features) in query.iter_mut() {
        // Simulate some reading
        let mut read = reading.get_mut(features.0[0]).unwrap();
        read.0 = sensor.read();
    }
}

// Sensor
#[derive(Component, Clone)]
pub struct MoistureSensor {
    // Assume the moisture sensor only needs the port to read
    port: u8,

    // We use this as a counter
    last_read: f64,
}

impl MoistureSensor {
    fn new(port: u8) -> Self {
        Self {
            port,
            last_read: 0.0,
        }
    }

    fn read(&mut self) -> f64 {
        self.last_read += 11.0;
        self.last_read
    }
}

impl Descriptor for MoistureSensor {
    fn id(&self) -> u8 {
        100
    }

    fn name(&self) -> String {
        "Moisture Sensor".to_string()
    }

    fn description(&self) -> String {
        "Sensor used to log the soil moisture".to_string()
    }
}
