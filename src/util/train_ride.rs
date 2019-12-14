use crate::util::train_stop::TrainStop;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TrainRide {
    pub img: Option<String>,
    pub schedule: Vec<TrainStop>
}
