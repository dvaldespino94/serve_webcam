use clap::Parser;

use crate::arguments::Cli;

mod arguments;
mod devices;

fn main() {
    let cli = Cli::parse();
    match cli.command {
        arguments::Commands::List => devices::list_devices_subcommand(),
        arguments::Commands::Serve {
            device_id,
            rtsp_path,
            rtsp_port,
        } => devices::serve_webcam_subcommand(device_id, rtsp_port.unwrap_or(554), rtsp_path),
    }
}
