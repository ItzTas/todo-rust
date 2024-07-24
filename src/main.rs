mod models;
mod widgets_internal;
use druid::{AppLauncher, PlatformError, WindowDesc};
use models::{
    app_state::AppState,
    widget_data::{InputData, TaskData},
};
use widgets_internal::widgets::build_main_widget;

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_main_widget()).title("To do :)");
    let data = initial_state();
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn initial_state() -> AppState {
    AppState {
        input_data: InputData {
            input_title: String::new(),
            input_text: String::new(),
        },
        task_data: TaskData {
            task_title: String::new(),
            task_text: String::new(),
        },
    }
}
