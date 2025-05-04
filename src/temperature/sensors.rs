use std::{
    collections::{self, HashMap},
    error::Error,
    fs, io,
    path::Path,
};

#[derive(Debug)]
pub struct Sensor {
    sensor_id: String,
    value: f64,
}

pub fn get_sensors(
    sysfs_sensors_path: &Path,
) -> Result<collections::HashMap<&str, collections::HashMap<&str, f64>>, Box<dyn Error>> {
    let monitors_map: HashMap<&str, HashMap<&str, f64>> = HashMap::new();

    let monitor_dirs = fs::read_dir(sysfs_sensors_path)
        .map_err(|e| io::Error::new(e.kind(), format!("can't open sensors directory due to error: {:?}", e)))?;

    for monitor in monitor_dirs {
        let mut monitor_name: Option<String> = None;

        let monitor_dir = monitor.map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("can't access monitor directory due to error: {:?}", e),
            )
        })?;

        let path = monitor_dir.path();

        let sensor_files = fs::read_dir(path.clone()).map_err(|e| {
            io::Error::new(
                e.kind(),
                format!("can't open {:?} monitor directory due to error: {:?}", path, e),
            )
        })?;

        for sensor_file_res in sensor_files {
            let sensor_file = sensor_file_res
                .map_err(|e| io::Error::new(e.kind(), format!("can't open sensor file due to error: {:?}", e)))?;

            let file_name_os = sensor_file.file_name();
            let file_name = file_name_os
                .to_str()
                .ok_or_else(|| format!("file name is not in Unicode: {:?}", sensor_file.file_name()))?;

            // name of monitor
            if file_name == "name" {
                monitor_name = Some(file_name.to_owned());
            }

            let Some(splitted) = file_name.split_once("_") else {
                continue;
            };

            match splitted.1 {
                "label" => {}
                "input" => {}
                _ => {}
            }
        }

        if monitor_name.is_none() {
            Err::<collections::HashMap<&str, collections::HashMap<&str, f64>>, &str>("failed to get monitor name");
        }
    }

    Ok(monitors_map)
}
