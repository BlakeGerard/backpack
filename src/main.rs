use backpack::*;

fn main() {
    // Initialize the Pack
    let mut pack = Pack::new(5, 5);
    println!("{}", pack);

    // Add a 'stick'
    let stick = Item::new(1, 3, '.');
    let result = pack.add_item(stick, (1, 2));
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
    println!("{}", pack);

    // Add a 'rock'
    let rock = Item::new(2, 2, '@');
    let result = pack.add_item(rock, (2, 2));
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
    println!("{}", pack);

    // transpose the stick, this will result in an error.
    let result = pack.transpose_item((1, 2));
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
    println!("{}", pack);

    // Move the stick away from the rock.
    let result = pack.move_item((1, 2), (0, 0));
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
    println!("{}", pack);

    // transpose the stick, this will result in an error.
    let result = pack.transpose_item((0, 0));
    if result.is_err() {
        println!("{}", result.err().unwrap());
    }
    println!("{}", pack);
}
