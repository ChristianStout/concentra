/*
CONCENTRA
---------

This application is designed to create timers for concentration, or flow timers.

It uses a `.toml` configuration file that defines `sessions` for different predefined timers
This allows you to give names to different workflows.
You can also just use it as a timer, if you would like

At it's core, it is super simple.
*/
mod config;
mod runner;

use config::*;

fn main() {
    load_config();
    println!("Hello, world!");
}
