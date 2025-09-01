use crate::session::*;
use std::process::Command;
use termion::*;
use std::io::stdin;

pub enum TimerState {
    Flow,
    Break,
    Paused,
    NotRunning,
}

pub struct App {
    current: i32,
    state: TimerState,
}

impl App {
    pub fn new() -> App {
        App {
            current: 0,
            state: TimerState::NotRunning,
        }
    }

    pub fn run(&mut self, session: &Session) {
        let freq = session.freq.unwrap_or(1);

        for _ in 0..freq {
            self.run_timer(&session.time);
        }

        self.display_time();
        // while std::io::stdin() != 'q' {
        //     self.display_time();
        // }
        print!("{}", termion::cursor::Show);
    }

    fn run_timer(&mut self, time: &Time) {
        let freq = time.freq.unwrap_or(1);

        for i in 0..freq {
            self.run_timer_single(time.on, TimerState::Flow);
            
            if i >= freq - 1 {
                break;
            }
            
            if let Some(seconds) = time.off {
                self.run_timer_single(seconds, TimerState::Break);
            }
        }

        if let Some(then) = &time.then {
            self.run_timer(&then);
        }
    }

    fn run_timer_single(&mut self, seconds: i32, state: TimerState) {
        self.state = state;
        self.current = seconds;
        for _ in 0..seconds {
            self.display_time();
            self.decrement();
            let mut child = Command::new("sleep").arg("1").spawn().unwrap();
            let _result = child.wait().unwrap();
        }
    }
    
    fn decrement(&mut self) {
        self.current -= 1;
    }

    fn display_time(&self) {
        let (x, y) = termion::terminal_size().unwrap();
        let mut display_text = String::new();

        let minutes = self.current / 60;
        let seconds = self.current % 60;

        display_text.push_str(format!("{:02}:{:02}", minutes, seconds).as_str());

        print!("{}{}{}", termion::clear::All, termion::cursor::Goto(x / 2, y / 2), termion::cursor::Hide);
        println!("{}{}", display_text, termion::cursor::Goto(1, y - 1));
    }
}
