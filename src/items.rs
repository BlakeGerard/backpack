#[derive(Debug, Copy, Clone, Eq, PartialEq)]
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
        Item {
            rows: rows,
            cols: cols,
            symbol: symbol,
        }
    }

    pub fn rows(&self) -> u32 {
        self.rows
    }

    pub fn cols(&self) -> u32 {
        self.cols
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }
}

#[derive(Debug, Clone, Eq, PartialEq, PartialOrd, Ord, Copy)]
pub struct Loc {
    row: u32,
    col: u32,
}

impl Loc {
    pub fn new(row: u32, col: u32) -> Self {
        Loc { row, col }
    }

    pub fn row(&self) -> u32 {
        self.row
    }

    pub fn col(&self) -> u32 {
        self.col
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PackedItem {
    loc: Loc,
    item: Item,
}

impl PackedItem {
    pub fn new(loc: Loc, item: Item) -> Self {
        Self { loc, item }
    }

    pub fn loc(&self) -> Loc {
        self.loc.clone()
    }

    pub fn item(&self) -> Item {
        self.item
    }

    pub fn row(&self) -> u32 {
        self.loc.row
    }

    pub fn col(&self) -> u32 {
        self.loc.col
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
        self.col() + self.cols() <= other.col()
        // self is entirely to the right of other.
        || self.col() >= other.col() + other.cols()
        // self is entirely above other
        || self.row() + self.rows() <= other.row()
        // self is entirely below other
        || self.row() >= other.row() + other.rows()
        {
            return false;
        }
        true
    }

    pub fn contains(&self, loc: &Loc) -> bool {
        if self.row() <= loc.row
            && loc.row < self.row() + self.rows()
            && self.col() <= loc.col
            && loc.col < self.col() + self.cols()
        {
            return true;
        }
        false
    }

    pub fn transpose(&mut self) {
        std::mem::swap(&mut self.item.rows, &mut self.item.cols);
    }

    pub fn move_to(&mut self, dst: Loc) {
        self.loc = dst;
    }
}
