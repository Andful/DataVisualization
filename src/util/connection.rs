use std::hash::Hash;
use std::cmp::Eq;
use serde::{Serialize, Deserialize};

#[derive(Hash, Serialize, Clone, Debug)]
pub struct Connection {
    pub line: i64,
    pub departure_station: String,
    pub arrival_station: String,
    pub departure_time: (u8,u8,u8),
    pub arrival_time: (u8,u8,u8),
}
impl Connection {
    pub fn add_days(&self, days: u8) -> Connection {
        Connection {
            line: self.line,
            departure_station: self.departure_station.clone(),
            arrival_station: self.arrival_station.clone(),
            departure_time: (self.departure_time.0 + days,self.departure_time.1,self.departure_time.2),
            arrival_time: (self.arrival_time.0 + days, self.arrival_time.1, self.arrival_time.2),
        }
    }
}

impl Eq for Connection {}

impl std::cmp::PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.departure_time == other.departure_time &&
        self.arrival_time == other.arrival_time &&
        self.departure_station == other.departure_station &&
        self.arrival_station == other.arrival_station
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonConnection {
    pub line: i64,
    pub departure_station: String,
    pub arrival_station: String,
    pub departure_time: String,
    pub arrival_time: String,
}

impl JsonConnection {
    pub fn to_connection(&self, last_arrival_of_same_line: (u8, u8, u8)) -> Option<Connection> {
        let mut split_departue = self.departure_time.split(":");
        let mut split_arrival = self.arrival_time.split(":");

        let departure_hour = split_departue.next().unwrap().parse().unwrap();
        let departure_minute = split_departue.next().unwrap().parse().unwrap();

        let arrival_hour = split_arrival.next().unwrap().parse().unwrap();
        let arrival_minute = split_arrival.next().unwrap().parse().unwrap();

        let mut departure_time = (last_arrival_of_same_line.0, departure_hour, departure_minute);
        if last_arrival_of_same_line.1 > departure_time.1 { // if the hour is different and it is greater probably it is the day after
            if last_arrival_of_same_line.1 - departure_time.1 < 6 {
                return None;
            } else {
                departure_time = (departure_time.0 + 1,departure_time.1, departure_time.2);
            }
        }
        let mut arrival_time = (departure_time.0, arrival_hour, arrival_minute);
        if departure_time.1 > arrival_time.1 {
            if departure_time.1 - arrival_time.1 < 6 {
                return None;
            } else {
                arrival_time = (arrival_time.0 + 1,arrival_time.1, arrival_time.2);
            }
        }
        Some(Connection {
            line: self.line,
            departure_station: self.departure_station.clone(),
            arrival_station: self.arrival_station.clone(),
            departure_time,
            arrival_time,
        })
    }
}
