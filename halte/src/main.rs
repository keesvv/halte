use std::io::{self, Read};

use clap::Parser;
use openov::tp::TimingPointResponse;

#[derive(Parser, Debug)]
struct Args {
    /// Timing point code
    #[arg(short)]
    tpc: u32,
    /// OVAPI endpoint
    #[arg(long, default_value = "https://v0.ovapi.nl")]
    endpoint: String,
}

fn main() {
    // let args = Args::parse();
    // let response = reqwest::blocking::get(format!("{}/tpc/{}", args.endpoint, args.tpc)).unwrap();
    // let dec = response.json::<TimingPointResponse>().unwrap();

    // TODO: remove, this is just for debugging purposes
    let mut data = Vec::new();
    io::stdin().read_to_end(&mut data).unwrap();

    let dec = serde_json::from_slice::<TimingPointResponse>(&data).unwrap();
    println!("{:#?}", dec);
}
