use clap::Parser;
use test_rust_chan_chan::{chanchan, normal};

/// try chan chan in Rust
#[derive(Parser)]
struct Cli {
    /// Enable chan chan
    #[clap(short, long)]
    enable_chan_chan: bool,
}

fn main() {
    let args = Cli::parse();
    if !args.enable_chan_chan {
        normal::proc();
    } else {
        chanchan::proc();
    }
}
