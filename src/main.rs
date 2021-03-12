use iced::window::Settings as WindowSettings;
use iced::{
    slider, Column, Element, HorizontalAlignment, Length, ProgressBar, Sandbox, Settings, Slider,
    Text,
};

pub fn main() -> iced::Result {
    Pomodoro::run(Settings {
        window: WindowSettings {
            size: (400, 400),
            ..WindowSettings::default()
        },
        ..Settings::default()
    })
}

#[derive(Default)]
struct Pomodoro {
    value: f32,
    progress_bar_slider: slider::State,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    SliderChanged(f32),
}

impl Sandbox for Pomodoro {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("A simple Progressbar")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::SliderChanged(x) => self.value = x,
        }
    }

    fn view(&mut self) -> Element<Message> {
        let title = Text::new("Counter")
            .width(Length::Fill)
            .size(100)
            .color([0.5, 0.5, 0.5])
            .horizontal_alignment(HorizontalAlignment::Center);

        Column::new()
            .padding(20)
            .push(ProgressBar::new(0.0..=100.0, self.value))
            .push(title)
            .push(
                Slider::new(
                    &mut self.progress_bar_slider,
                    0.0..=100.0,
                    self.value,
                    Message::SliderChanged,
                )
                .step(0.01),
            )
            .into()
    }
}
