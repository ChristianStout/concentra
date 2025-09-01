/*
CONCENTRA
---------

This application is designed to create timers for concentration, or flow timers.

It uses a `.toml` configuration file that defines `sessions` for different predefined timers
This allows you to give names to different workflows.
You can also just use it as a timer, if you would like

At it's core, it is super simple.
*/
mod session;
mod config;
mod runner;
mod cli;
mod app;

use session::*;
use config::*;
use cli::*;
use app::*;

fn main() {
    let cli: Cli = get_cli();
    let config: Config = load_config();
    let mut app: App = App::new();

    match &cli.command {
        Some(Commands::Begin { name }) => {
            println!("Beginning the `{}` workflow", name);
        },
        Some(Commands::Pause {  }) => {
            println!("Pausing workflow...");
        },
        Some(Commands::End {  }) => {
            println!("Ending workflow");
        },
        Some(Commands::Status { show_icon }) => {
            if let Some(icon) = show_icon {
                println!()
            }
            println!("Showing status")
        },
        Some(Commands::Now { time_on, time_off }) => {
            let session = Session {
                name: "$now".to_string(),
                time: Time {
                    on: *time_on,
                    off: *time_off,
                    freq: None,
                    then: None,
                },
                freq: None,
            };

            app.run(&session);
        }
        None => {}
    }

}

// fn main() {
//     load_config();
//     println!("Hello, world!");
// }
