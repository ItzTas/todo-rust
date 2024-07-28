use druid::{
    widget::{Button, Flex, Label, TextBox},
    LensExt, LocalizedString, Widget, WidgetExt,
};

use crate::models::{app_state::AppState, widget_data::InputData};

pub fn build_main_widget() -> impl Widget<AppState> {
    let title_input = TextBox::new()
        .with_placeholder("Dog")
        .padding(5.)
        .lens(AppState::input_data.then(InputData::input_title));
    let desc_input = TextBox::new()
        .with_placeholder("Walk the dog")
        .padding(5.)
        .lens(AppState::input_data.then(InputData::input_text));

    let text: LocalizedString<AppState> = LocalizedString::new("add task");

    let add_button = Button::new(text.clone()).on_click(|_ctx, data: &mut AppState, _env| {
        data.task_data.task_title = data.input_data.input_title.clone();
        data.task_data.task_text = data.input_data.input_text.clone();
    });

    let task_title_label =
        Label::new(|data: &AppState, _env: &_| format!("Title: {}", data.task_data.task_title))
            .padding(5.);
    let task_desc_label =
        Label::new(|data: &AppState, _env: &_| format!("Desc: {}", data.task_data.task_text))
            .padding(5.);

    Flex::column()
        .with_child(build_label(5., "insert title"))
        .with_child(title_input)
        .with_child(build_label(5., "insert description"))
        .with_child(desc_input)
        .with_child(add_button)
        .with_child(task_title_label)
        .with_child(task_desc_label)
}

pub fn build_label(pad: f64, label_text: &str) -> impl Widget<AppState> {
    Label::new(label_text).padding(pad)
}
