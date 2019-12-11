use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;
use std::collections::BTreeMap;
use std::option::Option;
use serde_json;
use serde::{Serialize, Deserialize};
use std::cmp::{Ord,Eq,PartialEq,PartialOrd,Ordering,Reverse};
use std::collections::BinaryHeap;


pub struct Node {
    outgoing: Vec<Rc<RefCell<Node>>>,
    arrival_time: Time,
    departure_time: Time,
    station: String,
    line: String
}

impl Node {
    fn new(ts : &TrainStop, line :&str) -> Node {
        let outgoing = Vec::new();
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
        Node{outgoing, arrival_time, departure_time, station,line}
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.departure_time.cmp(&other.departure_time)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.departure_time.cmp(&other.departure_time))
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.departure_time == other.departure_time
    }
}

#[derive(Eq,Copy,Debug)]
enum Time {
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
                let hours1 = hours + if *hours <= 4 {24} else {0};
                let minutes1 = minutes;
                match other {
                    Time::Beginning =>Ordering::Greater,
                    Time::Regular{hours, minutes} => {
                        let hours2 = hours + if *hours <= 4 {24} else {0};
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
    fn new(s: &str) -> Time {
        let s : Vec<&str> = s.split(":").collect();
        let hours: i64 = s[0].parse().unwrap();
        let minutes: i64 = s[1].parse().unwrap();
        Time::Regular{hours,minutes}
    }

    fn add(&self, i: i64) -> Time {
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

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainStop {
    arrival_time: Option<String>,
    departure_time: Option<String>,
    station: Option<String>,
    platform: Option<String>,
    on_time: Option<String>,
    cancelled: Option<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainRide {
    img: Option<String>,
    schedule: Vec<TrainStop>
}

fn load_day(rides : &BTreeMap<String,TrainRide>) -> BTreeMap::<String,BTreeMap<Time,Rc<RefCell<Node>>>> {
    let mut per_line = BTreeMap::<String,Vec<Rc<RefCell<Node>>>>::new();
    let mut per_station = BTreeMap::<String,BTreeMap<Time,Rc<RefCell<Node>>>>::new();

    for (line, ride) in rides {
        per_line.insert(line.clone(), Vec::new());
        for stop in ride.schedule.iter() {
            if let Some(station) = stop.station.clone() {
                if !per_station.contains_key(&station) {
                    per_station.insert(station.clone(), BTreeMap::new());
                }
                let node = Rc::new(RefCell::new(Node::new(stop, line)));
                per_line.get_mut(line).unwrap().push(node.clone());
                let departure_time = node.borrow().departure_time;
                per_station
                    .get_mut(&station)
                    .unwrap()
                    .insert(departure_time, node);
            }
        }
    }

    for (line, rides) in per_line {
        for (node1, node2) in rides.iter().zip(rides.iter().skip(1)) {
            node1
            .borrow_mut()
            .outgoing
            .push(node2.clone());
        }
    }

    for (station, station_nodes) in per_station.iter() {
        for ((_, node1), (_, node2)) in station_nodes.iter().zip(station_nodes.iter().skip(1)) {
            node1
            .borrow_mut()
            .outgoing
            .push(node2.clone());
        }
    }
    per_station
}

fn load_data(s : &str) -> BTreeMap<String,BTreeMap::<String,BTreeMap<Time,Rc<RefCell<Node>>>>> {
    let m: BTreeMap<String,BTreeMap<String,TrainRide>> = serde_json::from_str(s).unwrap();
    let mut result = BTreeMap::<String,BTreeMap::<String,BTreeMap<Time,Rc<RefCell<Node>>>>>::new();

    for (day, value) in m.into_iter() {
        result.insert(day,load_day(&value));
    }

    result
}

fn shortest_path(s : &str, t : &str, time: Time, graph : &BTreeMap<String,BTreeMap<Time,Rc<RefCell<Node>>>>) {
    let mut pq = BinaryHeap::new();
    let (_,root) = graph.get(s).unwrap().range(time..).next().unwrap();
    //println!("root at {} time {:?}", root.borrow().station, root.borrow().departure_time);
    pq.push(Reverse(root.clone()));

    let arrival_time = 'outer: loop {
        if pq.len() == 0 {
            break Time::End;
        }
        if let Some(Reverse(to_expand)) = pq.pop() {
            if to_expand.borrow().station == t {
                break to_expand.borrow().arrival_time
            }

            for to_add in to_expand.borrow().outgoing.iter() {
                pq.push(Reverse(to_add.clone()));
                if to_expand.borrow().station=="mt" ||
                    to_expand.borrow().station=="std" ||
                    to_expand.borrow().station=="rm" ||
                    to_expand.borrow().station=="wt" ||
                    to_expand.borrow().station=="ehv" ||
                    to_expand.borrow().station=="tb" {
                    //println!("expanding from {} to {} at time {:?}", to_expand.borrow().station, to_add.borrow().station, to_add.borrow().departure_time);
                }
                //println!("time next {:?}",pq.peek().unwrap().0.borrow().departure_time);
            }
        }
    };

    if let Time::Regular{hours, minutes} = arrival_time {
        println!("Arriving at {}:{}", hours, minutes);
    } else {
        println!("Could not arrive");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn time_comparison() {
        let a = Time::Regular{hours:10,minutes:8};
        let b = Time::Regular{hours:10,minutes:28};
        assert_eq!(a < b, true);
        assert_eq!(
            Rc::new(RefCell::new(a))<
            Rc::new(RefCell::new(b))
            , true);

        let mut pq =  BinaryHeap::<Rc<RefCell<Time>>>::new();
        pq.push(Rc::new(RefCell::new(a)));
        pq.push(Rc::new(RefCell::new(b)));
        println!("abcd {:?}", pq.pop().unwrap().borrow());
    }

    #[test]
    fn loading() {
        let data = read_to_string("static/data/timetable.json")
        .expect("error loading file");
        let graph = load_data(data.as_ref());
        println!("loaded");
        let from = Time::Regular{hours:9, minutes:0};
        let (_,e) =  graph.get("Thursday").unwrap().get("mt").unwrap().range(from..).next().unwrap();
        println!("1 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("2 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("3 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("4 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("5 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(1).unwrap().clone();
        println!("6 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(1).unwrap().clone();
        println!("7 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("8 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("9 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("10 {:?} {}",e.borrow().departure_time,e.borrow().station);
        let e = e.borrow().outgoing.get(0).unwrap().clone();
        println!("11 {:?} {}",e.borrow().departure_time,e.borrow().station);

        //println!("{:?}",t);
        //for e in e.borrow().outgoing.iter() {
        //    println!("{} {:?}",e.borrow().station, &e.borrow().departure_time)
        //}
        shortest_path("mt", "dt", Time::Regular{hours:9,minutes:0}, &graph.get("Monday").unwrap());
        //println!("Hello World");
    }
}
