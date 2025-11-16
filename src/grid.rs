use std::collections::{HashMap, HashSet};

// Position represents a cell coordinate in the grid
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position(pub i32, pub i32);

impl Position {
    #[inline]
    pub fn offset(self, dx: i32, dy: i32) -> Self {
        Position(self.0 + dx, self.1 + dy)
    }

    #[inline]
    pub fn new(x: i32, y: i32) -> Self {
        Position(x, y)
    }

    #[inline]
    pub fn x(&self) -> i32 {
        self.0
    }

    #[inline]
    pub fn y(&self) -> i32 {
        self.1
    }
}

// 8 neighbor offsets (constant to avoid tuple creation)
pub const NEIGHBOR_OFFSETS: [(i32, i32); 8] = [
    (-1, -1), (0, -1), (1, -1),
    (-1,  0),          (1,  0),
    (-1,  1), (0,  1), (1,  1),
];

// Grid utilities for the Game of Life simulation
pub struct Grid {
    pub width: i32,
    pub height: i32,
    pub wrap_world: bool,
}

impl Grid {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            width,
            height,
            wrap_world: false,
        }
    }

    #[inline]
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        (0..self.width).contains(&x) && (0..self.height).contains(&y)
    }

    #[inline]
    pub fn wrap(&self, x: i32, y: i32) -> Position {
        let mut nx = x % self.width;
        let mut ny = y % self.height;
        if nx < 0 { nx += self.width; }
        if ny < 0 { ny += self.height; }
        Position(nx, ny)
    }

    /// Calculate next generation based on current live cells
    /// Count neighbors with HashMap once -> apply rules
    pub fn next_generation(&self, live: &HashSet<Position>) -> HashSet<Position> {
        let mut counts: HashMap<Position, u8> = HashMap::with_capacity(live.len() * 8 + 8);

        // Count neighbors of all live cells (distribute to 8 directions)
        for &cell in live {
            for (dx, dy) in NEIGHBOR_OFFSETS {
                let p = if self.wrap_world {
                    self.wrap(cell.0 + dx, cell.1 + dy)
                } else {
                    cell.offset(dx, dy)
                };
                if self.wrap_world || self.in_bounds(p.0, p.1) {
                    *counts.entry(p).or_insert(0) += 1;
                }
            }
        }

        // Create new set from count: 3 -> birth, 2 && alive -> survive
        let mut next = HashSet::with_capacity(live.len());
        for (pos, n) in counts {
            let alive = live.contains(&pos);
            if n == 3 || (alive && n == 2) {
                next.insert(pos);
            }
        }

        next
    }
}