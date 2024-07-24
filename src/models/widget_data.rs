use druid::Data;
use druid::Lens;

#[derive(Clone, Data, Lens)]
pub struct InputData {
    pub input_title: String,
    pub input_text: String,
}

#[derive(Clone, Data, Lens)]
pub struct TaskData {
    pub task_title: String,
    pub task_text: String,
}
