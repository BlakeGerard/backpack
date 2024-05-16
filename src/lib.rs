#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Item {
    rows: u32,
    cols: u32,
    symbol: char,
}

impl Item {
    pub fn new(rows: u32, cols: u32, symbol: char) -> Self {
        let mut rows = rows;
        let mut cols = cols;
        if rows == 0 {
            rows = 1;
        }
        if cols == 0 {
            cols = 1;
        }
        Item { rows, cols, symbol }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackedItem {
    r: u32,
    c: u32,
    item: Item,
}

impl PackedItem {
    pub fn new(r: u32, c: u32, item: Item) -> Self {
        Self { r, c, item }
    }

    pub fn symbol(&self) -> char {
        self.item.symbol
    }

    pub fn intersects(&self, other: &Self) -> bool {
        if
        // self is entirely to the left of other.
        self.c + self.item.cols <= other.c
        // self is entirely to the right of other.
        || self.c >= other.c + other.item.cols
        // self is entirely above other
        || self.r + self.item.rows <= other.r
        // self is entirely below other
        || self.r >= other.r + other.item.rows
        {
            return false;
        }
        true
    }

    pub fn contains(&self, loc: (u32, u32)) -> bool {
        if self.r <= loc.0
            && loc.0 < self.r + self.item.rows
            && self.c <= loc.1
            && loc.1 < self.c + self.item.cols
        {
            return true;
        }
        false
    }

    pub fn transpose(&mut self) {
        std::mem::swap(&mut self.item.rows, &mut self.item.cols);
    }

    pub fn move_to(&mut self, dst: (u32, u32)) {
        self.r = dst.0;
        self.c = dst.1;
    }
}

#[derive(Debug)]
pub struct Pack {
    rows: u32,
    cols: u32,
    items: Vec<PackedItem>,
}

impl Pack {
    pub fn new(rows: u32, cols: u32) -> Self {
        let mut rows = rows;
        let mut cols = cols;
        if rows == 0 {
            rows = 1;
        }
        if cols == 0 {
            cols = 1;
        }
        Pack {
            rows,
            cols,
            items: Vec::new(),
        }
    }

    fn grab_item(&self, loc: (u32, u32)) -> Option<&PackedItem> {
        for packed_item in &self.items {
            if packed_item.contains(loc) {
                return Some(packed_item);
            }
        }
        None
    }

    fn grab_item_index(&self, loc: (u32, u32)) -> Option<usize> {
        for (idx, packed_item) in self.items.iter().enumerate() {
            if packed_item.contains(loc) {
                return Some(idx);
            }
        }
        None
    }

    fn item_placement_exceeds_bounds(&self, item: &PackedItem) -> bool {
        return item.r >= self.rows
            || item.c >= self.cols
            || item.r + item.item.rows > self.rows
            || item.c + item.item.cols > self.cols;
    }

    fn item_placement_intersects_contents(&self, item: &PackedItem) -> bool {
        for packed_item in &self.items {
            if item == packed_item {
                continue;
            }
            if item.intersects(&packed_item) {
                return true;
            }
        }
        false
    }

    fn item_placement_is_invalid(&self, item: &PackedItem) -> bool {
        return self.item_placement_exceeds_bounds(item)
            || self.item_placement_intersects_contents(item);
    }

    pub fn add_item(&mut self, item: Item, loc: (u32, u32)) -> Result<(), String> {
        let tentative = PackedItem::new(loc.0, loc.1, item);

        // Invalid loc for this Pack.
        if self.item_placement_exceeds_bounds(&tentative) {
            return Err("Item placement is invalid".to_string());
        }

        // Item overlaps other items.
        if self.item_placement_intersects_contents(&tentative) {
            return Err("Item intersects existing Pack contents.".to_string());
        }

        self.items.push(tentative);
        Ok(())
    }

    pub fn remove_item(&mut self, loc: (u32, u32)) -> Option<PackedItem> {
        if let Some(idx) = self.grab_item_index(loc) {
            return Some(self.items.swap_remove(idx));
        }
        None
    }

    pub fn transpose_item(&mut self, loc: (u32, u32)) -> Result<(), String> {
        if let Some(idx) = self.grab_item_index(loc) {
            // Do the tranposition.
            self.items[idx].transpose();

            // Undo the transposition if placement is invalid.
            if self.item_placement_is_invalid(&self.items[idx]) {
                self.items[idx].transpose();
                return Err("Invalid transposition.".to_string());
            }
        }
        Ok(())
    }

    pub fn move_item(&mut self, src: (u32, u32), dst: (u32, u32)) -> Result<(), String> {
        if let Some(idx) = self.grab_item_index(src) {
            // Do the move.
            self.items[idx].move_to(dst);

            // Undo the move if placement is invalid.
            if self.item_placement_is_invalid(&self.items[idx]) {
                self.items[idx].move_to(src);
                return Err("Invalid move".to_string());
            }
        }
        Ok(())
    }
}

use std::fmt;
impl fmt::Display for Pack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for r in 0..self.rows {
            for c in 0..self.cols {
                let mut next_symbol: char = ' ';
                if let Some(packed_item) = self.grab_item((r, c)) {
                    next_symbol = packed_item.symbol();
                }
                write!(f, "|{}", next_symbol)?;
            }
            write!(f, "|\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_1x1_item_to_1x1_pack() {
        let mut pack = Pack::new(1, 1);
        let pebble = Item::new(1, 1, '.');

        let result = pack.add_item(pebble, (0, 0));
        assert!(result.is_ok());
    }

    #[test]
    fn add_item_with_out_of_bounds_coordinates_is_an_error() {
        let mut pack = Pack::new(1, 1);
        let pebble = Item::new(1, 1, '.');

        let result = pack.add_item(pebble, (1, 0));
        assert!(result.is_err());
    }

    #[test]
    fn add_item_intersecting_existing_item_is_an_error() {
        let mut pack = Pack::new(3, 3);
        let stick = Item::new(1, 2, '*');
        let stone = Item::new(2, 2, '@');

        let result = pack.add_item(stick, (0, 0));
        assert!(result.is_ok());

        let result = pack.add_item(stone, (0, 0));
        assert!(result.is_err());
    }

    #[test]
    fn add_item_that_exceeds_pack_size_is_an_error() {
        let mut pack = Pack::new(1, 1);
        let cat = Item::new(3, 2, 'c');

        let result = pack.add_item(cat, (0, 0));
        assert!(result.is_err());
    }

    #[test]
    fn tranpose_item_valid_transposition_succeeds() {
        let mut pack = Pack::new(3, 3);
        let stick = Item::new(1, 3, '*');

        let result = pack.add_item(stick, (0, 0));
        assert!(result.is_ok());

        let result = pack.transpose_item((0, 0));
        assert!(result.is_ok());
    }

    #[test]
    fn transpose_item_causing_intersection_is_an_error() {
        let mut pack = Pack::new(3, 3);
        let stick = Item::new(1, 3, '*');
        let stone = Item::new(1, 1, '@');

        let result = pack.add_item(stick, (0, 0));
        assert!(result.is_ok());

        let result = pack.add_item(stone, (1, 0));
        assert!(result.is_ok());

        let result = pack.transpose_item((0, 0));
        assert!(result.is_err());
    }

    #[test]
    fn remove_item_from_unoccupied_space_returns_none() {
        let mut pack = Pack::new(3, 3);
        let stone = Item::new(1, 1, '*');

        let result = pack.add_item(stone, (0, 0));
        assert!(result.is_ok());

        let removed = pack.remove_item((1, 1));
        assert!(removed.is_none());
    }

    #[test]
    fn remove_item_at_occupied_location_returns_some_packed_item() {
        let mut pack = Pack::new(3, 3);
        let stone = Item::new(1, 1, '*');

        let result = pack.add_item(stone.clone(), (0, 0));
        assert!(result.is_ok());

        let _expected = Some(PackedItem {
            r: 0,
            c: 0,
            item: Item {
                rows: 1,
                cols: 1,
                symbol: '*',
            },
        });
        let removed = pack.remove_item((0, 0));

        assert!(matches!(removed, _expected));
    }

    #[test]
    fn move_item_to_out_of_bounds_location_is_an_error() {
        let mut pack = Pack::new(1, 1);
        let stone = Item::new(1, 1, '*');

        let result = pack.add_item(stone.clone(), (0, 0));
        assert!(result.is_ok());

        let result = pack.move_item((0, 0), (5, 5));
        assert!(result.is_err());
    }
}
