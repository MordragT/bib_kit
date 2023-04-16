use doc::Document;
use hayagriva::{io::to_yaml_str, Entry};

pub mod doc;
pub mod error;

#[wasm_bindgen]
pub extern "C" fn generate_citation(dom: &str, url: &str) -> String {
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));

    panic!("{url}");

    let doc = Document::parse(dom, url).unwrap();
    let entry = Entry::try_from(doc).unwrap();
    to_yaml_str(Some(&entry)).unwrap()
}
