use std::io::{self, Read, Write};

pub struct Key;

pub enum Message<T> {
    KeyPress(Key),
    Quit,
    App(T),
}

pub type Command<T> = Box<dyn FnOnce() -> Option<Message<T>> + Send + 'static>;

pub trait Model<T: Copy> {
    fn init(&self) -> Option<Command<T>> {
        None
    }
    fn update(&mut self, message: Message<T>) -> Option<Command<T>>;
    fn view(&self) -> String;
}

pub struct Program<'a, T> {
    model: Box<dyn Model<T> + 'a>,
    input: Box<dyn Read>,
    output: Box<dyn Write>,
}

impl<'a, T: Copy> Program<'a, T> {
    pub fn new(model: impl Model<T> + 'a) -> Self {
        Self {
            model: Box::new(model),
            input: Box::new(io::stdin()),
            output: Box::new(io::stdout()),
        }
    }

    pub fn run(mut self) {
        // initial render
        self.output.write(self.model.view().as_bytes()).unwrap();
        self.output.flush().unwrap();

        let maybe_init_command = self.model.init();
        let mut command = {
            if let Some(init_command) = maybe_init_command {
                if let Some(message) = init_command() {
                    self.model.update(message)
                } else {
                    None
                }
            } else {
                None
            }
        };

        loop {
            // render view
            self.output.write(self.model.view().as_bytes()).unwrap();
            self.output.flush().unwrap();
            // process commands
            // update model
            command = {
                if let Some(c) = command {
                    if let Some(message) = c() {
                        match message {
                            Message::Quit => break,
                            Message::App(_) => self.model.update(message),
                            _ => None,
                        }
                    } else {
                        None
                    }
                } else {
                    None
                }
            };
        }
    }
}

pub fn quit<T>() -> Option<Command<T>> {
    Some(Box::new(|| Some(Message::Quit)))
}
