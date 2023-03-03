pub mod modules;

#[cfg(test)]
mod test;

// Used for building modules
pub mod dev {
    pub use crate::modules::{output::OutputBuilder, sensor::SensorBuilder, Descriptor, Module};
    pub use crate::prelude::*;
}

// Used for normal users
pub mod prelude {
    pub use crate::modules::{
        output::{Output, OutputType, Reading},
        sensor::{Features, SensorBuilder},
        timer::{Duration, Timer},
        Metadata,
    };
    pub use crate::{DefaultStage, Robot};
}

// TODO: To be able to pub use prelude i need to port the macro libs
use bevy_ecs::prelude::*;

use crate::modules::{Module, UndefinedType};

// Before we build the framework

// TODO: allow to specify multiple labels
#[derive(StageLabel)]
pub struct DefaultStage;

/// Runtime
pub struct Robot {
    world: World,
    scheduler: Schedule,
}

impl Robot {
    pub fn new() -> Self {
        let mut scheduler = Schedule::default();
        scheduler.add_stage(DefaultStage, SystemStage::single_threaded());

        Self {
            world: World::new(),
            scheduler,
        }
    }

    pub fn run(&mut self) {
        self.scheduler.run(&mut self.world);
    }

    /// Adds a module, this only returns Self to follow the builder pattern
    pub fn with<T: Module<E>, E>(mut self, module: T) -> Self {
        self.add(module);
        self
    }

    /// Adds a module while also returning the module's initialization response
    pub fn add<T: Module<E>, E>(&mut self, module: T) -> E {
        module.init(self)
    }

    pub fn add_system<F, Params>(&mut self, system: F)
    where
        F: IntoSystemDescriptor<Params>,
    {
        self.scheduler.add_system_to_stage(DefaultStage, system);
    }

    pub fn with_system<F, Params>(mut self, system: F) -> Self
    where
        F: IntoSystemDescriptor<Params>,
    {
        self.add_system(system);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    fn simulate_readings(mut query: Query<&mut Reading>) {
        for mut reading in &mut query {
            reading.0 += 1.0;
        }
    }

    fn print_readings(query: Query<(&Output, &Reading)>) {
        for (output, reading) in &query {
            println!("{:?} read {:?}", output.0, reading.0);
        }
    }

    fn init() {
        let mut robot = Robot::new()
            .with_system(simulate_readings)
            .with_system(print_readings);

        SensorBuilder::new("SHT31", &mut robot)
            .with_type(&UndefinedType)
            .with_output(OutputType::Humidity)
            .with_output(OutputType::Temperature)
            .build();

        for _ in 0..10 {
            robot.run();
        }
    }

    #[test]
    fn it_works() {
        init()
    }
}
