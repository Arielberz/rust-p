mod customer;
mod account;
mod transaction;
mod bank;

use customer::Customer;
use account::Account;
use bank::Bank;

fn main() {
    let c1 = Customer::new(1, "Ariel".to_string(), 21);
    let c2 = Customer::new(2, "Noam".to_string(), 22);

    let a1 = Account::new(1001, 500, c1.clone());
    let a2 = Account::new(2002, 200, c2.clone());

    let mut bank = Bank::new("Helper Bank".to_string());
    bank.add_account(a1);
    bank.add_account(a2);

    bank.print();

    let ok1 = bank.transfer(1001, 2002, 100);
    println!("Transfer 1 success? {}", ok1);
    bank.print();

    let ok2 = bank.transfer(1001, 2002, 10_000);
    println!("Transfer 2 success? {}", ok2);
    bank.print();

    let idx = bank.find_account_index(2002);
    if idx >= 0 {
        let i = idx as usize;
        bank.accounts[i].change_owner(c1.clone());
    }

    bank.print();
}
