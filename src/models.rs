use chrono::NaiveDateTime;

#[derive(Serialize, Deserialize, Debug)]
pub struct Survie {
    pub id: i32,
    pub title: String
}
