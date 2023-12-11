use wasm_bindgen::prelude::*;

/// Common spaceships
/// 
/// This enum contains the most common spaceships
#[wasm_bindgen]
pub enum CommonSpaceships {
    Glider,
    Lightweight,
    Middleweight,
    Heavyweight
}

impl CommonSpaceships {
    /// Returns the pattern of the spaceship, the width and the height
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