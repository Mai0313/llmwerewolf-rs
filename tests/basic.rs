#[test]
fn add_works() {
    assert_eq!(llmwerewolf_rs::add(10, 5), 15);
}

#[test]
fn multiply_works() {
    assert_eq!(llmwerewolf_rs::multiply(4, 5), 20);
}

#[test]
fn subtract_works() {
    assert_eq!(llmwerewolf_rs::subtract(10, 3), 7);
}

#[test]
fn calculate_and_display_works() {
    let result = llmwerewolf_rs::calculate_and_display(7, 8);
    assert_eq!(result, "7 + 8 = 15");
}

#[test]
fn integration_test_complex_calculation() {
    // Test a more complex scenario using multiple functions
    let a = 10;
    let b = 5;
    let sum = llmwerewolf_rs::add(a, b);
    let product = llmwerewolf_rs::multiply(a, b);
    let difference = llmwerewolf_rs::subtract(a, b);

    assert_eq!(sum, 15);
    assert_eq!(product, 50);
    assert_eq!(difference, 5);

    // Test the display function
    let display = llmwerewolf_rs::calculate_and_display(a, b);
    assert_eq!(display, "10 + 5 = 15");
}

#[test]
fn edge_cases_test() {
    // Test edge cases
    assert_eq!(llmwerewolf_rs::add(0, 0), 0);
    assert_eq!(llmwerewolf_rs::multiply(0, 100), 0);
    assert_eq!(llmwerewolf_rs::subtract(0, 0), 0);

    // Test negative numbers
    assert_eq!(llmwerewolf_rs::add(-5, -3), -8);
    assert_eq!(llmwerewolf_rs::multiply(-2, 3), -6);
    assert_eq!(llmwerewolf_rs::subtract(-5, -3), -2);
}
