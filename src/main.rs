#![allow(dead_code)]

mod items;
mod packs;

use rand::prelude::*;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};
use std::time::Instant;

use crate::items::{Item, Loc};
use crate::packs::DensePack as Pack;

fn rand_loc(rng: &mut ThreadRng, rows: u32, cols: u32) -> Loc {
    Loc::new(rng.gen_range(0..rows), rng.gen_range(0..cols))
}

fn benchmark(rows: u32, cols: u32, iters: usize) {
    let mut rng = rand::thread_rng();

    let mut pack = Pack::new(rows, cols);

    let mut locs: Vec<Loc> = Vec::new();

    let mut benchmark_data: [(u128, f64); 4] = [(0, 0.0); 4];

    for _ in 0..iters {
        let action = rng.gen_range(1..=100);

        match action {
            // add_item
            1..=70 => {
                benchmark_data[0].0 += 1;

                let symbol: u8 = rng.gen_range(33..127);
                let item = Item::new(
                    String::from(""),
                    rng.gen_range(0..rows / 2),
                    rng.gen_range(0..cols / 2),
                    symbol as char,
                );
                let loc = rand_loc(&mut rng, rows, cols);

                let start = Instant::now();
                let result = pack.add_item(item, loc);
                let elapsed = start.elapsed().as_nanos() as f64;

                let delta = elapsed - benchmark_data[0].1;
                benchmark_data[0].1 += delta / benchmark_data[0].0 as f64;

                if result.is_ok() {
                    locs.push(loc);
                }
            }
            // remove item
            71..=80 => {
                if locs.is_empty() {
                    continue;
                }
                benchmark_data[1].0 += 1;

                let idx = rng.gen_range(0..locs.len());
                let loc = locs[idx];

                let start = Instant::now();
                let result = pack.remove_item(loc);
                let elapsed = start.elapsed().as_nanos() as f64;
                let delta = elapsed - benchmark_data[1].1;
                benchmark_data[1].1 += delta / benchmark_data[1].0 as f64;

                locs.swap_remove(idx);
            }
            // tranpose item
            81..=90 => {
                if locs.is_empty() {
                    continue;
                }
                benchmark_data[2].0 += 1;

                let idx = rng.gen_range(0..locs.len());
                let loc = locs[idx];

                let start = Instant::now();
                let _result = pack.transpose_item(loc);
                let elapsed = start.elapsed().as_nanos() as f64;
                let delta = elapsed - benchmark_data[2].1;
                benchmark_data[2].1 += delta / benchmark_data[2].0 as f64;
            }
            // move item
            91..=100 => {
                if locs.is_empty() {
                    continue;
                }
                benchmark_data[3].0 += 1;

                let idx = rng.gen_range(0..locs.len());
                let src = locs[idx];
                let dst = rand_loc(&mut rng, rows, cols);

                let start = Instant::now();
                let result = pack.move_item(src, dst);
                let elapsed = start.elapsed().as_nanos() as f64;
                let delta = elapsed - benchmark_data[3].1;
                benchmark_data[3].1 += delta / benchmark_data[3].0 as f64;

                if result.is_ok() {
                    locs[idx] = dst;
                }
            }
            _ => {
                unreachable!()
            }
        }
    }

    println!("Benchmark Results");
    println!("Action: Count, Avg Duration (nanos)");
    for (i, entry) in benchmark_data.iter().enumerate() {
        println!("{}: {}, {}", i, entry.0, entry.1);
    }
    println!("Final state:\n{}", pack);
}

fn help_message() -> &'static str {
    let help: &'static str = "
Commands:\n\
exit\n\
help\n\
items\n\
pack\n\
new <name> <rows> <cols> <symbol>\n\
add <name> <row> <col>\n\
remove <name>\n\
transpose <name>\n\
move <name> <dst_row> <dst_col>\n\
";
    help
}

fn interact(items: &mut Vec<Item>, pack: &mut Pack) -> bool {
    // Read from stdin.
    let mut buffer = String::new();
    let io_res = stdin().read_line(&mut buffer);
    if io_res.is_err() {
        return false;
    }

    // Some pre-processing.
    let mut words = buffer.trim().split_whitespace();

    // Consume the command token.
    let command = words.next();
    if command.is_none() {
        return true;
    }

    match command.unwrap() {
        "exit" => {
            return false;
        }
        "help" => {
            println!("{}", help_message());
            return true;
        }
        "items" => {
            print!("Items: [ ");
            for item in items {
                print!("{} ", item.name());
            }
            println!("]");
            return true;
        }
        "pack" => {
            println!("{}", pack);
            return true;
        }
        "new" => 'new: {
            let name = match words.next() {
                Some(name) => name,
                None => {
                    println!("Expected an item name.");
                    break 'new;
                }
            };

            if items.contains_key(name) {
                println!("'{}' already exists.", name);
                break 'new;
            }

            let symbol = match words.next().and_then(|s| s.parse::<char>().ok()) {
                Some(symbol) => symbol,
                None => {
                    println!("Expected an item symbol character.");
                    break 'new;
                }
            };

            let rows = match words.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(rows) => rows,
                None => {
                    println!("Expected a non-negative integer number of rows.");
                    break 'new;
                }
            };

            let cols = match words.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(cols) => cols,
                None => {
                    println!("Expected a non-negative integer number of columns.");
                    break 'new;
                }
            };

            let item = Item::new(name, rows, cols, symbol);
            items.insert(name.to_owned(), item);
        }
        "add" => 'add: {
            let name = match words.next() {
                Some(name) => name.to_owned(),
                None => {
                    println!("Expected an item name.");
                    break 'add;
                }
            };

            if !items.contains_key(&name) {
                println!(
                    "'{}' does not exist. Add this item with the 'new' command.",
                    name
                );
                break 'add;
            }

            let item = items.get(&name).unwrap();

            let row = match words.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(row) => row,
                None => {
                    println!("Expected a non-negative integer row coordinate value.");
                    break 'add;
                }
            };

            let col = match words.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(col) => col,
                None => {
                    println!("Expected a non-negative integer column coordinate value.");
                    break 'add;
                }
            };

            let result = pack.add_item(item.clone(), Loc::new(row, col));
            if result.is_err() {
                println!("Item insertion failed: {}", result.err().unwrap());
            }
        }
        "remove" => {
            todo!();
        }
        "tranpose" => {
            todo!();
        }
        "move" => {
            todo!();
        }
        _ => {
            println!("Unknown command");
        }
    }
    return true;
}

fn main() {
    let mut pack = Pack::new(10, 10);
    let mut items: Vec<Item> = Vec::new();
    loop {
        print!(">>> ");
        stdout().flush().unwrap();

        let decision = interact(&mut items, &mut pack);
        if decision == false {
            break;
        }
    }

    // benchmark(20, 20, 1_000_000);
}
