use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);
#[wasm_bindgen]
pub fn update_element(id: &str, text: &str) {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.get_element_by_id(id).unwrap();
    element.set_inner_html(text);
}

#[wasm_bindgen_test]
fn test_update_element() {
    update_element("test-id", "Hello, Wasm!");
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document.get_element_by_id("test-id").unwrap();
    assert_eq!(element.inner_html(), "Hello, Wasm!");
}