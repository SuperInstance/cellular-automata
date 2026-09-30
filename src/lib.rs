//! Generic cellular automaton simulator on a 2D grid.

pub type RuleFn<T> = fn(neighbors: &[Option<T>], current: &T) -> T;

#[derive(Clone, Debug)]
pub struct Automaton<T: Clone> {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<Vec<T>>,
    rule: RuleFn<T>,
    neighborhood: Neighborhood,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Neighborhood {
    Moore,      // 8 surrounding cells
    VonNeumann, // 4 cardinal cells
}

impl<T: Clone + Default + PartialEq> Automaton<T> {
    pub fn new(width: usize, height: usize, rule: RuleFn<T>, neighborhood: Neighborhood) -> Self {
        let grid = vec![vec![T::default(); width]; height];
        Self { width, height, grid, rule, neighborhood }
    }

    pub fn set(&mut self, x: usize, y: usize, value: T) {
        if y < self.height && x < self.width {
            self.grid[y][x] = value;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&T> {
        self.grid.get(y).and_then(|row| row.get(x))
    }

    pub fn step(&mut self) {
        let old = self.grid.clone();
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.collect_neighbors(&old, x, y);
                self.grid[y][x] = (self.rule)(&neighbors, &old[y][x]);
            }
        }
    }

    pub fn step_n(&mut self, steps: usize) {
        for _ in 0..steps { self.step(); }
    }

    fn collect_neighbors(&self, grid: &[Vec<T>], x: usize, y: usize) -> Vec<Option<T>> {
        let offsets: &[(i32, i32)] = match self.neighborhood {
            Neighborhood::Moore => &[(-1,-1),(0,-1),(1,-1),(-1,0),(1,0),(-1,1),(0,1),(1,1)],
            Neighborhood::VonNeumann => &[(0,-1),(-1,0),(1,0),(0,1)],
        };
        offsets.iter().map(|&(dx, dy)| {
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx >= 0 && ny >= 0 {
                grid.get(ny as usize)
                    .and_then(|row| row.get(nx as usize))
                    .cloned()
            } else {
                None
            }
        }).collect()
    }

    pub fn count_alive(&self) -> usize where T: PartialEq {
        let dead = T::default();
        self.grid.iter().flat_map(|row| row.iter()).filter(|c| **c != dead).count()
    }
}

/// Classic Conway's Game of Life rule for bool cells.
pub fn conway_rule(neighbors: &[Option<bool>], current: &bool) -> bool {
    let alive_count = neighbors.iter().filter(|n| n.unwrap_or(false)).count();
    match (current, alive_count) {
        (true, 2) | (true, 3) => true,
        (false, 3) => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blinker() {
        let mut auto = Automaton::new(5, 5, conway_rule, Neighborhood::Moore);
        // Vertical blinker
        auto.set(2, 1, true);
        auto.set(2, 2, true);
        auto.set(2, 3, true);
        auto.step();
        // Should become horizontal
        assert_eq!(*auto.get(1, 2).unwrap(), true);
        assert_eq!(*auto.get(3, 2).unwrap(), true);
        assert_eq!(*auto.get(2, 2).unwrap(), true);
    }
}

/// FNV-1a 64 — the digest every substrate in the SuperInstance fleet agrees on.
pub const FNV_OFFSET: u64 = 0xcbf29ce484222325;
pub const FNV_PRIME: u64 = 0x100000001b3;

#[inline]
pub fn fnv1a64(bytes: &[u8]) -> u64 {
    let mut h = FNV_OFFSET;
    for &b in bytes {
        h = (h ^ b as u64).wrapping_mul(FNV_PRIME);
    }
    h
}

/// True if this crate's FNV-1a still agrees with the rest of the fleet.
pub fn canary_holds() -> bool {
    fnv1a64("café Δ 日本語".as_bytes()) == 0x024a555471370b18d
}
