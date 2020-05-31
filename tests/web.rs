//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_game_of_life;
use wasm_game_of_life::board::Board;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
#[cfg(test)]
#[test]
fn pass() {
    assert_eq!(1 + 1, 2);
}
