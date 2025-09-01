/*
The `Runner` struct handles everything involving the actual
running of the program. So it runs the timers, and eventually
will play all of the sounds, including notification sounds and
music.
*/
use crate::config::*;
use crate::session::*;
use std::string::FromUtf8Error;
use std::thread;
use std::time::Duration;

pub enum TimerType {
    NotRunning,
    Flow,
    Break,
}

pub struct Runner {
    timer_status_time_remaining: i32,
    timer_status_type: TimerType,
}

impl Runner {
    pub fn run(&mut self, session: &Session) {
        // if config.service {
        //     // TODO: ensure `concentra.service`
        // }
        self.timer(&session.time);
    }

    fn timer(&mut self, time: &Time) {
        let freq = time.freq.unwrap_or(1);

        for _ in 0..freq {
            self.timer_status_type = TimerType::Flow;

            let timer = thread::spawn(|| {
                thread::sleep(Duration::from_secs(1));
                println!("Timer 1 fired");
            });
        }
    }
}
