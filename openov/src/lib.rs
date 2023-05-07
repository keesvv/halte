pub mod pass;
pub mod stop;
pub mod tp;

use std::fmt::Display;

use serde::Deserialize;

pub type Coord = f32;

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum Accessibility {
    #[serde(rename = "ACCESSIBLE")]
    Accessible,
}

#[derive(Deserialize, Debug, Clone, Copy)]
pub enum TransportType {
    #[serde(rename = "TRAM")]
    Tram,
}

impl Display for TransportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tram => write!(f, "Tram"),
        }
    }
}
