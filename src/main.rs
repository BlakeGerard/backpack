#![allow(dead_code)]

mod items;
mod packs;

use std::io::{stdin, stdout, Write};
use std::str::SplitWhitespace;

use crate::items::{Item, Loc};
use crate::packs::DensePack as Pack;

fn help_message() -> &'static str {
    let help: &'static str = "
Commands:\n\
exit\n\
help\n\
show store\n\
show pack\n\
pack <src row> <src col> <target row> <target col>\n\
store <src row> <src col> <target row> <target col>\n\
packtranspose  <src row> <src col>\n\
";
    help
}

fn move_between(mut words: SplitWhitespace, src: &mut Pack, dst: &mut Pack) -> Result<(), String> {
    let src_row = match words.next().and_then(|s| s.parse::<u32>().ok()) {
        Some(row) => row,
        None => {
            return Err("Expected a non-negative integer row coordinate value.".to_string());
        }
    };

    let src_col = match words.next().and_then(|s| s.parse::<u32>().ok()) {
        Some(col) => col,
        None => {
            return Err("Expected a non-negative integer column coordinate value.".to_string());
        }
    };
    let src_loc = Loc::new(src_row, src_col);
    let src_item = src.take_item(&src_loc);
    if src_item.is_none() {
        return Err("No item there.".to_string());
    }
    let src_item = src_item.unwrap();

    let dst_row = match words.next().and_then(|s| s.parse::<u32>().ok()) {
        Some(row) => row,
        None => {
            return Err("Expected a non-negative integer row coordinate value.".to_string());
        }
    };

    let dst_col = match words.next().and_then(|s| s.parse::<u32>().ok()) {
        Some(col) => col,
        None => {
            return Err("Expected a non-negative integer column coordinate value.".to_string());
        }
    };
    let dst_loc = Loc::new(dst_row, dst_col);
    dst.add_item(src_item, dst_loc)?;

    Ok(())
}

fn interact(store: &mut Pack, pack: &mut Pack) -> bool {
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
        "showstore" => {
            println!("{}", store);
            return true;
        }
        "showpack" => {
            println!("{}", pack);
            return true;
        }
        "pack" => {
            let result = move_between(words, store, pack);
            if result.is_err() {
                println!("{}", result.err().unwrap());
            }
            return true;
        }
        "store" => {
            let result = move_between(words, pack, store);
            if result.is_err() {
                println!("{}", result.err().unwrap());
            }
            return true;
        }
        "packtranspose" => {
            let src_row = match words.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(row) => row,
                None => {
                    println!("Expected a non-negative integer row coordinate value.");
                    return true;
                }
            };

            let src_col = match words.next().and_then(|s| s.parse::<u32>().ok()) {
                Some(col) => col,
                None => {
                    println!("Expected a non-negative integer column coordinate value.");
                    return true;
                }
            };
            let src_loc = Loc::new(src_row, src_col);
            let result = pack.transpose_item_at(src_loc);
            if result.is_err() {
                println!("{}", result.err().unwrap());
            }
            return true;
        }
        _ => {
            println!("Unknown command");
        }
    }
    return true;
}

fn get_user_store() -> Result<Pack, String> {
    let mut store = Pack::new(10, 10);

    let stone = Item::new("stone0", 2, 2, '*');
    store.add_item(stone, Loc::new(0, 0))?;

    let torch = Item::new("torch0", 1, 3, '&');
    store.add_item(torch, Loc::new(3, 2))?;

    let matches = Item::new("matches0", 3, 2, '!');
    store.add_item(matches, Loc::new(5, 5))?;

    let hatchet = Item::new("hatchet0", 2, 5, '<');
    store.add_item(hatchet, Loc::new(8, 0))?;

    Ok(store)
}

fn main() {
    let mut store = get_user_store().unwrap();
    let mut pack = Pack::new(10, 10);
    loop {
        print!(">>> ");
        stdout().flush().unwrap();

        let decision = interact(&mut store, &mut pack);
        if decision == false {
            break;
        }
    }
}
