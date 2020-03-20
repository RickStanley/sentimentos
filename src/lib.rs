mod lang;
mod language_processor;
mod tokenize;
mod utils;

use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn anaylize(phrase: &str, language: &str) -> String {
    // @todo Remove hardcoded PT Lang
    language_processor::analyze(phrase, language)
}
