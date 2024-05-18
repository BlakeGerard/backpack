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

    pub fn r(&self) -> u32 {
        self.r
    }

    pub fn c(&self) -> u32 {
        self.c
    }

    pub fn rows(&self) -> u32 {
        self.item.rows
    }

    pub fn cols(&self) -> u32 {
        self.item.cols
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
