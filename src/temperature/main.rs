use std::{error::Error, path};

mod sensors;

static SENSORS_PATH: &str = "/sys/class/hwmon";

fn main() -> Result<(), Box<dyn Error>> {
    let parsed_sensors = sensors::get_sensors(path::Path::new(SENSORS_PATH))?;

    println!("{:?}", parsed_sensors);
    Ok(())
}
