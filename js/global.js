//fariables that need to be shared between the modules

//a hashmap that converts the station code to its name (e.g. dt -> Delft)
let code_to_station = {};
export function set_code_to_station(e) {
    code_to_station = e;
}

export function get_code_to_station(e) {
    return code_to_station;
}

//a hashmap that converts link between two stations to a list of id of the paths to be highlited
let path_to_link = {}
export function set_path_to_link(e) {
    path_to_link = e;
}

export function get_path_to_link(e) {
    return path_to_link;
}
