mod models;
use druid::LensExt;
use druid::{
    widget::{Flex, Label, TextBox},
    AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc,
};
use models::{app_state::AppState, input_data::InputData};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_widget()).title("To do :)");
    let data = AppState {
        input_data: InputData {
            input_text: String::new(),
        },
    };
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn build_widget() -> impl Widget<AppState> {
    let textbox = TextBox::new()
        .with_placeholder("Walk the dog")
        .lens(AppState::input_data.then(InputData::input_text));

    let label = Label::dynamic(|data: &String, _env| format!("You typed: {}", &data))
        .padding(5.)
        .lens(AppState::input_data.then(InputData::input_text));

    Flex::column().with_child(label).with_child(textbox)
}
