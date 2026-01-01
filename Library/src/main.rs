mod member;
mod book;
mod loan;
mod library;

fn main() {
    let m1_id: u32 = 1;
    let m2_id: u32 = 2;

    let b1_isbn: u32 = 1111;
    let b2_isbn: u32 = 2222;

    let m1 = member::member::new(m1_id, "Ariel".to_string());
    let m2 = member::member::new(m2_id, "Noam".to_string());

    let b1 = book::book::new(b1_isbn, "Rust Basics".to_string(), 1);
    let b2 = book::book::new(b2_isbn, "Ownership Deep Dive".to_string(), 2);

    let mut lib = library::library::new("City Library".to_string());
    lib.add_member(m1);
    lib.add_member(m2);
    lib.add_book(b1);
    lib.add_book(b2);

    lib.print();

    let ok1 = lib.borrow_book(m1_id, b1_isbn, 7);
    println!("borrow 1 (should be true): {}", ok1);
    lib.print();

    let ok2 = lib.borrow_book(m1_id, b1_isbn, 7);
    println!("borrow 2 same book (should be false): {}", ok2);
    lib.print();

    let ok3 = lib.return_book(m1_id, b1_isbn);
    println!("return (should be true): {}", ok3);
    lib.print();

    let ok4 = lib.deactivate_member(m1_id);
    println!("deactivate member {} (should be true): {}", m1_id, ok4);
    lib.print();

    let ok5 = lib.borrow_book(m1_id, b2_isbn, 5);
    println!("borrow after deactivate (should be false): {}", ok5);
    lib.print();
}
