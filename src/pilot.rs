#[derive(Debug)]
pub struct Pilot {
    name: String,
    age: u32,
    points: u32,
}

impl Pilot {
    pub fn new(name: String, age: u32, points: u32) -> Self {
        Pilot { name, age, points }
    }
}

impl Pilot {
    pub fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Age: {}", self.age);
        println!("Points: {}", self.points);
    }
}
