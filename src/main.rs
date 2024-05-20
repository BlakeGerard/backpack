#![allow(dead_code)]

mod items;
mod packs;

use crate::items::{Item, Loc};
use crate::packs::DensePack as Pack;

use rand::prelude::*;
use std::io::{stdin, stdout, Write};
use std::time::Instant;

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
    //    println!("Final state:\n{}", pack);
}

enum Command {
    Exit,
    Help,
    ShowItems,
    ShowPack,
    Create,
    Add,
    Remove,
    Transpose,
    Move,
}

fn help_message() -> &'static str {
    let help: &'static str = "
Commands:\n\
exit\n\
help\n\
show items\n\
show pack\n\
create <name> <rows> <cols> <symbol>\n\
add <pack_name> <item_name> <row> <col>\n\
remove <pack_name> <row> <col>\n\
transpose <pack_name> <row> <col>\n\
move <pack_name> <src_row> <src_col> <dst_row> <dst_col>\n\
";
    help
}

fn parse_command(buffer: String) -> Result<Command, String> {
    let trimmed = buffer.trim();

    let result = match trimmed {
        "exit" => Ok(Command::Exit),
        "help" => Ok(Command::Help),
        _ => Err("Unknown command".to_string()),
    };
    result
}

fn interact(pack: &crate::Pack) -> bool {
    print!(">>> ");
    stdout().flush().unwrap();

    let mut buffer = String::new();
    let io_res = stdin().read_line(&mut buffer);

    // Unlikely error reading from stdin.
    if io_res.is_err() {
        return false;
    }

    let command = parse_command(buffer);

    if command.is_err() {
        return false;
    }

    match command.unwrap() {
        Command::Exit => {
            return false;
        }
        Command::Help => {
            println!("{}", help_message());
            return true;
        }
        Command::ShowItems => todo!(),
        Command::ShowPack => todo!(),
        Command::Create => todo!(),
        Command::Add => todo!(),
        Command::Remove => todo!(),
        Command::Transpose => todo!(),
        Command::Move => todo!(),
    }
}

fn main() {
    let pack = Pack::new(10, 10);
    loop {
        let decision = interact(&pack);
        if decision == false {
            break;
        }
    }

    //benchmark(20, 20, 1_000_000);

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
