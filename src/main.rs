use chrono::Timelike;

use play;
use std::thread;

use std::process;

use iced::{
    canvas::{self, Cache, Canvas, Cursor, Geometry, LineCap, Path, Stroke},
    executor, time,
    widget::Text,
    window::Settings as WindowSettings,
    Align, Application, Color, Column, Command, Container, Element, Length, Point, Rectangle, Row,
    Settings, Space, Subscription, Vector,
};
use iced_native::event::Event;
use iced_native::keyboard::Event as KeyboardEvent;

const WORK_LENGTH: u32 = 25;
const REST_LENGTH: u32 = 5;

const DATA_FOLDER: &str = "/usr/share/pomodorust";

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
    count: u32,
    total_work: u32,
    total_rest: u32,
    work_sessions: u32,
    work: bool,
    now: chrono::DateTime<chrono::Local>,
    paused: bool,
    previous: u32,
    clock: Cache,
}

#[derive(Debug, Clone)]
enum Message {
    Tick(chrono::DateTime<chrono::Local>),
    EventOccured(iced_native::Event),
}

impl Application for Clock {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Clock {
                count: 0,
                total_work: WORK_LENGTH * 60,
                total_rest: REST_LENGTH * 60,
                work_sessions: 0,
                work: true,
                now: chrono::Local::now(),
                paused: false,
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
                if self.previous != second && !self.paused {
                    self.previous = second;
                    self.count += 1;

                    if self.count
                        >= match self.work {
                            true => self.total_work,
                            false => self.total_rest,
                        }
                    {
                        self.work = !self.work;
                        self.count = 0;
                        play_sound();
                        if let true = self.work {
                            self.paused = true;
                            self.work_sessions += 1;
                        }
                    }
                }
            }
            Message::EventOccured(event) => {
                if let Event::Keyboard(keyboard_event) = event {
                    if let KeyboardEvent::CharacterReceived(ch) = keyboard_event {
                        match ch {
                            ' ' => {
                                self.paused = !self.paused;
                                self.clock.clear();
                            }
                            'q' => {
                                process::exit(0);
                            }
                            'r' => {
                                self.count = 0;
                                self.work = true;
                                self.work_sessions = 0;
                                self.clock.clear();
                            }
                            'n' => {
                                self.count = 0;
                                self.work = !self.work;
                                self.clock.clear();
                            }
                            'h' => {
                                self.count -= 60;
                                self.clock.clear();
                            }
                            'l' => {
                                self.count += 60;
                                self.clock.clear();
                            }
                            _ => {}
                        }
                    }
                }
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced_native::subscription::events().map(Message::EventOccured),
            time::every(std::time::Duration::from_millis(500))
                .map(|_| Message::Tick(chrono::Local::now())),
        ])
    }

    fn view(&mut self) -> Element<Message> {
        let minutes: u32 = self.count / 60;
        let seconds: u32 = self.count - 60 * minutes;

        let current = format!("{:0>#2}:{:0>#2}", minutes, seconds);
        let total = match self.work {
            true => format!("{:0>#2}:00", WORK_LENGTH),
            false => format!("{:0>#2}:00", REST_LENGTH),
        };
        let timer = Text::new(format!("{}/{}", current, total)).size(40);

        let state_text = match self.paused {
            false => match self.work {
                true => format!("work"),
                false => format!("rest"),
            },
            true => format!("STOP"),
        };
        let state = Text::new(state_text).size(40);

        let work_sessions = Text::new(self.work_sessions.to_string()).size(30);

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

        let row = Row::new()
            .push(timer)
            .push(Space::new(Length::Units(50), Length::Shrink))
            .push(state)
            .width(Length::Fill);
        Column::new()
            .padding(20)
            .push(canvas)
            .push(work_sessions)
            .push(row)
            .into()
    }
}

impl canvas::Program<Message> for Clock {
    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {
        let clock = self.clock.draw(bounds.size(), |frame| {
            let center = frame.center();
            let radius = frame.width().min(frame.height()) / 2.0;

            let background = Path::circle(center, radius);

            let color: Color = match self.paused {
                false => match self.work {
                    true => Color::from_rgb8(0xc2, 0x23, 0x30),
                    false => Color::from_rgb8(0x19, 0xa8, 0x5b),
                },
                true => Color::from_rgb8(0x77, 0x77, 0x77),
            };
            frame.fill(&background, color);

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

fn play_sound() {
    thread::spawn(|| {
        play::play(format!("{}/{}", DATA_FOLDER, "/assets/pomodoro.mp3")).unwrap();
    });
}
