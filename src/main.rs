use druid::{
    widget::{Button, Flex, Scroll},
    AppLauncher, PlatformError, Widget, WidgetExt, WindowDesc,
};

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(build_widget());
    let data = 0_u32;
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(data)
}

fn build_widget() -> impl Widget<u32> {
    let mut col = Flex::column();
    for i in 0..30 {
        let button = Button::new(format!("Button {}", i)).padding(5.);
        col.add_child(button)
    }
    Scroll::new(col)
}
