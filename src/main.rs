use druid::widget::{Button, Flex, Label, Controller, Either}; // Remove the redundant Button import
use druid::{AppLauncher, Color, FontDescriptor, FontFamily, FontWeight, LocalizedString, Widget, WidgetExt, WindowDesc, PlatformError, Data};

#[derive(Clone, Data)]
struct AppState {
    state_title: String,
    number: i32,
    show_hidden: bool,
    show_new: bool
}

fn new_window_ui() -> impl Widget<AppState> {
    Flex::column()
        .with_child(Label::new("This is a new window"))
        .with_child(Button::new("Close extra window")
            .on_click(|_ctx, state: &mut AppState, _env| state.show_new = false))
        .padding(20.0)
}

fn new_window() -> WindowDesc<AppState> {
    WindowDesc::new(new_window_ui())
        .title("New Window")
        .window_size((400.0, 300.0))
}


fn hidden_ui() -> impl Widget<AppState> {
    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(48.0);

    Label::new(|state: &AppState, _env: &_| format!("My State: {}", state.show_hidden))
        .with_font(font)
        .padding(20.0)
        .border(Color::BLACK, 50.0)
}

fn main_ui() -> impl Widget<AppState> {
    Flex::column()
    .with_child(Button::new("Open New Window")
            .on_click(|_ctx, state: &mut AppState, _env| {
                state.show_new = true;
            })
            .padding(20.0))
        .with_child(Either::new(
            |data: &AppState, _env| data.show_new,
            new_window_ui(),
            hidden_new()
        ))
}

fn hidden_new() -> impl Widget<AppState> {

    let font = FontDescriptor::new(FontFamily::SYSTEM_UI)
        .with_weight(FontWeight::BOLD)
        .with_size(48.0);

    let counter_label = Label::new(|state: &AppState, _env: &_| format!("{}: {}", state.state_title ,state.number)).with_font(font);

    let btn_increment = Button::new("+")
        .on_click(|_ctx, state: &mut AppState, _env| state.number += 1)
        .padding(20.0);

    let btn_decrement = Button::new("-")
        .on_click(|_ctx, state: &mut AppState, _env| state.number -= 1)
        .padding(20.0);

    let hidden_child = hidden_ui(); // Create an instance of the hidden UI

    let conditional_hidden_child = Either::new(
        |data: &AppState, _env| data.show_hidden,
        hidden_child,
        Label::new("Hidden UI is not visible"),
    );


    Flex::column()
        .with_child(counter_label)
        .with_child(
            Flex::row()
                .with_child(btn_increment)
                .with_child(btn_decrement)
        )
        .with_child(Button::new("Show/Hide")
            .on_click(|_ctx, state: &mut AppState, _env| {
            state.show_hidden = !state.show_hidden;
            })
            .padding(20.0))
        .with_child(conditional_hidden_child)
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(main_ui())
        .title("EdgeGUI")
        .window_size(( 500.0, 800.0 ));

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(AppState { state_title: "Counter".to_string(), number: 0, show_hidden: false, show_new: false })
}
