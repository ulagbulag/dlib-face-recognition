use clap::Parser;

/// Arguments for the example.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to input image
    #[arg(short, long)]
    pub input_image: String,

    ///Path to output image
    #[arg(short, long, default_value="../outputs/output.png")]
    pub output_image: String,
}