use super::{format_number, Calculator};

pub fn handle_basic(calc: &mut Calculator, label: &str) {
    match label {
        "AC"  => calc.clear(),
        "⌫"   => calc.backspace(),
        "%"   => calc.percent(),
        "+/−" => calc.negate(),
        "="   => calc.equals(),
        "+"   => calc.apply_op('+'),
        "−"   => calc.apply_op('-'),
        "×"   => calc.apply_op('*'),
        "÷"   => calc.apply_op('/'),
        "("   => calc.open_paren(),
        ")"   => calc.close_paren(),
        d if d.chars().next().map(|c| c.is_ascii_digit() || c == '.').unwrap_or(false) => {
            calc.push_digit(d.chars().next().unwrap())
        }
        _ => {}
    }
}

pub fn handle_sci(calc: &mut Calculator, label: &str) {
    match label {
        "sin"  => { let r = calc.deg_mode; calc.apply_sci_named("sin", |v| if r { v.to_radians().sin() } else { v.sin() }); }
        "cos"  => { let r = calc.deg_mode; calc.apply_sci_named("cos", |v| if r { v.to_radians().cos() } else { v.cos() }); }
        "tan"  => { let r = calc.deg_mode; calc.apply_sci_named("tan", |v| if r { v.to_radians().tan() } else { v.tan() }); }
        "asin" => { let r = calc.deg_mode; calc.apply_sci_named("asin", |v| { let a = v.asin(); if r { a.to_degrees() } else { a } }); }
        "acos" => { let r = calc.deg_mode; calc.apply_sci_named("acos", |v| { let a = v.acos(); if r { a.to_degrees() } else { a } }); }
        "atan" => { let r = calc.deg_mode; calc.apply_sci_named("atan", |v| { let a = v.atan(); if r { a.to_degrees() } else { a } }); }
        "π"    => {
            let repr = "π".to_string();
            calc.replace_formula_tail(&repr);
            calc.display = format_number(std::f64::consts::PI);
            calc.just_evaluated = true;
            calc.after_equals = true;
        }
        "e"    => {
            let repr = "e".to_string();
            calc.replace_formula_tail(&repr);
            calc.display = format_number(std::f64::consts::E);
            calc.just_evaluated = true;
            calc.after_equals = true;
        }
        "x²"   => calc.apply_sci_postfix("²", |v| v * v),
        "x³"   => calc.apply_sci_postfix("³", |v| v * v * v),
        "√x"   => calc.apply_sci_named("√", |v| v.sqrt()),
        "∛x"   => calc.apply_sci_named("∛", |v| v.cbrt()),
        "ln"   => calc.apply_sci_named("ln", |v| v.ln()),
        "log"  => calc.apply_sci_named("log", |v| v.log10()),
        "1/x"  => {
            let v = calc.current_value();
            if v == 0.0 {
                calc.set_error();
            } else {
                let arg = if calc.display.is_empty() { "0".to_string() } else { calc.display.clone() };
                let repr = format!("1/({})", arg);
                calc.replace_formula_tail(&repr);
                calc.display = format_number(1.0 / v);
                calc.just_evaluated = true;
                calc.after_equals = true;
            }
        }
        "x!"   => {
            let v = calc.current_value();
            let n = v as u64;
            if v < 0.0 || v.fract() != 0.0 || n > 20 {
                calc.set_error();
            } else {
                let factorial = (1..=n).product::<u64>() as f64;
                calc.apply_sci_postfix("!", |_| factorial);
            }
        }
        "xʸ"   => calc.apply_power(),
        "ʸ√x"  => calc.apply_nth_root(),
        "10ˣ"  => calc.apply_exp10(),
        "|x|"  => calc.apply_abs(),
        _ => {}
    }
}
