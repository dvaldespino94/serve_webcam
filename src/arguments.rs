use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "Webcam Server")]
#[command(author = "David Valdespino Pavón")]
#[command(version = "0.1.0")]
#[command(about = "Simple \"id-fixed\" Webcam server using ffmpeg")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    //List the devices
    List,
    //Serve the selected device(using ffmpeg for the moment)
    Serve {
        //Device id wich will be used to match the selected device
        device_id: String,
        //RTSP path on the serve to be used (This path doesn't include the server address or port)
        rtsp_path: String,
        //RTSP port on the server
        rtsp_port: Option<u16>,
    },
}
