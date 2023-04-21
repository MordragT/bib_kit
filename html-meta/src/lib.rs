#![feature(let_chains)]
#![feature(inherent_associated_types)]

pub use citation::*;
pub use dom::Dom;
use log::Level;
use wasm_bindgen::prelude::wasm_bindgen;

pub mod citation;
pub mod dom;
pub mod error;
pub mod meta;
pub mod priority;
pub mod query;

#[wasm_bindgen(start)]
pub fn init_console() {
    console_error_panic_hook::set_once();
    console_log::init_with_level(Level::Debug).unwrap();
}
