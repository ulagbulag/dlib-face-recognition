use clap::Parser;

/// Arguments for the example.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Path to the first image
    #[arg(short, long)]
    pub first_image: String,

    ///Path to the second image
    #[arg(short, long)]
    pub second_image: String,
}