mod display;
mod moisture_sensor;
mod temp_sensor;

#[cfg(test)]
mod tests {
    use crate::test::display::DisplayComponent;
    use crate::test::moisture_sensor::MoistureComponent;
    use crate::test::temp_sensor::TemperatureComponent;
    use crate::Robot;

    #[test]
    fn demo_application() {
        let mut robot = Robot::new();

        let temperature_sensor =
            robot.add(TemperatureComponent::new().add("Temperature Sensor", 10));
        let moisture_sensor = robot.add(MoistureComponent::new().add("Moisture Sensor", 100));

        robot.add(DisplayComponent::new(
            temperature_sensor.sensors[0].0,
            moisture_sensor.sensors[0].0,
        ));
        
        for _ in 0..10 {
            robot.run();
        }
    }
}
