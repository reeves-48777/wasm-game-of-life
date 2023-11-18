mod utils;

use wasm_bindgen::prelude::*;
use js_sys::Math;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub enum CommonSpaceships {
    Glider,
    Lightweight,
    Middleweight,
    Heavyweight
}
impl CommonSpaceships {
    pub fn pattern(&self) -> (Vec<Cell>, usize, usize) {
        match self {
            CommonSpaceships::Glider => {
                (vec![
                    Cell::Dead, Cell::Alive, Cell::Dead,
                    Cell::Dead, Cell::Dead, Cell::Alive,
                    Cell::Alive, Cell::Alive, Cell::Alive
                ], 3, 3)
            },

            CommonSpaceships::Lightweight => {
                (vec![
                    Cell::Dead, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive,
                    Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive,
                    Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive,
                    Cell::Alive, Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead
                ], 5, 4)
            },

            CommonSpaceships::Middleweight => {
                (vec![
                    Cell::Dead, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive,
                    Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive,
                    Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive,
                    Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead,Cell::Alive, Cell::Dead,
                    Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead
                ], 6, 5)
            },

            CommonSpaceships::Heavyweight => {
                (vec![
                    Cell::Dead, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive, Cell::Alive,
                    Cell::Alive, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive,
                    Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive,
                    Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Dead, Cell::Alive, Cell::Dead, 
                ], 7, 4)
            }
        }

    }
}



#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
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

                let next_cell = match (cell, live_neighbours) {
                    // Rule 1 : Any live cell with fewer than two lives neighbour dies, underpopulation
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2 : Any live cell with two or three live neighbours lives
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3 : Any live cell with more than three neighbours live dies, overpopulation
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4 : Anu dead cell with exactly three live neighbours becomes alive
                    (Cell::Dead, 3) => Cell::Alive,
                    // all the others remain in the same state
                    (other, _) => other
                };
                next[idx] = next_cell;
            }
        }
        self.cells = next;
    }

    pub fn new() -> Universe {
        let width = 64;
        let height = 64;

        let cells = (0..width * height)
        .map(|_index| {
            // initialisation de base (meme génération tout le temps)
            // if index % 2 == 0 || index % 7 == 0 {
            //     Cell::Alive
            // } else {
            //     Cell::Dead
            // }

            // init avec cellules à vide
            //Cell::Dead

            // init avec Math.random()
            if Math::random() > 0.5 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        })
        .collect();

        Universe {
            width,
            height,
            cells
        }
    }

    pub fn add_spaceship(&mut self, spaceship: CommonSpaceships, start_x: usize, start_y: usize) {
        let (pattern, pat_width, pat_height) = spaceship.pattern();

        for row in 0..pat_height {
            for col in 0..pat_width {
                let pattern_index = row * pat_width + col;
                let universe_index = (start_y + row) * self.width as usize + (start_x + col);

                if universe_index < self.cells.len() {
                    self.cells[universe_index] = pattern[pattern_index]
                }
            }
        }
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

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }
}

use std::fmt;
impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn create_universe() {
        let universe = Universe::new();
        assert_eq!(universe.width, 64);
        assert_eq!(universe.height, 64);
        assert!(universe.cells.len() > 0);
    }

    #[test]
    fn create_universe_and_check_render() {
        let mut universe = Universe::new();
        let text = universe.render();
        // println!("{}", text);
        assert!(text.len() > 0);
        universe.tick();
        let text_after_tick = universe.render();
        // println!("{}", text);
        assert_ne!(text, text_after_tick);
    }
}