pub struct member {
    pub id: u32,
    pub name: String,
    pub is_active: bool,
}

impl member {
    pub fn new(id: u32, name: String) -> Self {
        Self {
            id,
            name,
            is_active: true,
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
    }

    pub fn print(&self) {
        println!(
            "member {{ id: {}, name: {}, is_active: {} }}",
            self.id, self.name, self.is_active
        );
    }
}
