use std::cmp::{Ord,Eq,PartialEq,PartialOrd,Ordering};

#[derive(Eq,Copy,Debug,Hash)]
pub enum Time {
    Beginning,
    Regular {
        hours: i64,
        minutes: i64
    },
    End,
}

impl Ord for Time {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Time::Beginning => {
                match other {
                    Time::Beginning =>Ordering::Equal,
                    _ => Ordering::Less
                }
            },
            Time::End => {
                match other {
                    Time::End =>Ordering::Equal,
                    _ => Ordering::Greater
                }
            },
            Time::Regular{hours, minutes} => {
                let hours1 = hours + if *hours <= 3 {24} else {0};
                let minutes1 = minutes;
                match other {
                    Time::Beginning =>Ordering::Greater,
                    Time::Regular{hours, minutes} => {
                        let hours2 = hours + if *hours <= 3 {24} else {0};
                        let minutes2 = minutes;
                        match hours1.cmp(&hours2) {
                            Ordering::Equal => minutes1.cmp(minutes2),
                            e => e
                        }
                    },
                    Time::End => Ordering::Less,
                }
            }
        }
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Time {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Clone for Time {
    fn clone(&self) -> Time {
        if let Time::Regular{hours, minutes} = self {
            Time::Regular{hours: *hours,minutes: *minutes}
        } else {
            Time::Beginning
        }
    }
}

impl Time {
    pub fn new(s: &str) -> Time {
        let s : Vec<&str> = s.split(":").collect();
        let hours: i64 = s[0].parse().unwrap();
        let minutes: i64 = s[1].parse().unwrap();
        Time::Regular{hours,minutes}
    }

    pub fn add(&self, i: i64) -> Time {
        if let Time::Regular{hours,minutes} = self {
            let minutes = minutes + i;
            let hours = hours + minutes/60;
            let minutes = minutes % 60;
            let hours = hours % 24;
            Time::Regular{hours,minutes}
        } else {
            self.clone()
        }
    }
}
