use druid::Data;
use druid::Lens;

#[derive(Clone, Data, Lens)]
pub struct InputData {
    pub input_text: String,
}
