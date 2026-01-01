pub struct loan {
    pub numberId: u32,
    pub isbn: u32,
    pub days: u32,
    pub approved: bool,
}

impl loan {
    pub fn new(numberId: u32, isbn: u32, days: u32) -> Self {
        Self {
            numberId,
            isbn,
            days,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn print(&self) {
        println!(
            "loan {{ numberId: {}, isbn: {}, days: {}, approved: {} }}",
            self.numberId, self.isbn, self.days, self.approved
        );
    }
}
