extern crate baseball;
extern crate csv;
extern crate rustc_serialize;

use std::env;
use std::clone::Clone;

use csv::Reader;
use csv::Writer;

use baseball::retrosheet::transactions;


struct Trade {
    transactions: Vec<transactions::Transaction>,
    links: Vec<Option<Box<Link>>>,
}

struct Link {
    player: String,
    transaction: transactions::Transaction,
}


fn load_transactions(file: &str) -> Vec<transactions::Transaction> {
    let mut csv_reader = Reader::from_file(file)
                            .expect("Couldn't open file.")
                            .has_headers(false);

    let mut transactions = csv_reader.decode().collect::<csv::Result<Vec<transactions::Transaction>>>().unwrap();
    transactions.sort_by(|a, b| {
        a.primary_date.cmp(&b.primary_date)
    });
    transactions
}

fn group_transactions(transactions: &Vec<transactions::Transaction>) {
}

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() != 2 {
        println!("Arguments: tran.txt");
        return;
    }

    let tran_txt = &args[1];
    let transactions = load_transactions(&tran_txt);
}
