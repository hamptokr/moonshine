use std::io::{self, Read, Stdin, Stdout, Write};

pub struct Key;

pub enum Message<T> {
    KeyPress(Key),
    Quit,
    App(T),
}

pub type Command<T> = Box<dyn FnOnce() -> Option<Message<T>> + Send + 'static>;

pub trait Model {
    type Message;

    fn init(&self) -> Option<Command<Self::Message>> {
        None
    }
    fn update(&mut self, message: Message<Self::Message>) -> Option<Command<Self::Message>>;
    fn view(&self) -> String;
}

pub struct Program<M, I, O, R> {
    model: M,
    input: I,
    output: O,
    renderer: R,
}

impl<M> Program<M, Stdin, Stdout, FramerateRenderer>
where
    M: Model,
{
    pub fn new(model: M) -> Self {
        Self {
            model,
            input: io::stdin(),
            output: io::stdout(),
            renderer: FramerateRenderer {},
        }
    }
}

impl<M, I, O, R> Program<M, I, O, R>
where
    M: Model,
    I: Read,
    O: Write,
    R: Renderer,
{
    // TODO(kramer): this is hard-coded for example purposes, replace this with renderer trait calls
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

pub trait Renderer {}

pub struct FramerateRenderer {}

impl Renderer for FramerateRenderer {}

pub fn quit<T>() -> Option<Command<T>> {
    Some(Box::new(|| Some(Message::Quit)))
}
