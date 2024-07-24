use druid::{Data, Lens};

use super::widget_data::{InputData, TaskData};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub input_data: InputData,
    pub task_data: TaskData,
}
