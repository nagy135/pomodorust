use druid::widget::{Align, Flex, Label};
use druid::{AppLauncher, Data, Env, Lens, LocalizedString, Widget, WindowDesc};
use std::thread::sleep;
use std::time::Duration;

const WINDOW_TITLE: LocalizedString<TimerState> = LocalizedString::new("Pomodoro");

#[derive(Clone, Data, Lens)]
struct TimerState {
    timer: i32,
    work_limit: i32,
    rest_limit: i32,
    work: bool,
}

fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget)
        .title(WINDOW_TITLE)
        .window_size((400.0, 400.0));

    // create the initial app state
    let mut initial_state = TimerState {
        timer: 267,
        work_limit: 25 * 60,
        rest_limit: 5 * 60,
        work: true,
    };

    // start the application
    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Failed to launch application");

    sleep(Duration::new(2, 0));
    initial_state.timer = 388;
}

fn build_root_widget() -> impl Widget<TimerState> {
    // a label that will determine its text based on the current app data.
    let label = Label::new(|data: &TimerState, _env: &Env| {
        format!("Hello {}!", number_to_time(data.timer))
    });

    // arrange the two widgets vertically, with some padding
    let layout = Flex::column().with_child(label);

    // center the two widgets in the available space
    Align::centered(layout)
}

fn number_to_time(number: i32) -> String {
    let minutes = number / 60;
    let seconds = number % 60;
    format!("{}:{}", minutes, seconds)
}
