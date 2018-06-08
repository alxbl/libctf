pub struct User {
    id: i32,
    handle: String,
    access: i32,
    assigned: Option<i32>,
}

pub struct Challenge {
    id: i32,
    points: i32,
    submissions: i32,
    url: String,
    flag: String,
}

pub enum Trail {
}


pub struct Event {
    name: String,
    url: String,
}
