use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use toml::Table;
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize, Debug)]
pub struct Config {
    service: bool,
    start_sound: String,
    break_sound: String,
    end_sound: String,
    session: Option<Vec<Session>>
}

#[derive(Deserialize, Debug)]
pub struct Session {
    name: String,
    time: Time,
    freq: Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct Time {
    on: i32,
    off: Option<i32>,
    freq: Option<i32>,
    then: Option<Box<Time>>
}

pub fn load_config() {
 
    if let Some(proj_dirs) = ProjectDirs::from("io", "christianstout",  "concentra") {
        proj_dirs.config_dir();
        // Lin: /home/alice/.config/barapp
        // Win: C:\Users\Alice\AppData\Roaming\Foo Corp\Bar App\config
        // Mac: /Users/Alice/Library/Application Support/com.Foo-Corp.Bar-App
        println!("{}", proj_dirs.config_dir().display());
    }
    let mut config_file = File::open("examples/default.toml")
        .expect("Expected to be able to open the file");

    let mut contents = String::new();

    config_file.read_to_string(&mut contents)
        .expect("Was unable to read the contents of the file into a String");
    // let config_file = fs::read_to_tring("examples/example.toml").expect;
    // let value = contents.parse::<Table>().unwrap();
    // println!("{:?}", value);
    println!("Hello, world!");
    let config: Config = toml::from_str(&contents)
        .expect("Was unable to serialize the contents of the toml file into the Config struct");
    dbg!(config);
}