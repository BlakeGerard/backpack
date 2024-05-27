use crate::items::{Item, Loc, PackedItem};

#[derive(Debug)]
pub struct DensePack {
    rows: u32,
    cols: u32,
    items: Vec<PackedItem>,
}

impl DensePack {
    pub fn new(rows: u32, cols: u32) -> Self {
        let mut rows = rows;
        let mut cols = cols;
        if rows == 0 {
            rows = 1;
        }
        if cols == 0 {
            cols = 1;
        }
        DensePack {
            rows,
            cols,
            items: Vec::new(),
        }
    }

    fn grab_item(&self, loc: &Loc) -> Option<&PackedItem> {
        for packed_item in &self.items {
            if packed_item.contains(loc) {
                return Some(packed_item);
            }
        }
        None
    }

    fn grab_item_index(&self, name: &str) -> Option<usize> {
        for (idx, packed_item) in self.items.iter().enumerate() {
            if packed_item.name() == name {
                return Some(idx);
            }
        }
        None
    }

    fn grab_item_index_at(&self, loc: &Loc) -> Option<usize> {
        for (idx, packed_item) in self.items.iter().enumerate() {
            if packed_item.contains(loc) {
                return Some(idx);
            }
        }
        None
    }

    fn item_placement_exceeds_bounds(&self, item: &PackedItem) -> bool {
        return item.row() >= self.rows
            || item.col() >= self.cols
            || item.row() + item.rows() > self.rows
            || item.col() + item.cols() > self.cols;
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

    pub fn add_item(&mut self, item: Item, loc: Loc) -> Result<Loc, String> {
        let tentative = PackedItem::new(loc.clone(), item);

        // Invalid loc for this Pack.
        if self.item_placement_is_invalid(&tentative) {
            return Err("Invalid item placement".to_string());
        }

        self.items.push(tentative);
        Ok(loc)
    }

    pub fn remove_item(&mut self, name: &str) -> Option<PackedItem> {
        if let Some(idx) = self.grab_item_index(name) {
            return Some(self.items.swap_remove(idx));
        }
        None
    }

    pub fn remove_item_at(&mut self, loc: Loc) -> Option<PackedItem> {
        if let Some(idx) = self.grab_item_index_at(&loc) {
            return Some(self.items.swap_remove(idx));
        }
        None
    }

    pub fn transpose_item(&mut self, name: &str) -> Result<Loc, String> {
        if let Some(idx) = self.grab_item_index(&name) {
            // Do the tranposition.
            self.items[idx].transpose();

            // Undo the transposition if placement is invalid.
            if self.item_placement_is_invalid(&self.items[idx]) {
                self.items[idx].transpose();
                return Err("Invalid transposition.".to_string());
            }
            return Ok(self.items[idx].loc());
        }
        Err("No item at the given location".to_string())
    }

    pub fn transpose_item_at(&mut self, loc: Loc) -> Result<Loc, String> {
        if let Some(idx) = self.grab_item_index_at(&loc) {
            // Do the tranposition.
            self.items[idx].transpose();

            // Undo the transposition if placement is invalid.
            if self.item_placement_is_invalid(&self.items[idx]) {
                self.items[idx].transpose();
                return Err("Invalid transposition.".to_string());
            }
            return Ok(self.items[idx].loc());
        }
        Err("No item at the given location".to_string())
    }

    pub fn move_item(&mut self, name: &str, dst: Loc) -> Result<Loc, String> {
        if let Some(idx) = self.grab_item_index(name) {
            let src = self.items[idx].loc();

            // Do the move.
            self.items[idx].move_to(dst);

            // Undo the move if placement is invalid.
            if self.item_placement_is_invalid(&self.items[idx]) {
                self.items[idx].move_to(src);
                return Err("Invalid move".to_string());
            }
            return Ok(src);
        }
        Err("No item at the given location".to_string())
    }

    pub fn move_item_at(&mut self, src: Loc, dst: Loc) -> Result<Loc, String> {
        if let Some(idx) = self.grab_item_index_at(&src) {
            // Do the move.
            self.items[idx].move_to(dst);

            // Undo the move if placement is invalid.
            if self.item_placement_is_invalid(&self.items[idx]) {
                self.items[idx].move_to(src);
                return Err("Invalid move".to_string());
            }
            return Ok(self.items[idx].loc());
        }
        Err("No item at the given location".to_string())
    }
}

use std::fmt;
impl fmt::Display for DensePack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const SEP: char = '|';
        for r in 0..self.rows {
            for c in 0..self.cols {
                let mut next_symbol: char = ' ';
                if let Some(packed_item) = self.grab_item(&Loc::new(r, c)) {
                    next_symbol = packed_item.symbol();
                }
                write!(f, "{}{}", SEP, next_symbol)?;
            }
            write!(f, "{}\n", SEP)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_1x1_item_to_1x1_pack() {
        let mut pack = DensePack::new(1, 1);
        let pebble = Item::new("pebble", 1, 1, '.');

        let result = pack.add_item(pebble, Loc::new(0, 0));
        assert!(result.is_ok());
    }

    #[test]
    fn add_item_with_out_of_bounds_coordinates_is_an_error() {
        let mut pack = DensePack::new(1, 1);
        let pebble = Item::new("pebble", 1, 1, '.');

        let result = pack.add_item(pebble, Loc::new(1, 0));
        assert!(result.is_err());
    }

    #[test]
    fn add_item_intersecting_existing_item_is_an_error() {
        let mut pack = DensePack::new(3, 3);
        let stick = Item::new("stick", 1, 2, '*');
        let stone = Item::new("stone", 2, 2, '@');

        let result = pack.add_item(stick, Loc::new(0, 0));
        assert!(result.is_ok());

        let result = pack.add_item(stone, Loc::new(0, 0));
        assert!(result.is_err());
    }

    #[test]
    fn add_item_that_exceeds_pack_size_is_an_error() {
        let mut pack = DensePack::new(1, 1);
        let cat = Item::new("cat", 3, 2, 'c');

        let result = pack.add_item(cat, Loc::new(0, 0));
        assert!(result.is_err());
    }

    #[test]
    fn tranpose_item_valid_transposition_succeeds() {
        let mut pack = DensePack::new(3, 3);
        let stick = Item::new("stick", 1, 3, '*');

        let result = pack.add_item(stick, Loc::new(0, 0));
        assert!(result.is_ok());

        let result = pack.transpose_item_at(result.unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn transpose_item_causing_intersection_is_an_error() {
        let mut pack = DensePack::new(3, 3);
        let stick = Item::new("stick", 1, 3, '*');
        let stone = Item::new("stone", 1, 1, '@');

        let add_stick = pack.add_item(stick, Loc::new(0, 0));
        assert!(add_stick.is_ok());

        let add_stone = pack.add_item(stone, Loc::new(1, 0));
        assert!(add_stone.is_ok());

        let result = pack.transpose_item_at(add_stick.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn remove_item_from_unoccupied_space_returns_none() {
        let mut pack = DensePack::new(3, 3);
        let removed = pack.remove_item_at(Loc::new(0, 0));
        assert!(removed.is_none());
    }

    #[test]
    fn remove_item_at_occupied_location_returns_some_packed_item() {
        let mut pack = DensePack::new(3, 3);
        let stone = Item::new("stone", 1, 1, '*');

        let result = pack.add_item(stone.clone(), Loc::new(0, 0));
        assert!(result.is_ok());

        let removed = pack.remove_item_at(result.unwrap());
        assert!(matches!(removed, stone));
    }

    #[test]
    fn move_item_to_out_of_bounds_location_is_an_error() {
        let mut pack = DensePack::new(1, 1);
        let stone = Item::new("stone", 1, 1, '*');

        let result = pack.add_item(stone, Loc::new(0, 0));
        assert!(result.is_ok());

        let result = pack.move_item_at(result.unwrap(), Loc::new(5, 5));
        assert!(result.is_err());
    }
}
