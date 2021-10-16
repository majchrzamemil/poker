use csv::Reader;
use serde::Deserialize;
use std::cmp::Ordering::{Greater, Less};
use std::error::Error;

#[derive(Deserialize)]
struct Entry {
    name: String,
    cash: f32,
}

impl Entry {
    fn cash_compare_abs(&self, rhs: &Entry) -> std::cmp::Ordering {
        if self.cash.abs() < rhs.cash.abs() {
            return Less;
        } else {
            return Greater;
        }
    }
}

struct Transfer {
    from: String,
    to: String,
    cash: f32,
}

fn read() -> Result<Vec<Entry>, Box<dyn Error>> {
    let mut rdr = Reader::from_path("poker.csv")?;
    let mut entries: Vec<Entry> = Vec::new();
    for result in rdr.deserialize() {
        let record: Entry = result?;
        entries.push(record);
    }
    Ok(entries)
}

fn settle(
    deptors: &mut Vec<Entry>,
    winners: &mut Vec<Entry>,
) -> Result<Vec<Transfer>, Box<dyn Error>> {
    let mut transfers: Vec<Transfer> = Vec::new();

    for depotr_idx in 0..deptors.len() {
        'inner: for winner_idx in 0..winners.len() {
            match deptors[depotr_idx].cash_compare_abs(&winners[winner_idx]) {
                Less => {
                    let transfer = Transfer {
                        from: String::from(&deptors[depotr_idx].name),
                        to: String::from(&winners[winner_idx].name),
                        cash: deptors[depotr_idx].cash.abs(),
                    };
                    transfers.push(transfer);
                    winners[winner_idx].cash = winners[winner_idx].cash + deptors[depotr_idx].cash;
                    deptors[depotr_idx].cash = 0.0;
                    break 'inner;
                }
                _ => {}
            }
        }
    }
    deptors.retain(|x| x.cash != 0.0);

    //this part will be invalid for more complex data.
    //There is a need to check if deptor has more or equal money than winner
    //TODO: FIX THAT SHIT ^!!
    while deptors.len() > 0 {
        for winner_idx in 0..winners.len() {
            for depotr_idx in 0..deptors.len() {
                let transfer = Transfer {
                    from: String::from(&deptors[depotr_idx].name),
                    to: String::from(&winners[winner_idx].name),
                    cash: winners[winner_idx].cash.abs(),
                };
                transfers.push(transfer);
                deptors[depotr_idx].cash = winners[winner_idx].cash + deptors[depotr_idx].cash;
                winners[winner_idx].cash = 0.0;
            }
        }
        deptors.retain(|x| x.cash > 0.01);
    }
    Ok(transfers)
}

fn main() {
    let entries = read().unwrap();
    println!("Readed CSV values:");
    for entry in &entries {
        println!("Name:{}, cash:{}", entry.name, entry.cash);
    }

    let mut deptors: Vec<Entry> = Vec::new();
    let mut winners: Vec<Entry> = Vec::new();
    for entry in entries {
        if entry.cash < 0.0 {
            deptors.push(entry);
        } else {
            winners.push(entry);
        }
    }

    winners.reverse();
    let transfers = settle(&mut deptors, &mut winners).unwrap();
    println!("----------------------------\nSettelements:");
    for transfer in &transfers {
        println!(
            "form:{}, to:{}, cash:{}",
            transfer.from, transfer.to, transfer.cash
        );
    }
}
