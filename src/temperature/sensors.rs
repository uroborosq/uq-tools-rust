use std::{
    collections::{self, HashMap},
    error::Error,
    fs, io,
    path::Path,
};

pub fn get_sensors(sysfs_sensors_path: &Path) -> Result<collections::HashMap<String, collections::HashMap<String, f64>>, Box<dyn Error>> {
    let mut monitors_map: HashMap<String, HashMap<String, f64>> = HashMap::new();

    let monitor_dirs =
        fs::read_dir(sysfs_sensors_path).map_err(|e| io::Error::new(e.kind(), format!("can't open sensors directory due to error: {e}")))?;

    for monitor in monitor_dirs {
        let mut monitor_name: Option<String> = None;

        let monitor_dir = monitor.map_err(|e| io::Error::new(e.kind(), format!("can't access monitor directory due to error: {e}")))?;

        let path = monitor_dir.path();

        let sensor_files =
            fs::read_dir(path.clone()).map_err(|e| io::Error::new(e.kind(), format!("can't open {path:?} monitor directory due to error: {e}")))?;

        for sensor_file_res in sensor_files {
            let sensor_file = sensor_file_res.map_err(|e| io::Error::new(e.kind(), format!("can't open sensor file due to error: {e}")))?;

            // TODO: i wanna make it one line
            let file_name_os = sensor_file.file_name();
            let file_name = file_name_os
                .to_str()
                .ok_or_else(|| format!("file name is not in Unicode: {:?}", sensor_file.file_name()))?;

            println!("{file_name}");

            // name of monitor
            if file_name == "name" {
                monitor_name = Some(file_name.to_owned());
            }

            let Some((sensor_id, sensor_type)) = file_name.split_once("_") else {
                continue;
            };

            match sensor_type {
                "label" => {
                    let sensor_label = fs::read_to_string(sensor_file.path())?;
                }
                "input" => {
                    let input_string = fs::read_to_string(sensor_file.path())?;
                }
                _ => {
                    continue;
                }
            }
        }

        if monitor_name.is_none() {
            Err::<collections::HashMap<&str, collections::HashMap<&str, f64>>, &str>("failed to get monitor name");
        } else {
            monitors_map.insert(monitor_name.unwrap(), HashMap::new());
            // monitors_map[&monitor_name.unwrap().as_str()] = HashMap::new();
        }
    }

    Ok(monitors_map)
}
