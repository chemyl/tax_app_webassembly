use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn calculate_tax(income: f64) -> f64 {
    let mut tax: f64 = 0.0;
    if income >= 1000.0 {
        tax = income * 0.03;
    } else if income >= 100000.0 {
        tax = income * 0.06;
    }
    tax
}