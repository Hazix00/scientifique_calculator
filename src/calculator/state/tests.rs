use super::*;

// Simulates user input through the Calculator's public API.
// Each character in input is mapped to a button press:
//   '0'..'9', '.' => push_digit
//   '+', '-', '*', '/' => apply_op
//   '=' => equals
//   '(' , ')' => open_paren / close_paren
fn calc(input: &str) -> f64 {
    let mut calc = Calculator::default();
    for ch in input.chars() {
        match ch {
            '0'..='9' | '.' => calc.push_digit(ch),
            '+' => calc.apply_op('+'),
            '-' => calc.apply_op('-'),
            '*' => calc.apply_op('*'),
            '/' => calc.apply_op('/'),
            '=' => calc.equals(),
            '(' => calc.open_paren(),
            ')' => calc.close_paren(),
            _ => {}
        }
    }
    calc.current_value()
}

// Directly calls the formula parser (recursive descent evaluator).
// Useful for testing the parser independently of the Calculator state machine.
fn formula_result(input: &str) -> f64 {
    evaluate_formula(input)
}

// ─── Calculator integration tests ───

#[test]
fn test_simple_addition() {
    // 2 + 3 = 5
    assert_eq!(calc("2+3="), 5.0);
}

#[test]
fn test_simple_multiplication() {
    // 4 × 5 = 20
    assert_eq!(calc("4*5="), 20.0);
}

#[test]
fn test_order_of_operations_basic() {
    // 2 + 3 × 4 = 2 + 12 = 14 (not 20)
    // Verifies multiplication binds tighter than addition at equals time.
    assert_eq!(calc("2+3*4="), 14.0);
}

#[test]
fn test_parentheses_basic() {
    // (2 + 3) × 4 = 5 × 4 = 20
    // Verifies parenthesized group is evaluated before the outer operator.
    assert_eq!(calc("(2+3)*4="), 20.0);
}

#[test]
fn test_nested_parentheses() {
    // (3 + 1) ÷ 2 = 4 ÷ 2 = 2
    // Verifies nested parens evaluate to the correct intermediate value.
    assert_eq!(calc("(3+1)/2="), 2.0);
}

#[test]
fn test_complex_expression_with_parens() {
    // (896 × 9) − (8 × 5) × 2
    //   = 8064 − 40 × 2
    //   = 8064 − 80
    //   = 7984
    // This was the bug that produced 0 — verifies full-chain precedence works.
    assert_eq!(calc("(896*9)-(8*5)*2="), 7984.0);
}

#[test]
fn test_chained_mixed_ops() {
    // 10 + 2 × 3 − 4 ÷ 2
    //   = 10 + 6 − 2
    //   = 14
    // Verifies multiple +/− and ×/÷ operators in one expression.
    assert_eq!(calc("10+2*3-4/2="), 14.0);
}

// ─── Formula parser unit tests ───

#[test]
fn test_formula_parser_addition() {
    // "23 + 5" should parse to 28.0
    assert!((formula_result("23 + 5") - 28.0).abs() < f64::EPSILON);
}

#[test]
fn test_formula_parser_precedence() {
    // "23 + 6 × 5" should respect precedence: 23 + (6×5) = 53
    // Ensures parse_additive calls parse_multiplicative for the RHS.
    assert!((formula_result("23 + 6 × 5") - 53.0).abs() < f64::EPSILON);
}

#[test]
fn test_formula_parser_parens() {
    // "(34 − 55) + (3 − 1)" = (−21) + 2 = −19
    // Verifies parentheses are handled in parse_atom.
    assert!((formula_result("(34 − 55) + (3 − 1)") - (-19.0)).abs() < f64::EPSILON);
}

#[test]
fn test_formula_parser_full_chain() {
    // "(896 × 9) − (8 × 5) × 2" = 8064 − 80 = 7984
    // End-to-end parser test with the exact expression that originally returned 0.
    assert!((formula_result("(896 × 9) − (8 × 5) × 2") - 7984.0).abs() < f64::EPSILON);
}

#[test]
fn test_formula_parser_complex() {
    // "23 − 6 × (34 − 55) + (3 − 1) ÷ 3"
    //   = 23 − 6 × (−21) + 2 ÷ 3
    //   = 23 + 126 + 0.667
    //   = 149.667
    // Combines subtraction, multiplication into parens, addition, and division into parens.
    let result = formula_result("23 − 6 × (34 − 55) + (3 − 1) ÷ 3");
    assert!((result - 149.66666666666666).abs() < 0.0001);
}

// ─── Edge case and regression tests ───

#[test]
fn test_division_by_zero() {
    // 5 ÷ 0 should set the error flag and not panic.
    let mut calc = Calculator::default();
    calc.push_digit('5');
    calc.apply_op('/');
    calc.push_digit('0');
    calc.equals();
    assert!(calc.error);
}

#[test]
fn test_backspace_clears_display() {
    // Typing "123" then backspace should leave "12" in both display and formula.
    let mut calc = Calculator::default();
    calc.push_digit('1');
    calc.push_digit('2');
    calc.push_digit('3');
    calc.backspace();
    assert_eq!(calc.display, "12");
    assert_eq!(calc.formula, "12");
}

#[test]
fn test_backspace_after_paren() {
    // Typing "(23+5)" then close-paren (which sets just_evaluated=true),
    // then backspace should clear only the display — NOT reset the whole calculator.
    // This was a regression where backspace wiped everything after parens.
    let mut calc = Calculator::default();
    calc.open_paren();
    calc.push_digit('2');
    calc.push_digit('3');
    calc.apply_op('+');
    calc.push_digit('5');
    calc.close_paren();
    calc.backspace();
    assert!(calc.display.is_empty());
    assert_eq!(calc.formula, "(23 + 5)");
}
