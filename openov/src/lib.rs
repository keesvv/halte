pub mod pass;
pub mod stop;
pub mod tp;

use serde::Deserialize;

pub type Coord = f32;

#[derive(Deserialize, Debug)]
pub enum Accessibility {
    #[serde(rename = "ACCESSIBLE")]
    Accessible,
}

#[derive(Deserialize, Debug)]
pub enum TransportType {
    #[serde(rename = "TRAM")]
    Tram,
}
