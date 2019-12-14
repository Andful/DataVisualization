use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainStop {
    pub arrival_time: Option<String>,
    pub departure_time: Option<String>,
    pub station: Option<String>,
    pub platform: Option<String>,
    pub on_time: Option<String>,
    pub cancelled: Option<String>
}
