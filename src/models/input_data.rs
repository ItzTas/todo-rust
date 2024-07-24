use druid::{Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct Title {
    input_text: String,
}
