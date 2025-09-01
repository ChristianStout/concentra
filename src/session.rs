use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Session {
    pub name: String,
    pub time: Time,
    pub freq: Option<i32>
}

#[derive(Deserialize, Debug)]
pub struct Time {
    pub on: i32,
    pub off: Option<i32>,
    pub freq: Option<i32>,
    pub then: Option<Box<Time>>
}
