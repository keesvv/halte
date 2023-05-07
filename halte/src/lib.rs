use std::fmt::Debug;

use openov::{
    pass::Pass,
    tp::{TimingPoint, TimingPointResponse},
    TransportType,
};

#[derive(Debug)]
pub struct Timing {
    pub arrival: chrono::NaiveDateTime,
    pub arrival_expected: chrono::NaiveDateTime,
    pub departure: chrono::NaiveDateTime,
}

impl From<&Pass> for Timing {
    fn from(value: &Pass) -> Self {
        Self {
            arrival: value.target_arrival_time,
            arrival_expected: value.expected_arrival_time,
            departure: value.target_departure_time,
        }
    }
}

#[derive(Debug)]
pub struct Line {
    pub destination: String,
    pub transport: TransportType,
    pub name: String,
}

impl From<&Pass> for Line {
    fn from(value: &Pass) -> Self {
        Self {
            destination: value.destination_name.to_owned(),
            transport: value.transport_type,
            name: value.line_public_no.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Departure {
    pub id: String,
    pub line: Line,
    pub timing: Timing,
}

impl From<(&String, &Pass)> for Departure {
    fn from((id, pass): (&String, &Pass)) -> Self {
        Self {
            id: id.to_owned(),
            line: Line::from(pass),
            timing: Timing::from(pass),
        }
    }
}

pub struct Coords(f32, f32);

impl Debug for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Debug)]
pub struct Location {
    pub pos: Coords,
    pub city: String,
}

impl From<&TimingPoint> for Location {
    fn from(value: &TimingPoint) -> Self {
        Self {
            pos: Coords(value.stop.lat, value.stop.long),
            city: value.stop.tp_town.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct Stop {
    pub name: String,
    pub location: Location,
    pub departures: Vec<Departure>,
}

impl From<TimingPointResponse> for Stop {
    fn from(value: TimingPointResponse) -> Self {
        let tp = value.tps.values().next().unwrap();

        Self {
            name: tp.stop.tp_name.to_owned(),
            location: Location::from(tp),
            departures: tp.passes.iter().map(Departure::from).collect(),
        }
    }
}
