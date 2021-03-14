use chrono::Timelike;
use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    executor, time,
    widget::Text,
    window::Settings as WindowSettings,
    Align, Application, Color, Column, Command, Container, Element, Length, Point, Rectangle,
    Settings, Subscription, Vector,
};

pub fn main() -> iced::Result {
    Clock::run(Settings {
        window: WindowSettings {
            size: (400, 200),
            ..WindowSettings::default()
        },
        antialiasing: true,
        ..Settings::default()
    })
}

struct Clock {
    count: i32,
    total_work: i32,
    total_rest: i32,
    work: bool,
    now: chrono::DateTime<chrono::Local>,
    previous: u32,
    clock: Cache,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                count: 0,
                total_work: 15 * 60,
                total_rest: 5 * 60,
                work: true,
                now: chrono::Local::now(),
                previous: chrono::Local::now().minute(),
                clock: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Pomodoro")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::Tick(local_time) => {
                let now = local_time;

                if now != self.now {
                    self.now = now;
                    self.clock.clear();
                }
                let second = self.now.second();
                if self.previous != second {
                    self.previous = second;
                    self.count += 1;
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(500))
            .map(|_| Message::Tick(chrono::Local::now()))
    }

    fn view(&mut self) -> Element<Message> {
        let minutes: i32 = self.count / 60;
        let seconds: i32 = self.count - 60 * minutes;

        let current = format!("{:0>#2}:{:0>#2}", minutes, seconds);
        let total = match self.work {
            true => "15:00",
            false => "5:00",
        };
        let timer = Text::new(format!("{}/{}", current, total)).size(50);

        let canvas = Container::new(
            Canvas::new(self)
                .width(Length::Units(100))
                .height(Length::Units(100)),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(5)
        .align_x(Align::End)
        .center_y();

        Column::new().padding(20).push(canvas).push(timer).into()
    }
}

impl canvas::Program<Message> for Clock {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);
            frame.fill(&background, Color::from_rgb8(0xc2, 0x23, 0x30));

            let short_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.5 * radius));

            let long_hand = Path::line(Point::ORIGIN, Point::new(0.0, -0.8 * radius));

            let thin_stroke = Stroke {
                width: radius / 100.0,
                color: Color::WHITE,
                line_cap: LineCap::Round,
                ..Stroke::default()
            };

            let wide_stroke = Stroke {
                width: thin_stroke.width * 3.0,
                ..thin_stroke
            };

            frame.translate(Vector::new(center.x, center.y));

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.hour(), 12));
                frame.stroke(&short_hand, wide_stroke);
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.minute(), 60));
                frame.stroke(&long_hand, wide_stroke);
            });

            frame.with_save(|frame| {
                frame.rotate(hand_rotation(self.now.second(), 60));
                frame.stroke(&long_hand, thin_stroke);
            })
        });

        vec![clock]
    }
}

fn hand_rotation(n: u32, total: u32) -> f32 {
    let turns = n as f32 / total as f32;

    2.0 * std::f32::consts::PI * turns
}
