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
    List,
    Serve {
        device_id: String,
        rtsp_path: String,
    },
}
