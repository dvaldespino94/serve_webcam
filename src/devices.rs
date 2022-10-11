use std::{os::unix::process::CommandExt, path::Path, process::Command};

use regex::Regex;

#[derive(Debug)]
struct Device {
    id: String,
    name: String,
    path: String,
}

fn get_devices() -> Vec<Device> {
    #[cfg(target_os = "linux")]
    let V4L_COMMAND: &str = "v4l2-ctl";
    #[cfg(target_os = "macos")]
    let V4L_COMMAND: &str = "./fake-v4l-ctl.sh";

    let raw_data = match std::process::Command::new(V4L_COMMAND)
        .arg("--list-devices")
        .output()
    {
        Ok(output_info) => output_info.stdout,
        Err(err) => {
            println!("Error running v4l: {err:?}");
            return Vec::new();
        }
    };

    let data = String::from_utf8(raw_data).unwrap();

    let lines: Vec<&str> = data.split("\n").into_iter().map(|x| x.trim()).collect();

    let mut devices = Vec::new();
    let re = Regex::new(r"(?P<name>.*) \((?P<id>.*)\):").unwrap();

    for device_definition in lines.split(|x| x.is_empty()) {
        if device_definition.len() < 2 {
            continue;
        }

        if let Some(caps) = re.captures(device_definition.get(0).unwrap()) {
            let device_name = caps.name("name").unwrap().as_str();
            let device_id = caps.name("id").unwrap().as_str();
            let device_path = device_definition.get(1).unwrap().to_owned();

            devices.push(Device {
                id: device_id.to_string(),
                name: device_name.to_string(),
                path: device_path.to_string(),
            })
        }
    }

    devices
}

pub fn list_devices_subcommand() {
    let devices = get_devices();
    println!("Listing {} devices:", devices.len());

    for device in devices {
        println!(
            "\t- V4L {}: ({}) \"{}\"",
            device.id, device.path, device.name
        )
    }
}

pub fn serve_webcam_subcommand(device_id: String, rtsp_port: u16, rtsp_path: String) {
    if let Some(device) = get_devices().iter().find(|dev| dev.id == device_id) {
        println!(
            "Serving device '{}' located at {}",
            device.name, device.path
        );

        let rtsp_path_str = format!("rtsp://localhost:{}/{}",rtsp_port, rtsp_path);

        let mut command = Command::new("ffmpeg");

        command
            .arg("-hide_banner")
            .args(["-f", "v4l2"])
            .args(["-i", device.path.as_str()])
            .args(["-pix_fmt", "yuv420p"])
            .args(["-preset", "ultrafast"])
            .arg("-b:v 600k")
            .args(["-f", "rtsp"])
            .arg(rtsp_path_str);

        println!("Command: {:?}", command);
        command.exec();
    } else {
        println!("Device {device_id} wasn't found!");
    }
}
