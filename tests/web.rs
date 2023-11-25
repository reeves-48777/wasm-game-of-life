//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use wasm_game_of_life::Universe;

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
fn test_tick() {
    let mut input_universe = input_spaceship();
    let expected_universe = expected_spaceship();

    input_universe.tick();
    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}

#[wasm_bindgen_test]
fn test_delta_based_update() {
    let mut input_universe = input_spaceship();
    let expected_coordinates = vec![(1,2), (2,1), (3,1), (4,2)];

    input_universe.tick();
    let given_coordinates: Vec<(u32, u32)> = serde_wasm_bindgen::from_value(input_universe.get_changes()).unwrap();
    assert_eq!(given_coordinates, expected_coordinates);
}