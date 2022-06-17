use moonshine::{Command, Message, Model, Program};
use std::time::Duration;

struct Ticker(u64);

#[derive(Debug)]
enum TickerMessage {
    Tick,
}

impl Model for Ticker {
    type Message = TickerMessage;

    fn init(&self) -> Option<Command<TickerMessage>> {
        Some(Box::new(tick))
    }

    fn update(&mut self, message: Message<TickerMessage>) -> Option<Command<TickerMessage>> {
        match message {
            Message::App(TickerMessage::Tick) => {
                self.0 -= 1;

                if self.0 <= 0 {
                    return moonshine::quit();
                }

                Some(Box::new(tick))
            }
            Message::KeyPress(_) => return moonshine::quit(),
            _ => None,
        }
    }

    fn view(&self) -> String {
        format!(
            "Hi. This program will exit in {} seconds. To quit sooner press any key.\n",
            self.0
        )
    }
}

fn tick() -> Option<Message<TickerMessage>> {
    std::thread::sleep(Duration::from_millis(1000));
    Some(Message::App(TickerMessage::Tick))
}

fn main() {
    let program = Program::new(Ticker(5));
    program.run();
}
