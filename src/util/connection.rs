use crate::util::time::Time;
use std::hash::Hash;
use std::cmp::Eq;
use std::marker::Copy;
use std::hash::{Hasher};

#[derive(Hash)]
pub struct Connection {
    pub line: String,
    pub departure_station: String,
    pub arrival_station: String,
    pub departure_time: Time,
    pub arrival_time: Time,
}
impl Eq for Connection {}

/*impl Hash for Connection {
    fn hash<H: Hasher>(&self, state: &mut H) {
        if let Time::Regular{hours, minutes} = {

        }
        self.departure_time.minutes.hash(state);
        self.phone.hash(state);
    }
}*/

impl std::cmp::PartialEq for Connection {
    fn eq(&self, other: &Self) -> bool {
        self.departure_time == other.departure_time &&
        self.arrival_time == other.arrival_time &&
        self.departure_station == other.departure_station &&
        self.arrival_station == other.arrival_station
    }
}
