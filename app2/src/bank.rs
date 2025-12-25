use crate::account::Account;
use crate::transaction::Transaction;

pub struct Bank {
    pub name: String,
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

impl Bank {
    pub fn new(name: String) -> Self {
        Self {
            name,
            accounts: Vec::new(),
            transactions: Vec::new(),
        }
    }

    pub fn add_account(&mut self, account: Account) {
        self.accounts.push(account); 
    }

    pub fn print(&self) {
        println!("==============================");
        println!("Bank: {}", self.name);

        println!("--- Accounts ({}):", self.accounts.len());
        for i in 0..self.accounts.len() {
            print!("[{}] ", i);
            self.accounts[i].print();
        }

        println!("--- Transactions ({}):", self.transactions.len());
        for i in 0..self.transactions.len() {
            print!("[{}] ", i);
            self.transactions[i].print();
        }
        println!("==============================");
    }

    pub fn find_account_index(&self, number: u32) -> i32 {
        for i in 0..self.accounts.len() {
            if self.accounts[i].number == number {
                return i as i32;
            }
        }
        -1
    }

    pub fn transfer(&mut self, from: u32, to: u32, amount: i32) -> bool {
        let mut tx = Transaction::new(from, to, amount);

        if amount <= 0 {
            self.transactions.push(tx);
            return false;
        }

        let from_i32 = self.find_account_index(from);
        let to_i32 = self.find_account_index(to);

        if from_i32 < 0 || to_i32 < 0 {
            self.transactions.push(tx);
            return false;
        }

        let from_idx = from_i32 as usize;
        let to_idx = to_i32 as usize;

        if from_idx == to_idx {
            self.transactions.push(tx);
            return false;
        }

        let success: bool;

        if from_idx < to_idx {
            let (left, right) = self.accounts.split_at_mut(to_idx);
            let from_acc = &mut left[from_idx];
            let to_acc = &mut right[0];

            // withdraw ואז deposit רק אם הצליח
            if from_acc.withdraw(amount) {
                to_acc.deposit(amount);
                tx.approve();
                success = true;
            } else {
                success = false;
            }
        } else {
            let (left, right) = self.accounts.split_at_mut(from_idx);
            let to_acc = &mut left[to_idx];
            let from_acc = &mut right[0];

            if from_acc.withdraw(amount) {
                to_acc.deposit(amount);
                tx.approve();
                success = true;
            } else {
                success = false;
            }
        }

        self.transactions.push(tx);
        success
    }
}
