use clap::Parser;

use arguments::Cli;

mod arguments;
mod devices;

fn main() {
    //Parse the arguments
    let cli = Cli::parse();

    //Do what the user commanded
    match cli.command {
        //List the available devices
        arguments::Commands::List => devices::list_devices_subcommand(),

        //Or start serving the user selected device
        arguments::Commands::Serve {
            device_id,
            rtsp_path,
            rtsp_port,
        } => devices::serve_webcam_subcommand(device_id, rtsp_port.unwrap_or(554), rtsp_path),
    }
}
