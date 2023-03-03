use crate::modules::timer::{timer_system, Duration, Timer};
use crate::modules::{output::OutputBuilder, Descriptor, Metadata, UndefinedType};
use crate::Robot;
use bevy_ecs::prelude::*;
use std::ops::Deref;

/// Sensor setup helper
/// The sensors can be seen as the parent entity that contain
/// reference to all of its Output entities.
pub struct SensorBuilder<'c> {
    robot: &'c mut Robot,
    name: String,
    metadata: Metadata,
    outputs: Vec<OutputBuilder>,
    timer: Option<Duration>,
    entity: Entity,
}

impl<'c> SensorBuilder<'c> {
    /// Created an undefined sensor with no outputs
    pub fn new(name: &str, robot: &'c mut Robot) -> Self {
        let entity = robot.world.spawn(()).id();

        Self {
            name: name.into(),
            robot,
            metadata: UndefinedType.metadata(),
            outputs: vec![],
            timer: None,
            entity,
        }
    }

    /// Sets the outputs type, which contains helpful metadata for loggers
    pub fn with_type<T: Descriptor + 'static>(mut self, sensor_type: &T) -> Self {
        self.set_type(sensor_type);
        self
    }

    pub fn set_type<T: Descriptor + 'static>(&mut self, sensor_type: &T) {
        self.metadata = sensor_type.metadata();
    }

    /// Registers a new output
    pub fn with_output<T: Descriptor + 'static>(mut self, output: T) -> Self {
        self.set_output(output);
        self
    }

    pub fn set_output<T: Descriptor + 'static>(&mut self, output: T) {
        self.outputs.push(OutputBuilder::new().with_type(output));
    }

    /// Registers and adds a new timer
    pub fn with_timer(mut self, duration: Option<Duration>) -> Self {
        self.timer = duration;
        self.with_system(timer_system)
    }

    pub fn set_timer(mut self, duration: Option<Duration>) {
        self.timer = duration;
        self.set_system(timer_system);
    }

    /// Registers a system if its not already present
    pub fn with_system<F, Params>(self, system: F) -> Self
    where
        F: IntoSystemDescriptor<Params>,
    {
        self.robot.add_system(system);
        self
    }

    pub fn set_system<F, Params>(self, system: F)
    where
        F: IntoSystemDescriptor<Params>,
    {
        self.robot.add_system(system);
    }

    pub fn add_component<T: Component>(&mut self, component: T) {
        self.robot
            .world
            .get_entity_mut(self.entity)
            .unwrap()
            .insert(component);
    }

    pub fn with_component<T: Component>(mut self, component: T) -> Self {
        self.add_component(component);
        self
    }

    /// Handles setting up all the necessary sensor components
    pub fn build(self) -> Entity {
        // Encapsulating this helps us borrow world multiple times
        {
            let mut ent = self.robot.world.get_entity_mut(self.entity).unwrap();

            ent.insert((Name(self.name), self.metadata));

            if let Some(timer) = self.timer {
                ent.insert(Timer::new(timer));
            }
        }

        let mut feature_entities = vec![];
        for feature in self.outputs {
            feature_entities.push(feature.build(&self.entity, &mut self.robot.world));
        }

        self.robot
            .world
            .get_entity_mut(self.entity)
            .unwrap()
            .insert(Features(feature_entities));

        self.entity
    }
}

#[derive(Component, Default, Debug)]
/// Represents the sensors unique type
pub struct Name(pub String);
impl Deref for Name {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Component, Default, Debug)]
/// Sensor's associated features
pub struct Features(pub Vec<Entity>);
impl Deref for Features {
    type Target = Vec<Entity>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Features {
    /// Used to find sensors with features
    pub fn is_sensor(&self) -> bool {
        !self.is_empty()
    }
}
