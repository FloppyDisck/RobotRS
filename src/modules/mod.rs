use crate::Robot;
use bevy_ecs::prelude::*;

pub mod output;
pub mod sensor;
pub mod timer;

/// Here you will take care of initializing all your Sensors and Features
pub trait Module<T> {
    /// Initializes everything in your modules,
    /// you can optionally return information like entities
    /// for when users want to use that specific sensor/feature
    fn init(self, robot: &mut Robot) -> T;
}

#[derive(Component, Debug)]
pub struct Metadata {
    pub id: u8,
    pub name: String,
    pub description: String,
}

/// Describes the current entity being used, its very useful for logging
pub trait Descriptor {
    fn id(&self) -> u8;
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn metadata(&self) -> Metadata {
        Metadata {
            id: self.id(),
            name: self.name(),
            description: self.description(),
        }
    }
}

#[derive(Default, Debug)]
pub struct UndefinedType;

impl Descriptor for UndefinedType {
    fn id(&self) -> u8 {
        255
    }

    fn name(&self) -> String {
        "Undefined".to_string()
    }

    fn description(&self) -> String {
        "N/A".to_string()
    }
}
