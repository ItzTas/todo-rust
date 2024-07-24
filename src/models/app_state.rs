use druid::{Data, Lens};

use super::input_data::InputData;

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub input_data: InputData,
}
