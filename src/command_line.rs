use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Args {

    #[arg(short, long)]
    pub key: String,

    #[arg(short, long)]
    pub message: String,
}
