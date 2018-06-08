pub struct User {
    pub id: i32,
    pub handle: String,
    pub access: i32,
    pub assigned: Option<i32>,
}

pub struct Challenge {
    pub id: i32,
    pub points: i32,
    pub submissions: i32,
    pub url: String,
    pub flag: String,
}

pub enum Trail {
}


pub struct Event {
    pub name: String,
    pub url: String,
}
