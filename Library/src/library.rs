use crate::book::book;
use crate::loan::loan;
use crate::member::member;

pub struct library {
    pub name: String,
    pub members: Vec<member>,
    pub books: Vec<book>,
    pub loans: Vec<loan>,
}

impl library {
    pub fn new(name: String) -> Self {
        Self {
            name,
            members: Vec::new(),
            books: Vec::new(),
            loans: Vec::new(),
        }
    }

    pub fn add_member(&mut self, m: member) {
        self.members.push(m);
    }

    pub fn add_book(&mut self, b: book) {
        self.books.push(b);
    }

    pub fn add_loan(&mut self, l: loan) {
        self.loans.push(l);
    }

    pub fn find_member_index(&self, member_id: u32) -> i32 {
        for i in 0..self.members.len() {
            if self.members[i].id == member_id {
                return i as i32;
            }
        }
        -1
    }

    pub fn find_book_index(&self, isbn: u32) -> i32 {
        for i in 0..self.books.len() {
            if self.books[i].isbn == isbn {
                return i as i32;
            }
        }
        -1
    }

    pub fn deactivate_member(&mut self, member_id: u32) -> bool {
        let mi = self.find_member_index(member_id);
        if mi == -1 {
            return false;
        }
        self.members[mi as usize].deactivate();
        true
    }

    pub fn borrow_book(&mut self, member_id: u32, isbn: u32, days: u32) -> bool {
        let member_index = self.find_member_index(member_id);
        let book_index = self.find_book_index(isbn);

        if member_index == -1 || book_index == -1 {
            return false;
        }

        let mi = member_index as usize;
        let bi = book_index as usize;

        if !self.members[mi].is_active {
            return false;
        }

        let copy_ok = {
            let b = &mut self.books[bi];
            b.borrow_copy()
        };

        if !copy_ok {
            return false;
        }

        let mut l = loan::new(member_id, isbn, days);
        l.approve();
        self.add_loan(l);

        true
    }

    pub fn return_book(&mut self, member_id: u32, isbn: u32) -> bool {
        let mut loan_index: i32 = -1;
        for i in 0..self.loans.len() {
            if self.loans[i].numberId == member_id && self.loans[i].isbn == isbn {
                loan_index = i as i32;
                break;
            }
        }
        if loan_index == -1 {
            return false;
        }

        let book_index = self.find_book_index(isbn);
        if book_index == -1 {
            return false;
        }
        let bi = book_index as usize;

        {
            let b = &mut self.books[bi];
            b.return_copy();
        }

        self.loans.remove(loan_index as usize);
        true
    }

    pub fn print(&self) {
        println!("\n=== library: {} ===", self.name);

        println!("-- members --");
        for i in 0..self.members.len() {
            self.members[i].print();
        }

        println!("-- books --");
        for i in 0..self.books.len() {
            self.books[i].print();
        }

        println!("-- loans --");
        for i in 0..self.loans.len() {
            self.loans[i].print();
        }

        println!("====================\n");
    }
}
