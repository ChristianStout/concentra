use crate::session::Session;
use std::process::Command;
use termion::*;
use std::io::stdin;

pub struct App {
    current: i32,
}

impl App {
    pub fn new() -> App {
        App {
            current: 0,
        }
    }

    pub fn run(&mut self, session: &Session) {
        self.current = session.time.on;
        for _ in 0..session.time.on {
            self.display_time();
            self.decrement();
            let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            let _result = child.wait().unwrap();
        }
        self.display_time();
    }

    fn decrement(&mut self) {
        self.current -= 1;
    }

    fn display_time(&self) {
        let (x, y) = termion::terminal_size().unwrap();
        let mut display_text = String::new();

        let minutes = self.current / 60;
        let seconds = self.current % 60;

        for _ in 0..(y / 2) {
            display_text.push('\n');
        }

        for _ in 0..(x / 2) {
            display_text.push(' ');
        }

        display_text.push_str(format!("{:02}:{:02}", minutes, seconds).as_str());

        for _ in 0..(y / 2) {
            display_text.push('\n');
        }

        // print!("{}[2J", 27 as char);

        // Command::new("clear").spawn().unwrap();
        println!("{}", display_text);
    }
}
