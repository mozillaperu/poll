use std::collections::HashMap;
//use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Survie {
    pub id: i32,
    pub title: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Answer {
    pub id: i32,
    pub name: String,
    pub value: String
}
