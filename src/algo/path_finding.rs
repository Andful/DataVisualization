use std::collections::BTreeMap;
use std::collections::HashSet;
use std::collections::HashMap;
use std::cmp::Reverse;
use std::option::Option;

use petgraph::graph::Graph;
use petgraph::graph::NodeIndex;
use petgraph::graph::EdgeIndex;
use petgraph::Direction::Outgoing;
use petgraph::Direction::Incoming;
use petgraph::Directed;
use priority_queue::PriorityQueue;

use crate::util::train_ride::TrainRide;
use crate::util::stop::Stop;
use crate::util::time::Time;
use crate::util::connection::Connection;

pub fn load_day(rides: &HashMap<String,TrainRide>) -> (Vec<String>, Vec<Connection>) {
    let mut station_codes = Vec::new();
    let mut connections = Vec::new();
    let mut already_considered = HashSet::new();
    let mut per_line = HashMap::<String,Vec<Stop>>::new();

    for (line, ride) in rides {
        per_line.insert(line.clone(), Vec::new());
        let per_line = per_line.get_mut(line).unwrap();
        for stop in ride.schedule.iter() {
            if let Some(station) = stop.station.clone() {
                let stop = Stop::new(stop, line);
                if !already_considered.contains(&stop.station) {
                    already_considered.insert(station.clone());
                    station_codes.push(station.clone());
                }
                per_line.push(stop);
            }
        }
    }

    for (line, rides) in per_line {
        for (node1, node2) in rides.iter().zip(rides.iter().skip(1)) {
            if node1.departure_time < node2.arrival_time {
                connections.push(Connection{
                    line: line.clone(),
                    arrival_station: node1.station.clone(),
                    departure_station: node2.station.clone(),
                    departure_time: node1.departure_time,
                    arrival_time: node2.arrival_time
                });
            }
        }
    }

    connections.sort_by_key(|x| x.arrival_time);

    (station_codes, connections)
}

pub fn load_data(s :&str) -> HashMap<String, (Vec<String>, Vec<Connection>)> {
    let m: HashMap<String,HashMap<String,TrainRide>> = serde_json::from_str(s).unwrap();
    let mut result = HashMap::new();
    for (day, e) in m.into_iter() {
        result.insert(day, load_day(&e));
    }
    result
}


fn dfs<'a>(root: &str,expand :&HashMap::<String, Vec<&'a Connection>>, depths :&mut HashMap<&'a Connection,i64>, depth: i64) {
    for c in expand.get(root).unwrap().iter() {
        if *depths.get(c).unwrap() > depth {
            depths.insert(c, depth);
            dfs(&c.arrival_station,expand,depths,depth+1);
        }
    }
}

fn shortest_path(departure_station : &str, arrival_station : &str, time: Time, stations:  &Vec<String>,connections: &Vec<Connection>) {
    let mut earliest_arrival = HashMap::<String, Time>::new();
    let mut earliest = Time::End;
    let mut in_connections = HashMap::new();

    for station in stations.iter() {
        earliest_arrival.insert(station.clone(),Time::End);
    }

    earliest_arrival.insert(String::from(departure_station), time);
    println!("length {}",connections.len());

    for c in connections.iter() {
        if c.departure_time >= *earliest_arrival.get(&c.departure_station).unwrap() &&
            c.arrival_time < *earliest_arrival.get(&c.arrival_station).unwrap() {

            in_connections.insert(c.arrival_station.clone(),c);
            if c.arrival_station == arrival_station {
                if earliest > c.arrival_time {
                    earliest = c.arrival_time;
                };
            }
        } else if c.arrival_time > earliest {
            break;
        }
    }

    let mut path = Vec::new();
    let mut last_connection = in_connections.get(arrival_station);
    println!("is some:{}",last_connection.is_some());
    while last_connection.is_some() {
        if let Some(connection) = in_connections.get(arrival_station) {
            path.push(connection);
            last_connection = in_connections.get(&connection.departure_station);
        }
    }

    for connection in path.iter() {
        print!("{} - ",connection.departure_station);
    }
    if path.len() > 0 {
        println!("{}",path.last().unwrap().arrival_station);
        println!("arrived at {:?}", path.last().unwrap().arrival_time);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;
    #[test]
    fn inequality() {
        assert_eq!(Time::Regular{hours:6,minutes:45} > Time::Regular{hours:4,minutes:49}, true);
    }

    #[test]
    fn loading() {
        let data = read_to_string("static/data/timetable.json")
        .expect("error loading file");
        let element = load_data(data.as_ref());
        let (stations, connections) = element.get("Monday").unwrap();
        println!("loaded");
        shortest_path("mt", "dt", Time::Regular{hours:9,minutes:0}, stations, &connections);
    }
}
