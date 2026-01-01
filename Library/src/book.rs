#[derive(Clone)]
pub struct book {
    pub isbn: u32,
    pub title: String,
    pub copies_total: u32,
    pub copies_available: u32,
}

impl book {
    pub fn new(isbn: u32, title: String, copies_total: u32) -> Self {
        Self {
            isbn,
            title,
            copies_total,
            copies_available: copies_total,
        }
    }

    pub fn borrow_copy(&mut self) -> bool {
        if self.copies_available == 0 {
            false
        } else {
            self.copies_available -= 1;
            true
        }
    }

    pub fn return_copy(&mut self) {
        if self.copies_available < self.copies_total {
            self.copies_available += 1;
        }
    }

    pub fn print(&self) {
        println!(
            "book {{ isbn: {}, title: {}, copies_total: {}, copies_available: {} }}",
            self.isbn, self.title, self.copies_total, self.copies_available
        );
    }
}
