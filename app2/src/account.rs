use crate::customer::Customer;

pub struct Account {
    pub number: u32,
    pub balance: i32,
    pub owner: Customer,
}

impl Account {
    pub fn new(number: u32, balance: i32, owner: Customer) -> Self {
        Self { number, balance, owner }
    }

    pub fn deposit(&mut self, amount: i32) {
        if amount <= 0 {
            return;
        }
        self.balance += amount;
    }

    pub fn withdraw(&mut self, amount: i32) -> bool {
        if amount <= 0 {
            return false;
        }

        if self.balance - amount < 0 {
            return false;
        }
        self.balance -= amount;
        true
    }

    pub fn change_owner(&mut self, new_owner: Customer) {
        self.owner = new_owner; 
    }

    pub fn print(&self) {
        println!("Account {{ number: {}, balance: {} }}", self.number, self.balance);
        print!("  owner: ");
        self.owner.print();
    }
}
