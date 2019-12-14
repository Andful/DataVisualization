use std::rc::Rc;
use std::cell::RefCell;
use std::option::Option;

use crate::util::time::Time;
use crate::util::train_stop::TrainStop;
use std::cmp::{Ord,Eq,PartialEq,PartialOrd,Ordering};

pub struct Stop {
    pub arrival_time: Time,
    pub departure_time: Time,
    pub station: String,
    pub line: String
}

impl Stop {
    pub fn new(ts : &TrainStop, line :&str) -> Stop {
        let line = String::from(line);
        let departure_time = match &ts.departure_time {
            Some(s) => Time::new(&s.as_ref()),
            _ => Time::End
        };
        let arrival_time = match &ts.arrival_time {
            Some(s) => Time::new(&s.as_ref()),
            _ => Time::Beginning
        };
        let station = ts.station.clone().unwrap();
        Stop{arrival_time, departure_time, station,line}
    }
}
impl Ord for Stop {
    fn cmp(&self, other: &Self) -> Ordering {
        self.departure_time.cmp(&other.departure_time)
    }
}

impl PartialOrd for Stop {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.departure_time.cmp(&other.departure_time))
    }
}

impl Eq for Stop {}

impl PartialEq for Stop {
    fn eq(&self, other: &Self) -> bool {
        self.departure_time == other.departure_time
    }
}
