use alloc::string::String;
use crate::println;

pub struct CommandLineInterface {
    buffer: String,
}

impl CommandLineInterface {
    pub fn new() -> Self {
        CommandLineInterface {
            buffer: String::new(),
        }
    }

    /// Function to have keys passed into the CLI so they can be added
    /// to the buffer, ready for parsing.
    pub fn handle_key(&mut self, key: char) {
        match key {
            // Call process_command when enter is pressed
            '\n' => {
                self.process_command();
            }
            // Handle backspace by removing the last character
            '\u{8}' => {
                self.buffer.pop();
            }
            // If not a command key pressed, append the char to the buffer
            _ => {
                self.buffer.push(key);
            }
        }
    }

    /// Function to process the command in the buffer once enter key is pressed.
    fn process_command(&mut self) {
        println!("{}", self.buffer);

        match self.buffer {
            _ => {
                println!("Unknown Command! Try typing help to get some basic commands.");
            }
        }
        // After the command has been parsed and executed,
        // the buffer is cleared for the next command.
        self.buffer.clear();
    }
}