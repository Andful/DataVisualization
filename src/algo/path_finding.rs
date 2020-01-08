use std::collections::HashSet;
use std::collections::HashMap;
use std::collections::BTreeSet;
use std::collections::VecDeque;

use crate::util::train_ride::TrainRide;
use crate::util::stop::Stop;
use crate::util::time::Time;
use crate::util::connection::Connection;
use crate::util::connection::JsonConnection;

use serde_json::Value;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::Worker;
use web_sys::console;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
struct PathFinder {
    worker: Option<Worker>,
    stations:  HashSet<String>,
    connections: Vec<Connection>,
    connection_to_path: HashMap<BTreeSet<String>, HashSet<String>>,
    removed_stations: HashSet<String>,
    removed_link: HashSet<String>,
    removed_train: HashSet<i64>,
    function_call_n: Option<i64>,
}

#[derive(Deserialize)]
struct FunctionCall {
    n: i64,
    f: String,
    args: HashMap<String, String>
}

#[wasm_bindgen]
impl PathFinder {
    pub fn onmessage(&mut self, data : &str) {
        let data : FunctionCall =  match serde_json::from_str(data) {
            Ok(e) => e,
            Err(e) => {
                console::error_1(&JsValue::from(&e.to_string()));
                console::error_1(&JsValue::from(data));
                FunctionCall{n:-1,f:String::new(),args:HashMap::new()}
            }
        };

        self.function_call_n = Some(data.n);

        match data.f.as_ref() {
            "generate_links" => self.generate_links(&data.args),
            "remove_link" => self.remove_link(&data.args),
            "remove_train" => self.remove_train(&data.args),
            "remove_station" => self.remove_station(&data.args),
            "add_link" => self.add_link(&data.args),
            "add_train" => self.add_train(&data.args),
            "add_station" => self.add_station(&data.args),
            "reset" => self.reset(&data.args),
            "compute_paths" => self.compute_paths(&data.args),
            _ => console::error_1(&JsValue::from(format!("function {} not supported",data.f)))
        }

        self.end(self.function_call_n.unwrap());
        self.function_call_n = None;
    }

    pub fn load_data(worker : JsValue, timetable :&str) -> PathFinder {
        let mut result = PathFinder::new(timetable);
        result.worker = Some(worker.unchecked_into::<Worker>());
        result
    }
}


impl PathFinder {
    fn get_links<'a>(&'a self, conn: &Connection) -> Option<&'a HashSet<String>> {
        let mut key = BTreeSet::new();
        key.insert(conn.arrival_station.clone());
        key.insert(conn.departure_station.clone());
        self.connection_to_path.get(&key)
    }

    fn post_message<T: Serialize>(&self, e : &T) {
        let data = match serde_json::to_string(e) {
            Ok(s) => s,
            Err(e) => {
                console::error_1(&JsValue::from(&e.to_string()));
                console::error_1(&JsValue::from(line!()));
                String::new()
            }
        };
        if let Some(worker) = &self.worker {
            match worker.post_message(&JsValue::from(format!("{{\"n\":{},\"data\":{}}}",self.function_call_n.unwrap(),data))) {
                Ok(()) => (),
                Err(e) => console::error_1(&e)
            }
        }
    }

    fn end(&mut self,n: i64) {
        if let Some(worker) = &self.worker {
            if let Err(e) = worker.post_message(&JsValue::from(format!("{{\"n\":{},\"end\":true}}",n))) {
                console::error_1(&e);
                console::error_1(&JsValue::from(line!()));
            }
        }
        self.function_call_n = None;
    }

    fn new(timetable :&str) -> PathFinder {
        let mut week = HashMap::<String,u8>::new();
        week.insert("Monday".to_string(),0);
        week.insert("Tuesday".to_string(),1);
        week.insert("Wednesday".to_string(),2);
        week.insert("Thursday".to_string(),3);
        week.insert("Friday".to_string(),4);
        week.insert("Saturday".to_string(),5);
        week.insert("Sunday".to_string(),6);
        let timetable: HashMap<String, Vec<JsonConnection>> = match serde_json::from_str(timetable) {
            Ok(e) => e,
            Err(e) => {
                console::error_1(&JsValue::from(&e.to_string()));
                HashMap::new()
            },
        };

        let mut connections = Vec::<Connection>::new();
        for (day, e) in timetable.into_iter() {
            let mut new_connections = PathFinder::parse_json_connections(*week.get(&day).unwrap(), &e);
            connections.append(&mut new_connections);
        }
        connections.sort_by_key(|x| x.arrival_time);

        let mut stations = HashSet::<String>::new();
        for e in connections.iter() {
            stations.insert(e.arrival_station.clone());
            stations.insert(e.departure_station.clone());
        }

        PathFinder{
            worker: None,
            stations,
            connections,
            connection_to_path:HashMap::new(),
            removed_stations: HashSet::new(),
            removed_link: HashSet::new(),
            removed_train: HashSet::new(),
            function_call_n: None,
        }
    }

    fn generate_links(&mut self, args : &HashMap<String, String>) {
        let mut result = HashMap::<String,&Vec<String>>::new();
        let pairs = args.get("pairs").unwrap();

        let pairs: Vec<Vec<String>> = match serde_json::from_str(pairs) {
            Ok(e) => e,
            Err(e) => {
                console::error_1(&JsValue::from(&e.to_string()));
                Vec::new()
            },
        };
        let mut adjacency_list : HashMap<&str, Vec<&str>> = HashMap::new();
        for pair in pairs.iter() {
            if !adjacency_list.contains_key(&pair[0].as_ref()) {
                adjacency_list.insert(pair[0].as_ref(),Vec::new());
            }
            adjacency_list.get_mut(&pair[0].as_ref()).unwrap().push(pair[1].as_ref());

            if !adjacency_list.contains_key(&pair[1].as_ref()) {
                adjacency_list.insert(pair[1].as_ref(),Vec::new());
            }
            adjacency_list.get_mut(&pair[1].as_ref()).unwrap().push(pair[0].as_ref());
        }
        let mut links = HashSet::<BTreeSet<String>>::new();
        for e in self.connections.iter() {
            let link : BTreeSet<String> = vec![e.arrival_station.clone(), e.departure_station.clone()].into_iter().collect();
            links.insert(link);
        }

        let mut connection_to_path = HashMap::<BTreeSet<String>, HashSet<String>>::new();
        let mut result = HashMap::<String, Vec<String>>::new();

        for link in links.iter() {
            let link : Vec<&str> = link.iter().map(|x| x.as_ref()).collect();
            let to = link[0];
            let from = link[1];
            let mut came_from = HashMap::<&str,&str>::new();
            came_from.insert(from,"");
            let mut to_process = VecDeque::new();
            to_process.push_front(from);
            while !to_process.is_empty() {
                let next_to_process = to_process.pop_back().unwrap();
                for e in adjacency_list.get(next_to_process).unwrap() {
                    if ! came_from.contains_key(e) {
                        to_process.push_front(e);
                        came_from.insert(e, next_to_process);
                        if e == &to {
                            break;
                        }

                    }
                }
            }

            let path : Vec<String> = {
                let mut path = Vec::new();
                let mut current = to;
                path.push(String::from(current));
                while current != from {
                    current = came_from.get(current).unwrap();
                    path.push(String::from(current));
                }
                path[0..path.len()-1]
                .iter()
                .zip(path[1..path.len()].iter())
                .map(|(x,y)| if x < y {x.clone() + "-" + y} else {y.clone()+ "-" + x})
                .collect()
            };
            connection_to_path.insert(link.iter().map(|x| x.to_string()).collect(), path.clone().into_iter().collect());
            result.insert(link.iter().map(|x| x.to_string()).collect::<Vec<String>>().join("-"), path);
        }

        self.connection_to_path = connection_to_path;

        self.post_message(&result);

    }

    fn remove_station(&mut self, args : &HashMap<String, String>) {
        self.removed_stations.insert(args.get("station").unwrap().clone());
        self.post_message(&self.removed_stations);
    }

    fn remove_train(&mut self, args : &HashMap<String, String>) {
        self.removed_train.insert(args.get("train").unwrap().parse().unwrap());
        self.post_message(&self.removed_train);
    }

    fn remove_link(&mut self, args : &HashMap<String, String>) {
        self.removed_link.insert(args.get("link").unwrap().clone());
        self.post_message(&self.removed_link);
        console::log_1(&format!("new removed link {:?}",self.removed_link).into());
    }

    fn add_station(&mut self, args : &HashMap<String, String>) {
        self.removed_stations.remove(args.get("station").unwrap());
        self.post_message(&self.removed_stations);
    }
    fn add_train(&mut self, args : &HashMap<String, String>) {
        self.removed_train.remove(&args.get("train").unwrap().parse().unwrap());
        self.post_message(&self.removed_train);
    }
    fn add_link(&mut self, args : &HashMap<String, String>) {
        self.removed_link.remove(args.get("link").unwrap());
        self.post_message(&self.removed_link);
    }

    fn reset(&mut self, _args : &HashMap<String, String>) {
        self.removed_stations = HashSet::new();
        self.removed_train = HashSet::new();
        self.removed_link = HashSet::new();
    }

    fn compute_paths(&mut self, args : &HashMap<String, String>) {
        console::log_1(&format!("{:?}",self.removed_link).into());
        let day : u8 = args.get("day").unwrap().parse().unwrap();
        let modified : bool = args.get("modified").unwrap().parse().unwrap();
        let mut start_time = (0, 5, 0);
        let from = args.get("from").unwrap().as_ref();
        let to = args.get("to").unwrap().as_ref();
        loop {
            let path = if modified {
                    self.shortest_path(
                        from,
                        to,
                        start_time,
                        &self.removed_link,
                        &self.removed_stations,
                        &self.removed_train
                    )
            } else {
                self.shortest_path(
                    from,
                    to,
                    start_time,
                    &HashSet::new(),
                    &HashSet::new(),
                    &HashSet::new()
                )
            };
            if path.is_empty() || path[path.len() - 1].arrival_time > (1,5,0) {
                break;
            } else {
                self.post_message(&path);
            }
            start_time = path[0].departure_time;
            start_time.2 += 1;
            start_time.1 += start_time.2/60;
            start_time.2 = start_time.2 % 60;
            start_time.0 += start_time.1/24;
            start_time.1 = start_time.1 % 60;
            if start_time.0 > day {
                break
            }
        }
    }

    fn parse_json_connections(day :u8, json_connections : &Vec<JsonConnection>) -> Vec<Connection> {
        let mut result = Vec::<Connection>::new();

        let mut last_line = -1;
        let mut last_arrival = (0, 0, 0);

        for json_connection in json_connections.iter() {
            if last_line != json_connection.line {
                last_line = json_connection.line;
                last_arrival = (0,0,0);
            }

            if let Some(new_connection) = json_connection.to_connection(last_arrival) {
                result.push(new_connection.add_days(day));
            }
        }
        result
    }

    fn shortest_path(&self, departure_station : &str, arrival_station : &str, time: (u8,u8,u8), removed_link: &HashSet<String>, removed_stations: &HashSet<String>, removed_train: &HashSet<i64>) -> Vec<Connection> {
        assert!(self.stations.contains(&departure_station.to_string()));
        assert!(self.stations.contains(&arrival_station.to_string()));
        let mut earliest_arrival = HashMap::<String, (u8,u8,u8)>::new();
        let mut earliest = (9,25,61);
        let mut in_connections = HashMap::<&str,&Connection>::new();
        for station in self.stations.iter() {
            earliest_arrival.insert(station.clone(),(9,25,61));
        }
        earliest_arrival.insert(String::from(departure_station), time);
        println!("length {}",self.connections.len());
        for c in self.connections.iter() {
            if c.departure_time >= *earliest_arrival.get(&c.departure_station).unwrap() &&
                c.arrival_time <= *earliest_arrival.get(&c.arrival_station).unwrap() &&
                !removed_train.contains(&c.line) && //check if train is removed
                if let Some(e) = self.get_links(&c){removed_link.intersection(e).count() == 0} else {true} && //check if no rail has been removed
                ((in_connections.contains_key(&c.departure_station.as_ref()) &&
                    in_connections.get(&c.departure_station.as_ref()).unwrap().line == c.line) || //check that no exchange has been made
                !removed_stations.contains(&c.departure_station)) { //check that the exchange station is not used
                in_connections.insert(c.arrival_station.as_ref(),c);
                earliest_arrival.insert(c.arrival_station.clone(), c.arrival_time);
                if c.arrival_station == arrival_station {
                    if earliest > c.arrival_time {
                        earliest = c.arrival_time;
                    };
                }
            } else if c.arrival_time > earliest {
                break;
            }
        }
        let mut path = Vec::<Connection>::new();
        let mut last_connection = in_connections.get(arrival_station);
        println!("is some:{}",last_connection.is_some());

        while last_connection.is_some() {
            if let Some(connection) = last_connection {
                path.push((*connection).clone());
                last_connection = in_connections.get(&connection.departure_station.as_ref());
            }
        }
        path.reverse();
        path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn loading() {
        let timetable = read_to_string("static/data/timetable.json.real")
        .expect("error loading file");
        let pairs = read_to_string("static/data/pairs.json")
        .expect("error loading file");
        let path_finder = PathFinder::new(timetable.as_ref());
        let mut count = 0;
        println!("{}",count);
        println!("{}", serde_json::to_string(&path_finder.shortest_path("mt", "dt", (0,0,0),&HashSet::new(),&HashSet::new(),&HashSet::new())).unwrap());
        println!("{:?}",path_finder.connection_to_path.get(&vec![String::from("gv"),String::from("dt")].into_iter().collect()));
    }
}
