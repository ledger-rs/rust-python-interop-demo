/*!
 * Demo of Python-Rust interop
 */

use chrono::naive::NaiveDate;

fn main() {
    // println!("Hello, world!");
    demo();
}

/// Create a big structure in Rust, simulating a parser,
/// and return to Python through a method call
fn demo() {
    let x = create_structure();

}

fn create_structure() -> Journal {
    let journal = Journal {
        xacts: vec![]
    };

    // todo!("populate data")

    journal
}

#[derive(Debug)]
pub struct Journal {
    xacts: Vec<Xact>,
}

#[derive(Debug)]
pub struct Xact {
    date: NaiveDate,
    payee: String,
}