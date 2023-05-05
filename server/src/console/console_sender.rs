use super::console_handler::ConsoleHandler;

pub trait ConsoleSender {
    fn get_name(&self) -> &String;
    fn send_console_message(&self, message: String);
}

pub struct Console {
    name: String,
}

impl Console {
    pub fn init() -> Self {
        Console {
            name: "Console".to_string(),
        }
    }
}

impl ConsoleSender for Console {
    fn get_name(&self) -> &String {
        &self.name
    }

    fn send_console_message(&self, message: String) {
        ConsoleHandler::send_message(message)
    }
}