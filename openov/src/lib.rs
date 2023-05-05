pub mod pass;
pub mod stop;
pub mod tp;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub enum Accessibility {
    #[serde(rename = "ACCESSIBLE")]
    Accessible,
}
