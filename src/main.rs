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
add <pack_name> <item_name> <row> <col>\n\
remove <pack_name> <row> <col>\n\
transpose <pack_name> <row> <col>\n\
move <pack_name> <src_row> <src_col> <dst_row> <dst_col>\n\
";
    help
}

fn interact(items: &mut HashMap<String, crate::Item>, pack: &mut crate::Pack) -> bool {
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
            for (name, _) in items.iter() {
                print!("{} ", name);
            }
            println!("]");
            return true;
        }
        "pack" => {
            println!("{}", pack);
            return true;
        }
        "new" => 'new: {
            let name = words.next();
            if name.is_none() {
                println!("Expected an item name");
                break 'new;
            }

            let symbol = words.next();
            if symbol.is_none() {
                println!("Expected an item symbol character");
                break 'new;
            }
            let symbol = symbol.unwrap().parse::<char>();
            if symbol.is_err() {
                println!("Expected an item symbol character");
                break 'new;
            }

            let row = words.next();
            if row.is_none() {
                println!("Expected a row value");
                break 'new;
            }
            let row = row.unwrap().parse::<u32>();
            if row.is_err() {
                println!("Expected a non-negative integer row value");
                break 'new;
            }

            let col = words.next();
            if col.is_none() {
                println!("Expected a col value");
                break 'new;
            }
            let col = col.unwrap().parse::<u32>();
            if col.is_err() {
                println!("Expected a non-negative integer col value");
                break 'new;
            }
            let item = Item::new(row.unwrap(), col.unwrap(), symbol.unwrap());
            items.insert(name.unwrap().to_string(), item);
        }
        "add" => {
            todo!();
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
    let mut items: HashMap<String, Item> = HashMap::new();
    loop {
        print!(">>> ");
        stdout().flush().unwrap();

        let decision = interact(&mut items, &mut pack);
        if decision == false {
            break;
        }
    }

    // benchmark(20, 20, 1_000_000);

    // // Initialize the Pack
    // let mut pack = GridPack::new(5, 5);
    // println!("{}", pack);

    // // Add a 'stick'
    // let stick = Item::new(1, 3, '.');
    // let result = pack.add_item(stick, (1, 2));
    // if result.is_err() {
    //     println!("{}", result.err().unwrap());
    // }
    // println!("{}", pack);

    // // Add a 'rock'
    // let rock = Item::new(2, 2, '@');
    // let result = pack.add_item(rock, (2, 2));
    // if result.is_err() {
    //     println!("{}", result.err().unwrap());
    // }
    // println!("{}", pack);

    // // transpose the stick, this will result in an error.
    // let result = pack.transpose_item((1, 2));
    // if result.is_err() {
    //     println!("{}", result.err().unwrap());
    // }
    // println!("{}", pack);

    // // Move the stick away from the rock.
    // let result = pack.move_item((1, 2), (0, 0));
    // if result.is_err() {
    //     println!("{}", result.err().unwrap());
    // }
    // println!("{}", pack);

    // // transpose the stick, this will result in an error.
    // let result = pack.transpose_item((0, 0));
    // if result.is_err() {
    //     println!("{}", result.err().unwrap());
    // }
    // println!("{}", pack);
}
