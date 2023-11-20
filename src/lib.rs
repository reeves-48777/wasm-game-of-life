mod utils;

use wasm_bindgen::prelude::*;
use fixedbitset::FixedBitSet;
use js_sys::Math;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[allow(unused_macros)]
macro_rules! log {
    ($( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    };
}

#[wasm_bindgen]
pub enum CommonSpaceships {
    Glider,
    Lightweight,
    Middleweight,
    Heavyweight
}
impl CommonSpaceships {
    pub fn pattern(&self) -> (Vec<bool>, usize, usize) {
        match self {
            CommonSpaceships::Glider => {
                (vec![
                    false, true, false,
                    false, false, true,
                    true, true, true
                ], 3, 3)
            },

            CommonSpaceships::Lightweight => {
                (vec![
                    false, true, true, true, true,
                    true, false, false, false, true,
                    false, false, false, false, true,
                    true, false, false, true, false
                ], 5, 4)
            },

            CommonSpaceships::Middleweight => {
                (vec![
                    false, true, true, true, true, true,
                    true, false, false, false, false, true,
                    false, false, false, false, false, true,
                    true, false, false, false,true, false,
                    false, false, true, false, false, false
                ], 6, 5)
            },

            CommonSpaceships::Heavyweight => {
                (vec![
                    false, true, true, true, true, true, true,
                    true, false, false, false, false, false, true,
                    false, false, false, false, false, false, true,
                    false, false, false, false, false, true, false, 
                ], 7, 4)
            }
        }

    }
}



#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: FixedBitSet
}

impl Universe {
    fn get_index(&self, row :u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }
    fn live_neighbour_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for delta_row in [self.height -1, 0, 1].iter().cloned() {
            for delta_col in [self.width -1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let neighbour_row = (row + delta_row) % self.height;
                let neighbour_col = (col + delta_col) % self.width;
                let idx = self.get_index(neighbour_row, neighbour_col);
                count += self.cells[idx] as u8;
            }
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
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbours = self.live_neighbour_count(row, col);

                // additional logging
                // log!(
                //     "cell[{},{}] is initially {:?} and has {} live neighbours",
                //     row,
                //     col,
                //     cell,
                //     live_neighbours
                // );

                next.set(idx, match (cell, live_neighbours) {
                    // Rule 1 : Any live cell with fewer than two lives neighbour dies, underpopulation
                    (true, x) if x < 2 => {
                        // Exercise - debugging - log in tick function
                        // log!("cell[{}, {}] was alive, became dead", row, col);
                        false
                    },
                    // Rule 2 : Any live cell with two or three live neighbours lives
                    (true, 2) | (true, 3) => true,
                    // Rule 3 : Any live cell with more than three neighbours live dies, overpopulation
                    (true, x) if x > 3 => {
                        // Exercise - debugging - log in tick function
                        // log!("cell[{}, {}] was alive, became dead", row, col);
                        false
                    },
                    // Rule 4 : Anu dead cell with exactly three live neighbours becomes alive
                    (false, 3) => {
                        // Exercise - debugging - log in tick function
                        // log!("cell[{}, {}] was dead, became alive", row, col);
                        true
                    },
                    // all the others remain in the same state
                    (other, _) => other
                });

                // log!("  it becomes {:?}", next[idx]);
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        utils::set_panic_hook();

        let width = 64;
        let height = 64;

        let size = (width * height) as usize;
        let mut cells = FixedBitSet::with_capacity(size);

        // Exercise - implementing
        // Random cell values with Math.random() from js
        for i in 0..size {
            cells.set(i, Math::random() > 0.5);
        }

        // Exercise - debugging
        //panic!()

        Universe {
            width,
            height,
            cells
        }
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

                if universe_index < self.cells.len() {
                    self.cells.set(universe_index, pattern[pattern_index]);
                }
            }
        }
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells.toggle(idx);
    }

    /// Set the width of the Universe
    /// 
    /// Resets all cells to dead state
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        (0..width * self.height).for_each(|i| self.cells.set(i as usize, false));
    }

    /// Set the height of the Universe
    /// 
    /// Resets all cells to dead state
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        (0..self.width * height).for_each(|i| self.cells.set(i as usize, false));
    }

    pub fn render(&self) -> String {
        self.to_string()
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const u32 {
        self.cells.as_slice().as_ptr()
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

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::*;
    use super::*;

    pub fn input_spaceship() -> Universe {
        let mut universe = Universe::new();
        universe.set_width(6);
        universe.set_height(6);
        universe.set_cells(&[(1,2), (2,3), (3,1), (3,2), (3,3)]);
        universe
    }

    pub fn expected_spaceship() -> Universe {
        let mut universe = Universe::new();
        universe.set_width(6);
        universe.set_height(6);
        universe.set_cells(&[(2,1), (2,3), (3,2), (3,3), (4,2)]);
        universe
    }

    wasm_bindgen_test_configure!(run_in_browser);
    
    #[wasm_bindgen_test]
    fn create_universe() {
        let universe = Universe::new();
        assert_eq!(universe.width, 64);
        assert_eq!(universe.height, 64);
        assert!(universe.cells.len() > 0);
    }

    #[wasm_bindgen_test]
    fn create_universe_and_check_render() {
        let mut universe = Universe::new();
        let text = universe.render();
        assert!(text.len() > 0);
        universe.tick();
        let text_after_tick = universe.render();
        assert_ne!(text, text_after_tick);
    }

    #[wasm_bindgen_test]
    fn test_tick() {
        let mut input_universe = input_spaceship();
        let expected_universe = expected_spaceship();

        input_universe.tick();
        assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
    }
    
}