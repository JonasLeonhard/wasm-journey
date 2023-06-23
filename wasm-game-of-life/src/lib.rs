use core::fmt;

use wasm_bindgen::prelude::*;
extern crate js_sys;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new(width: u32, height: u32) -> Universe {
        let cells = (0..width * height)
            .map(|_i| {
                if js_sys::Math::random() < 0.5 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn set_alive(&mut self, row: u32, column: u32) {
        let cell_index = self.get_index(row, column);
        self.cells[cell_index] = Cell::Alive;
    }

    pub fn clear(&mut self) {
        self.cells = (0..self.width * self.height).map(|_i| Cell::Dead).collect();
    }

    /**
     * create a 3x3 grid around the cell (0 here):
     * |  |-1|  |
     * |-1| 0| 1|
     * |  | 1|  |
     *
     * instead of starting at -1, we start with self.width - 1
     * we use %modulo self.height at the line neighbour_row to wrap around.
     * This avoids if edgecases when a value would wrap around the grid
     *
     * example for a 4x4 universe, where we index row = 4, col = 1 (0 here):
     *
     * |  |  |  |n1|
     * |  |  |  | 0|
     * |  |  |  |  |
     * |  |  |  |  |
     *
     * we start by getting all delta rows. that would equal to
     * delta_row = [3, 0, 1]
     * delta_col = [3, 0, 1]
     *
     * lets calculate the first neighbour:
     * let neighbour_row = (row + delta_row) % self.height;
     * let neighbour_row = (4 + 3) % 4 = 3;
     * let neighbour_col = (col + delta_col) % self.width;
     * let neighbour_col = (1 + 3) % 4 = 0;
     *
     * we can now index n1 at (3, 0) and increase the alive count by its Cell value;
     */
    fn live_neighbour_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 { self.height - 1 } else { row - 1 };

        let south = if row == self.height - 1 { 0 } else { row + 1 };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    pub fn tick(&mut self) {
        let mut next_universe = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let cell_index = self.get_index(row, col);
                let cell = self.cells[cell_index];
                let live_neighbors = self.live_neighbour_count(row, col);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, neighbours) if neighbours < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, neighbours) if neighbours > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next_universe[cell_index] = next_cell;
            }
        }

        self.cells = next_universe;
    }
}

impl fmt::Display for Universe {
    /**
     * implements the to_string() method
     */
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{symbol}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}
