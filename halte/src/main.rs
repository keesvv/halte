use std::io::{self, Read};

use halte::Stop;
use openov::tp::TimingPointResponse;

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone, Copy)]
enum Source {
    HTTP,
    #[clap(alias = "-")]
    Stdin,
}

#[derive(Parser, Debug)]
struct Args {
    /// Data source
    #[arg(short, default_value = "http")]
    source: Source,
    /// Timing point code
    #[arg(short)]
    tpc: u32,
    /// OVAPI endpoint
    #[arg(long, default_value = "https://v0.ovapi.nl")]
    endpoint: String,
}

impl TryFrom<Args> for TimingPointResponse {
    // TODO: handle errors
    type Error = ();

    fn try_from(args: Args) -> Result<Self, Self::Error> {
        Ok(match args.source {
            Source::HTTP => {
                let response =
                    reqwest::blocking::get(format!("{}/tpc/{}", args.endpoint, args.tpc)).unwrap();
                response.json().unwrap()
            }
            Source::Stdin => {
                let mut data = Vec::new();
                io::stdin().read_to_end(&mut data).unwrap();
                serde_json::from_slice(&data).unwrap()
            }
        })
    }
}

fn main() {
    let args = Args::parse();
    let response = TimingPointResponse::try_from(args).unwrap();

    println!("{:#?}", Stop::from(response));
}
