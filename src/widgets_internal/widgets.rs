use druid::{
    widget::{Flex, Label, TextBox},
    LensExt, Widget, WidgetExt,
};

use crate::models::{app_state::AppState, widget_data::InputData};

pub fn build_main_widget() -> impl Widget<AppState> {
    let title_input = TextBox::new()
        .with_placeholder("Dog")
        .padding(5.)
        .lens(AppState::input_data.then(InputData::input_title));
    let desc_input = TextBox::new()
        .with_placeholder("Dog")
        .padding(5.)
        .lens(AppState::input_data.then(InputData::input_text));

    Flex::column()
        .with_child(build_label(5., "insert title"))
        .with_child(title_input)
        .with_child(build_label(5., "insert description"))
        .with_child(desc_input)
}

pub fn build_label(pad: f64, label_text: &str) -> impl Widget<AppState> {
    Label::new(label_text).padding(pad)
}
