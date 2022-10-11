use std::process::Command;

use regex::Regex;

//Device Information
#[derive(Debug)]
struct Device {
    //The device id
    id: String,
    //The name reported by v4l2-ctl --list-devices
    name: String,
    //The device path (probably, but not necesarily /dev/video#)
    path: String,
}

fn get_devices() -> Vec<Device> {
    //Run the real v4l2-ctl on linux
    #[cfg(target_os = "linux")]
    const V4L_COMMAND: &str = "v4l2-ctl";

    //Run a fake v4l2-ctl on macos for development/debugging/testing purposes
    #[cfg(target_os = "macos")]
    const V4L_COMMAND: &str = "./fake-v4l-ctl.sh";

    //Get the command's output
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

    //Convert the output into a string
    let data = String::from_utf8(raw_data).unwrap();

    //Split the output into lines, and trim them
    let lines: Vec<&str> = data.split("\n").into_iter().map(|x| x.trim()).collect();

    //Device list to be filled
    let mut devices = Vec::new();

    //Regex to match the device name and id
    let re = Regex::new(r"(?P<name>.*) \((?P<id>.*)\):").unwrap();

    //Iterate over the device definitions
    for device_definition in lines.split(|x| x.is_empty()) {
        //We need at least 2 lines to go on
        if device_definition.len() < 2 {
            continue;
        }

        //If the regex matches
        if let Some(caps) = re.captures(device_definition.get(0).unwrap()) {
            //Get the device name from the regex
            let device_name = caps.name("name").unwrap().as_str();
            //Get the device id from the regex
            let device_id = caps.name("id").unwrap().as_str();
            //Get the first device path listed
            let device_path = device_definition.get(1).unwrap().to_owned();

            devices.push(Device {
                id: device_id.to_string(),
                name: device_name.to_string(),
                path: device_path.to_string(),
            })
        }
    }

    //Return the devices
    devices
}

//List devices
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

//Search for a device id and serve from that device
pub fn serve_webcam_subcommand(device_id: String, rtsp_port: u16, rtsp_path: String) {
    //Search for the device
    if let Some(device) = get_devices().iter().find(|dev| dev.id == device_id) {
        //If the device was found report it
        println!(
            "Serving device '{}' located at {}",
            device.name, device.path
        );

        //Get the rtsp full path from the parameters
        let rtsp_path_str = format!("rtsp://localhost:{}/{}", rtsp_port, rtsp_path);

        //Initialize the command to be executed
        let mut command = Command::new("ffmpeg");

        //Add parameters to the command
        command
            //Avoid ffmpeg's header on each execution
            .arg("-hide_banner")
            //v4l2 input format
            .args(["-f", "v4l2"])
            //Input device
            .args(["-i", device.path.as_str()])
            //Pixel format
            .args(["-pix_fmt", "yuv420p"])
            //Preset
            .args(["-preset", "ultrafast"])
            //Video bitrate
            .args(["-b:v", "600k"])
            //Output format
            .args(["-f", "rtsp"])
            //Output path
            .arg(rtsp_path_str);

        //Prints the output command
        println!("Command: {:?}", command);

        //Execute the output command(linux only)
        #[cfg(target_os = "linux")]
        command.exec();
    } else {
        println!("Device {device_id} wasn't found!");
    }
}
