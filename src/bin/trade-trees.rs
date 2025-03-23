
use std::collections::HashMap;
use std::env;
use std::path::Path;

use baseball::transactions::Transaction;


struct Trade {
    transactions: Vec<Transaction>,
    links: Vec<Option<Box<Link>>>,
}

struct Link {
    player: String,
    transaction: Transaction,
}


fn load_transactions(file: &str) -> Vec<Transaction> {
    let mut transactions = Transaction::load_transactions(Path::new(file));
    transactions.sort_by(|a, b| {
        a.primary_date.cmp(&b.primary_date)
    });
    transactions
}

fn group_transactions(transactions: Vec<Transaction>) {
    let mut players = HashMap::new();
    for transaction in &transactions {
        let player_id = &transaction.player;
        let player_transactions = players.entry(player_id).or_insert(Vec::new());
        player_transactions.push(transaction);
    }
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
