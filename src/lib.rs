// Importing wasm_bindgen crate to enable communication between JavaScript and Rust
use wasm_bindgen::prelude::*;

// Define the function that will be exposed to JavaScript
#[wasm_bindgen]
pub fn calculate_tax(income: f64) -> f64 {
    let mut tax = 0.0;

    if income <= 9875.0 {
        tax = income * 0.10;
    } else if income <= 40125.0 {
        tax = 987.5 + (income - 9875.0) * 0.12;
    } else if income <= 85525.0 {
        tax = 4617.5 + (income - 40125.0) * 0.22;
    } else if income <= 163300.0 {
        tax = 14605.5 + (income - 85525.0) * 0.24;
    } else if income <= 207350.0 {
        tax = 33271.5 + (income - 163300.0) * 0.32;
    } else if income <= 518400.0 {
        tax = 47367.5 + (income - 207350.0) * 0.35;
    } else {
        tax = 156235.0 + (income - 518400.0) * 0.37;
    }
    tax
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_tax() {
        // boundary  1: to $9,875
        assert_eq!(calculate_tax(0.0), 0.0);
        assert_eq!(calculate_tax(9_875.0), 987.5);

        // boundary 2: from $9,875 to $40,125
        assert_eq!(calculate_tax(9_876.0), 987.5 + 0.12);
        assert_eq!(calculate_tax(40_125.0), 4_617.5);

        // boundary 3: from $40,125 to $85,525
        assert_eq!(calculate_tax(40_126.0), 4_617.5 + 0.22);
        assert_eq!(calculate_tax(85_525.0), 14_605.5);

        // boundary 4: from $85,525 to $163,300
        assert_eq!(calculate_tax(85_526.0), 14_605.5 + 0.24);
        assert_eq!(calculate_tax(163_300.0), 33_271.5);

        // boundary 5: from $163,300 to $207,350
        assert_eq!(calculate_tax(163_301.0), 33_271.5 + 0.32);
        assert_eq!(calculate_tax(207_350.0), 47_367.5);

        // boundary 6: from $207,350 to $518,400
        assert_eq!(calculate_tax(207_351.0), 47_367.5 + 0.35);
        assert_eq!(calculate_tax(518_400.0), 156_235.0);

        // boundary 7: up $518,400
        assert_eq!(calculate_tax(518_401.0), 156_235.0 + 0.37);
        assert_eq!(calculate_tax(1_000_000.0), 156_235.0 + (1_000_000.0 - 518_400.0) * 0.37);
    }
}