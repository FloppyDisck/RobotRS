use crate::modules::{Descriptor, Metadata};
use crate::UndefinedType;
use bevy_ecs::prelude::*;
use std::ops::Deref;

/// Output setup helper
/// Each Output can be seen as its own Entity,
/// meaning that each of the Sensor's readings can be considered separate from it
pub struct OutputBuilder {
    metadata: Metadata,
}

impl OutputBuilder {
    /// Creates an Undefined output, an output without any metadata
    pub fn new() -> Self {
        Self {
            metadata: UndefinedType.metadata(),
        }
    }

    /// Define the output's type
    pub fn with_type<T: Descriptor + 'static>(mut self, output_type: T) -> Self {
        self.metadata = output_type.metadata();
        self
    }

    /// Handles setting up all the necessary output components
    pub fn build(self, sensor: &Entity, world: &mut World) -> Entity {
        world
            .spawn(OutputBundle {
                output: Output(sensor.clone()),
                meta: self.metadata,
                reading: Reading::default(),
            })
            .id()
    }
}

#[derive(Bundle, Debug)]
pub struct OutputBundle {
    output: Output,
    meta: Metadata,
    reading: Reading,
}

#[derive(Component, Debug)]
/// Output identifier that contains its host's id
pub struct Output(pub Entity);
impl Deref for Output {
    type Target = Entity;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component, Default, Debug)]
/// Output's reading
pub struct Reading(pub f64);
impl Reading {
    pub fn set(&mut self, n: f64) {
        self.0 = n;
    }
}
impl Deref for Reading {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component, Default, Debug)]
/// Output metadata descriptor
pub enum OutputType {
    #[default]
    Temperature,
    Humidity,
    Moisture,
}

impl Descriptor for OutputType {
    fn id(&self) -> u8 {
        match self {
            OutputType::Temperature => 0,
            OutputType::Humidity => 1,
            OutputType::Moisture => 2,
        }
    }

    fn name(&self) -> String {
        match self {
            OutputType::Temperature => "Temperature".to_string(),
            OutputType::Humidity => "Humidity".to_string(),
            OutputType::Moisture => "Moisture".to_string(),
        }
    }

    fn description(&self) -> String {
        match self {
            OutputType::Temperature => "Description".to_string(),
            OutputType::Humidity => "Description".to_string(),
            OutputType::Moisture => "Description".to_string(),
        }
    }
}
