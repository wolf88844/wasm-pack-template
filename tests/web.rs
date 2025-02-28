//! Test suite for the Web and headless browsers.
#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;

extern crate wasm_game_of_life;
use wasm_game_of_life::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[cfg(test)]
pub fn input_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(1, 2), (2, 3), (3, 1), (3, 2), (3, 3)]);
    universe
}

#[cfg(test)]
pub fn expected_spaceship() -> Universe {
    let mut universe = Universe::new();
    universe.set_width(6);
    universe.set_height(6);
    universe.set_cells(&[(2, 1), (2, 3), (3, 2), (3, 3), (4, 2)]);
    universe
}

#[wasm_bindgen_test]
pub fn test_tick() {
    let mut input_universe = input_spaceship();
    let expected_universe = expected_spaceship();

    // Add debug output
    let input_str = format!("Input cells: {:?}", input_universe.get_cells());
    let expected_str = format!("Expected cells: {:?}", expected_universe.get_cells());
    web_sys::console::log_1(&input_str.into());

    input_universe.tick();

    let actual_str = format!("After tick: {:?}", input_universe.get_cells());
    web_sys::console::log_1(&actual_str.into());
    web_sys::console::log_1(&expected_str.into());

    assert_eq!(&input_universe.get_cells(), &expected_universe.get_cells());
}
