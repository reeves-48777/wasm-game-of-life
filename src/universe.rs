use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;
use js_sys::Math;

use super::spaceships::CommonSpaceships;
use super::utils;

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet,
    back_buffer: FixedBitSet,
}

impl Universe {

    /// Get the index of a cell in the universe, given a row and a column
    fn get_index(&self, row :u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Count the number of living neighbours of a cell in the universe
    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height -1
        } else {
            row - 1
        };

        let south = if row == self.height + 1 {
            0
        } else {
            row + 1
        };

        let west = if col == 0 {
            self.width - 1
        } else {
            col - 1
        };

        let east = if col == self.width + 1 {
            0
        } else {
            col + 1
        };

        let nw = self.get_index(north, west);
        if self.cells.contains(nw) {
            count += 1;
        }
    
        let n = self.get_index(north, col);
        if self.cells.contains(n) {
            count += 1;
        }

        let ne = self.get_index(north, east);
        if self.cells.contains(ne) {
            count += 1;
        }

        let w = self.get_index(row, west);
        if self.cells.contains(w) {
            count += 1;
        }

        let e = self.get_index(row, east);
        if self.cells.contains(e) {
            count += 1;
        }

        let sw = self.get_index(south, west);
        if self.cells.contains(sw) {
            count += 1;
        }

        let s = self.get_index(south, col);
        if self.cells.contains(s) {
            count += 1;
        }

        let se = self.get_index(south, east);
        if self.cells.contains(se) {
            count += 1;
        }
        
        count
    }

    /// Get the dead an alive values of the entire universe
    pub fn get_cells(&self) -> Vec<bool> {
        let mut cells = Vec::new();
        (0..self.width * self.height).for_each(|i| cells.push(self.cells.contains(i as usize)));
        cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells.set(idx, true);
        }
    }
}

// public methods exported to js
#[wasm_bindgen]
impl Universe {

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 128;
        let height = 128;

        let size = (width * height) as usize;
        let cells = FixedBitSet::with_capacity(size);
        let back_buffer = FixedBitSet::with_capacity(size);

        Universe {
            width,
            height,
            cells,
            back_buffer
        }
    }

    /// Calculate the next state of the universe
    pub fn tick(&mut self) {
        //let _timer = Timer::new("Universe::tick");
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                self.back_buffer.set(idx, match (cell, live_neighbours) {
                    // Rule 1 : Any live cell with fewer than two lives neighbour dies, underpopulation
                    (true, x) if x < 2 => {
                        false
                    },
                    // Rule 2 : Any live cell with two or three live neighbours lives
                    (true, 2) | (true, 3) => true,
                    // Rule 3 : Any live cell with more than three neighbours live dies, overpopulation
                    (true, x) if x > 3 => {
                        false
                    },
                    // Rule 4 : Anu dead cell with exactly three live neighbours becomes alive
                    (false, 3) => {
                        true
                    },
                    // all the others remain in the same state
                    (other, _) => other
                });

            }
        }
        std::mem::swap(&mut self.cells, &mut self.back_buffer);   
    }


    /// Add a spaceship in the universe
    /// 
    /// Place a given type spaceship at a given position of the universe
    pub fn add_spaceship(&mut self, spaceship: CommonSpaceships, start_x: usize, start_y: usize) {
        let (pattern, pat_width, pat_height) = spaceship.pattern();

        for row in 0..pat_height {
            for col in 0..pat_width {
                let pattern_index = row * pat_width + col;
                let universe_index = (start_y + row) * self.width as usize + (start_x + col);

                if universe_index < self.back_buffer.len() {
                    if !self.cells.contains(universe_index) {
                        self.cells.set(universe_index, pattern[pattern_index]);
                        self.back_buffer.set(universe_index, pattern[pattern_index]);
                    }
                    self.cells.set(universe_index, pattern[pattern_index]);
                    self.back_buffer.set(universe_index, pattern[pattern_index]);
                }
            }
        }
    }

    /// Toggle the state of a cell in the universe, given its row and column
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
        self.back_buffer.toggle(idx);
    }

    /// Set the width of the Universe
    /// 
    /// Resets all cells to dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        let size = width * self.height;
        self.cells = FixedBitSet::with_capacity(size as usize);
        self.back_buffer = FixedBitSet::with_capacity(size as usize);
        self.dead_cells();
        std::mem::swap(&mut self.cells, &mut self.back_buffer);
    }

    /// Set the height of the Universe
    /// 
    /// Resets all cells to dead state
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        let size = self.width * height;
        self.cells = FixedBitSet::with_capacity(size as usize);
        self.back_buffer = FixedBitSet::with_capacity(size as usize);
        self.dead_cells();
        std::mem::swap(&mut self.cells, &mut self.back_buffer);
    }

    /// Render as text the universe
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Returns the width of the universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Returns the height of the universe
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Returns a pointer to the cells array
    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
    }

    /// Generate random cells
    pub fn random_cells(&mut self) {
        for i in 0..self.width * self.height {
            self.back_buffer.set(i as usize, Math::random() > 0.5);
            std::mem::swap(&mut self.cells, &mut self.back_buffer);
        }
    }

    /// Kill all cells
    pub fn dead_cells(&mut self) {
        for i in 0..self.width * self.height {
            self.back_buffer.set(i as usize, false);
            std::mem::swap(&mut self.cells, &mut self.back_buffer);
        }
    }
}

use std::fmt;
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let symbol = if self.cells.contains(idx) {'◼'} else {'◻'};
                write!(f, "{}", symbol)?;
            }
        }
        writeln!(f)?;
        Ok(())
    }
}