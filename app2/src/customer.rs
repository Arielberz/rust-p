#[derive(Clone)]
pub struct Customer {
    pub id: u32,
    pub name: String,
    pub age: u8,
}

impl Customer {
    pub fn new(id: u32, name: String, age: u8) -> Self {
        Self { id, name, age }
    }

    pub fn print(&self) {
        println!("Customer {{ id: {}, name: {}, age: {} }}", self.id, self.name, self.age);
    }
}
