#[derive(Clone)]
pub struct Transaction {
    pub from_account: u32,
    pub to_account: u32,
    pub amount: i32,
    pub approved: bool,
}

impl Transaction {
    pub fn new(from_account: u32, to_account: u32, amount: i32) -> Self {
        Self {
            from_account,
            to_account,
            amount,
            approved: false,
        }
    }

    pub fn approve(&mut self) {
        self.approved = true;
    }

    pub fn print(&self) {
        println!(
            "Transaction {{ from: {}, to: {}, amount: {}, approved: {} }}",
            self.from_account, self.to_account, self.amount, self.approved
        );
    }
}
