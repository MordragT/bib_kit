use doc::Document;
use hayagriva::{io::to_yaml_str, Entry};
use wasm_bindgen::{prelude::wasm_bindgen, JsError};

pub mod doc;
pub mod error;

#[wasm_bindgen]
pub extern "C" fn generate_citation(dom: &str, url: &str) -> Result<String, JsError> {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    let doc = Document::parse(dom, url)?;
    let entry = Entry::try_from(doc)?;
    Ok(to_yaml_str(Some(&entry)).unwrap())
}
