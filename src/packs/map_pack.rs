use crate::items::{Item, Loc, PackedItem};

use std::collections::BTreeMap;

#[derive(Debug)]
pub struct MapPack {
    rows: u32,
    cols: u32,
    items: BTreeMap<Loc, PackedItem>,
}

impl MapPack {
    pub fn new(rows: u32, cols: u32) -> Self {
        let mut rows = rows;
        let mut cols = cols;
        if rows == 0 {
            rows = 1;
        }
        if cols == 0 {
            cols = 1;
        }
        MapPack {
            rows,
            cols,
            items: BTreeMap::new(),
        }
    }

    fn grab_item(&self, loc: &Loc) -> Option<&PackedItem> {
        self.items.get(&loc)
    }

    fn find_item(&self, loc: &Loc) -> Option<&PackedItem> {
        for packed_item in self.items.values() {
            if packed_item.contains(loc) {
                return Some(packed_item);
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
        for packed_item in self.items.values() {
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
        let tentative = PackedItem::new(loc, item);

        // Invalid loc for this Pack.
        if self.item_placement_exceeds_bounds(&tentative) {
            return Err("Item placement is invalid".to_string());
        }

        // Item overlaps other items.
        if self.item_placement_intersects_contents(&tentative) {
            return Err("Item intersects existing Pack contents.".to_string());
        }

        let result_loc = tentative.loc();
        self.items.insert(result_loc.clone(), tentative);
        Ok(result_loc)
    }

    pub fn remove_item(&mut self, loc: Loc) -> Option<PackedItem> {
        self.items.remove(&loc)
    }

    pub fn transpose_item(&mut self, loc: Loc) -> Result<Loc, String> {
        // Do the tranposition.
        match self.items.get_mut(&loc) {
            Some(item) => {
                item.transpose();
            }
            None => {
                return Err("Invalid location".to_string());
            }
        }

        // Undo the transposition if placement is invalid.
        if self.item_placement_is_invalid(&self.items[&loc]) {
            self.items.get_mut(&loc).unwrap().transpose();
            return Err("Invalid transposition.".to_string());
        }
        Ok(loc)
    }

    pub fn move_item(&mut self, src: Loc, dst: Loc) -> Result<Loc, String> {
        // Do the move.
        match self.items.get_mut(&src) {
            Some(item) => {
                item.move_to(dst.clone());
            }
            None => {
                return Err("Invalid loc".to_string());
            }
        }

        // Undo the move if placement is invalid.
        if self.item_placement_is_invalid(&self.items[&src]) {
            self.items.get_mut(&src).unwrap().move_to(src);
            return Err("Invalid move".to_string());
        }

        let item = self.items.remove(&src).unwrap();
        self.items.insert(dst.clone(), item);
        Ok(dst)
    }
}

use std::fmt;
impl fmt::Display for MapPack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        const SEP: char = '|';
        for r in 0..self.rows {
            for c in 0..self.cols {
                let mut next_symbol: char = ' ';
                if let Some(packed_item) = self.find_item(&Loc::new(r, c)) {
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
        let mut pack = MapPack::new(1, 1);
        let pebble = Item::new(1, 1, '.');

        let result = pack.add_item(pebble, Loc::new(0, 0));
        assert!(result.is_ok());
    }

    #[test]
    fn add_item_with_out_of_bounds_coordinates_is_an_error() {
        let mut pack = MapPack::new(1, 1);
        let pebble = Item::new(1, 1, '.');

        let result = pack.add_item(pebble, Loc::new(1, 0));
        assert!(result.is_err());
    }

    #[test]
    fn add_item_intersecting_existing_item_is_an_error() {
        let mut pack = MapPack::new(3, 3);
        let stick = Item::new(1, 2, '*');
        let stone = Item::new(2, 2, '@');

        let result = pack.add_item(stick, Loc::new(0, 0));
        assert!(result.is_ok());

        let result = pack.add_item(stone, Loc::new(0, 0));
        assert!(result.is_err());
    }

    #[test]
    fn add_item_that_exceeds_pack_size_is_an_error() {
        let mut pack = MapPack::new(1, 1);
        let cat = Item::new(3, 2, 'c');

        let result = pack.add_item(cat, Loc::new(0, 0));
        assert!(result.is_err());
    }

    #[test]
    fn tranpose_item_valid_transposition_succeeds() {
        let mut pack = MapPack::new(3, 3);
        let stick = Item::new(1, 3, '*');

        let result = pack.add_item(stick, Loc::new(0, 0));
        assert!(result.is_ok());

        let result = pack.transpose_item(result.unwrap());
        assert!(result.is_ok());
    }

    #[test]
    fn transpose_item_causing_intersection_is_an_error() {
        let mut pack = MapPack::new(3, 3);
        let stick = Item::new(1, 3, '*');
        let stone = Item::new(1, 1, '@');

        let add_stick = pack.add_item(stick, Loc::new(0, 0));
        assert!(add_stick.is_ok());

        let add_stone = pack.add_item(stone, Loc::new(1, 0));
        assert!(add_stone.is_ok());

        let result = pack.transpose_item(add_stick.unwrap());
        assert!(result.is_err());
    }

    #[test]
    fn remove_item_from_unoccupied_space_returns_none() {
        let mut pack = MapPack::new(3, 3);
        let removed = pack.remove_item(Loc::new(0, 0));
        assert!(removed.is_none());
    }

    #[test]
    fn remove_item_at_occupied_location_returns_some_packed_item() {
        let mut pack = MapPack::new(3, 3);
        let stone = Item::new(1, 1, '*');

        let result = pack.add_item(stone.clone(), Loc::new(0, 0));
        assert!(result.is_ok());

        let removed = pack.remove_item(result.unwrap());
        assert!(matches!(removed, stone));
    }

    #[test]
    fn move_item_to_out_of_bounds_location_is_an_error() {
        let mut pack = MapPack::new(1, 1);
        let stone = Item::new(1, 1, '*');

        let result = pack.add_item(stone, Loc::new(0, 0));
        assert!(result.is_ok());

        let result = pack.move_item(result.unwrap(), Loc::new(5, 5));
        assert!(result.is_err());
    }

    #[test]
    fn move_item_is_correct() {
        let mut pack = MapPack::new(1, 2);
        let a = Item::new(1, 1, '^');

        let result = pack.add_item(a, Loc::new(0, 0));
        assert!(result.is_ok());
    }
}
