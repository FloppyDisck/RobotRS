use crate::modules::output::{OutputType, Reading};
use crate::modules::sensor::{Features, SensorBuilder};
use crate::modules::{Descriptor, Metadata, Module};
use crate::Robot;
use bevy_ecs::prelude::*;

#[derive(Component, Clone)]
pub struct DisplayComponent {
    temp: Entity,
    moisture: Entity,
}

impl DisplayComponent {
    pub fn new(temp: Entity, moisture: Entity) -> Self {
        Self { temp, moisture }
    }
}

impl Descriptor for DisplayComponent {
    fn id(&self) -> u8 {
        105
    }

    fn name(&self) -> String {
        "Display".to_string()
    }

    fn description(&self) -> String {
        "Display sensor".to_string()
    }
}

impl Module<Entity> for DisplayComponent {
    fn init(self, robot: &mut Robot) -> Entity {
        SensorBuilder::new("Display", robot)
            .with_type(&self)
            .with_system(display_data)
            .with_component(self.clone())
            .build()
    }
}

fn display_data(
    display_query: Query<&DisplayComponent>,
    features: Query<(&Features, &Metadata)>,
    reading: Query<(&Reading, &Metadata)>,
) {
    for display in display_query.iter() {
        let (temp_features, temp_meta) = features.get(display.temp).unwrap();
        let (moisture_features, moisture_meta) = features.get(display.moisture).unwrap();

        let (moisture_reading, _) = reading.get(moisture_features.0[0]).unwrap();
        let mut humidity_reading = 0.0;
        let mut temp_reading = 0.0;

        for feature in temp_features.0.iter() {
            let (read, meta) = reading.get(*feature).unwrap();

            if meta.id == OutputType::Temperature.id() {
                temp_reading = read.0;
            } else {
                humidity_reading = read.0;
            }
        }

        println!(
            "{:?} {:?} {:?} read {:?} moisture",
            moisture_meta.id, moisture_meta.name, moisture_meta.description, moisture_reading
        );
        println!(
            "{:?} {:?} {:?} read {:?} temperature and {:?} humidity",
            temp_meta.id, temp_meta.name, temp_meta.description, temp_reading, humidity_reading
        );
    }
}
