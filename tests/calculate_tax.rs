use wasm_bindgen_test::*;
use tax_app_webassembly::calculate_tax;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_calculate_tax_wasm() {
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