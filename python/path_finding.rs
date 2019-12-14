use std::collections::BTreeMap;
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

pub fn load_day(rides: &HashMap<String,TrainRide>) -> (Graph<Stop, (), Directed>, HashMap<String,BTreeMap<Time,NodeIndex>>) {
    let mut graph = Graph::<Stop, (), Directed>::new();

    let mut per_station = HashMap::<String,BTreeMap<Time,NodeIndex>>::new();
    let mut per_line = HashMap::<String,Vec<NodeIndex>>::new();

    for (line, ride) in rides {
        per_line.insert(line.clone(), Vec::new());
        let per_line = per_line.get_mut(line).unwrap();
        for stop in ride.schedule.iter() {
            if let Some(station) = stop.station.clone() {
                if !per_station.contains_key(&station) {
                    per_station.insert(station.clone(), BTreeMap::new());
                }
                let node = Stop::new(stop, line);
                let departure_time = node.departure_time;
                let node = graph.add_node(node);
                per_line.push(node);
                per_station
                    .get_mut(&station)
                    .unwrap()
                    .insert(departure_time, node);
            }
        }
    }

    for (_, rides) in per_line {
        for (node1, node2) in rides.iter().zip(rides.iter().skip(1)) {
            let departure_time_1 = graph.node_weight(*node1).unwrap().departure_time;
            let arrival_time_2 = graph.node_weight(*node2).unwrap().arrival_time;
            if departure_time_1 < arrival_time_2 {
                graph.add_edge(*node1, *node2, ());
            }
        }
    }

    for (_, station_nodes) in per_station.iter() {
        for (_, node1) in station_nodes.iter() {
            if let Some(Stop{arrival_time,..}) = graph.node_weight(*node1) {
                match arrival_time {
                    Time::Regular{hours, minutes} => {
                        for (_, node2) in station_nodes.range(Time::Regular{hours:*hours,minutes:*minutes}.add(5)..) {
                            let departure_time1 = graph.node_weight(*node1).unwrap().departure_time;
                            let departure_time2 = graph.node_weight(*node2).unwrap().departure_time;
                            if departure_time1 < departure_time2 {
                                graph.add_edge(*node1, *node2, ());
                            }
                        }
                    },
                    Time::Beginning => {
                        for (_, node2) in station_nodes.iter() {
                            let departure_time1 = graph.node_weight(*node1).unwrap().departure_time;
                            let departure_time2 = graph.node_weight(*node2).unwrap().departure_time;
                            if departure_time1 < departure_time2 {
                                graph.add_edge(*node1, *node2, ());
                            }
                        }
                    }
                    Time::End => ()
                }
            }
        }
    }

    (graph, per_station)
}

pub fn load_data(s :&str) -> HashMap<String, (Graph<Stop, (), Directed>, HashMap<String,BTreeMap<Time,NodeIndex>>)> {
    let m: HashMap<String,HashMap<String,TrainRide>> = serde_json::from_str(s).unwrap();
    let mut result = HashMap::new();
    for (day, e) in m.into_iter() {
        result.insert(day, load_day(&e));
    }
    result
}

fn shortest_path(s : &str, t : &str, time: Time, graph: &Graph<Stop, (), Directed>,nodes: &HashMap<String,BTreeMap<Time,NodeIndex>>) {
    let mut pq = PriorityQueue::new();
    let mut shortest_path = HashMap::new();
    let (_,root) = nodes.get(s).unwrap().range(time..).next().unwrap();

    pq.push(*root, Reverse(graph.node_weight(*root).unwrap().arrival_time));
    if let Some(Stop{departure_time, station,arrival_time,..}) = graph.node_weight(*root) {
        shortest_path.insert(String::from(s), (vec![(station,arrival_time,departure_time)],departure_time));
    }

    let result = 'outer: loop {
        if let Some((to_expand, _)) = pq.pop() {
            if let Some(Stop{station,departure_time, arrival_time,..}) = graph.node_weight(to_expand) {
                let station_to_expand = station;
                let departure_time_to_expand = departure_time;
                let arrival_time_to_expand = arrival_time;
                let station_expand = station;
                if station_to_expand == t {
                    print!("arrivl:{:?}: ",arrival_time);
                    for (s,a,d) in shortest_path.get(station_to_expand).unwrap().0.iter() {
                    //    print!(",{} {:?} {:?}",s,a,d)
                    }
                    println!();
                    if false {
                        break 'outer  Some(shortest_path.get(station_to_expand).unwrap());
                    }
                }
                for to_add in graph.neighbors(to_expand) {
                    if let Some(Stop{arrival_time,departure_time, station,..}) = graph.node_weight(to_add) {
                        assert!(departure_time_to_expand < departure_time, "{:?}<{:?}");
                        let mut path = shortest_path.get(station_to_expand).unwrap().0.clone();
                        if let Some((_, time)) = shortest_path.get(station) {
                            if station_to_expand != station && time > &arrival_time {
                                pq.push(to_add, Reverse(*arrival_time));
                                path.push((station,arrival_time,departure_time));
                                shortest_path.insert(station.clone(), (path, arrival_time));
                            } else if station_to_expand == station {
                                println!("{} {:?}",station,arrival_time);
                                pq.push(to_add, Reverse(*arrival_time));
                            }
                        } else {
                            pq.push(to_add.clone(), Reverse(*arrival_time));
                            path.push((station,arrival_time,departure_time));
                            shortest_path.insert(station.clone(), (path, arrival_time));
                        }
                    }
                }
            }
        } else {
            break None;
        }
    };

    if let Some((path, Time::Regular{hours, minutes})) = result {
        for (e,a,d) in path.iter() {
            print!("{} {:?} {:?},",e,a,d);
        }
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
    fn inequality() {
        assert_eq!(Time::Regular{hours:6,minutes:45} > Time::Regular{hours:4,minutes:49}, true);
    }
    #[test]
    fn loading() {
        let data = read_to_string("static/data/timetable.json")
        .expect("error loading file");
        let element = load_data(data.as_ref());
        let (graph, nodes) = element.get("Monday").unwrap();
        println!("loaded");
        shortest_path("dt", "mt", Time::Regular{hours:9,minutes:0}, &graph, &nodes);
    }
}
