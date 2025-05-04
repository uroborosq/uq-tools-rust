use core::str;
use std::{env, error::Error, process::Command};

const XDG_CURRENT_DESKTOP: &str = "XDG_CURRENT_DESKTOP";
const SWAY_DESKTOP_ENVIRONMENT: &str = "sway";
const DEFAULT_RESTART_TIMES: u64 = 1000;

fn main() -> Result<(), Box<dyn Error>> {
    let desktop_environment = env::var(XDG_CURRENT_DESKTOP).map_err(|e| {
        format!(
            "Can't determine desktop environment! Error while reading {} env var: {}",
            XDG_CURRENT_DESKTOP, e
        )
    })?;

    if desktop_environment == SWAY_DESKTOP_ENVIRONMENT {
        return Err(format!(
            "Can be used only with {} desktop environment! Detected DE: {}",
            SWAY_DESKTOP_ENVIRONMENT, desktop_environment
        )
        .into());
    }

    let mut cmd = Command::new("waybar");
    let cmd = cmd.args(["--config", "$HOME/.config/waybar/config.json"]);

    for _ in 0..DEFAULT_RESTART_TIMES {
        let result = cmd.output();
        if let Err(err) = result {
            println!("can't spawn waybar process: {}", err)
        } else if let Ok(value) = result {
            println!("waybar process finished with code {:?}", value.status);
            let stdout_result = str::from_utf8(&value.stdout);
            if let Err(decode_error) = stdout_result {
                println!(
                    "can't decode waybar output due to utf8 decode error: {:?}",
                    decode_error
                );
            } else if let Ok(stdout) = stdout_result {
                println!("waybar process stdout: {:?}", stdout);
            }

            let stderr_result = str::from_utf8(&value.stdout);
            if let Err(decode_error) = stdout_result {
                println!(
                    "can't decode waybar stderr due to utf8 decode error: {:?}",
                    decode_error
                );
            } else if let Ok(stderr) = stderr_result {
                println!("waybar process stderr: {:?}", stderr);
            }
        }

        std::thread::sleep(std::time::Duration::from_millis(500));
    }

    Ok(())
}
