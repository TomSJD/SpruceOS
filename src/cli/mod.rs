use crate::cli::basic_cli::CommandLineInterface;
use spin::mutex::Mutex;
use lazy_static::lazy_static;

pub mod basic_cli;

lazy_static! {
    pub static ref CLI: Mutex<CommandLineInterface> = Mutex::new(CommandLineInterface::new());
}