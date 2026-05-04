use super::format_number;

#[derive(Default)]
pub struct Calculator {
    pub display: String,
    pub formula: String,
    pub pending_op: Option<char>,
    pub pending_value: Option<f64>,
    pub just_evaluated: bool,
    pub after_equals: bool,
    pub show_scientific: bool,
    pub error: bool,
    pub deg_mode: bool,
    pub paren_stack: Vec<(Option<char>, Option<f64>)>,
    pub pending_sci: Option<(SciOp, f64)>,
}

#[derive(Clone, Copy)]
pub enum SciOp {
    Power,
    NthRoot,
}

impl Calculator {
    pub fn current_value(&self) -> f64 {
        self.display.parse().unwrap_or(0.0)
    }

    pub fn push_digit(&mut self, ch: char) {
        if self.error {
            self.formula.clear();
            self.display.clear();
            self.pending_sci = None;
            self.error = false;
            self.just_evaluated = false;
            self.after_equals = false;
        } else if self.after_equals {
            self.formula.clear();
            self.display.clear();
            self.pending_sci = None;
            self.just_evaluated = false;
            self.after_equals = false;
        } else if self.just_evaluated {
            self.display.clear();
            self.just_evaluated = false;
        }
        if ch == '.' && self.display.contains('.') {
            return;
        }
        if self.display == "0" && ch != '.' {
            self.display.clear();
            if self.formula.ends_with('0') {
                self.formula.pop();
            }
        }
        self.display.push(ch);
        self.formula.push(ch);
    }

    pub fn apply_op(&mut self, op: char) {
        if self.error { return; }
        self.eval_pending_sci();
        if let Some(pending) = self.pending_op {
            let a = self.pending_value.unwrap_or(0.0);
            let b = self.current_value();
            let result = match pending {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => {
                    if b == 0.0 { self.set_error(); return; }
                    a / b
                }
                _ => b,
            };
            self.display = format_number(result);
        }
        let op_sym = match op {
            '+' => '+',
            '-' => '−',
            '*' => '×',
            '/' => '÷',
            _ => op,
        };
        self.formula.push(' ');
        self.formula.push(op_sym);
        self.formula.push(' ');
        self.pending_op = Some(op);
        self.pending_value = Some(self.current_value());
        self.just_evaluated = true;
        self.after_equals = false;
    }

    pub fn equals(&mut self) {
        if self.error { return; }
        self.eval_pending_sci();
        if let Some(pending) = self.pending_op {
            let a = self.pending_value.unwrap_or(0.0);
            let b = self.current_value();
            let result = match pending {
                '+' => a + b,
                '-' => a - b,
                '*' => a * b,
                '/' => {
                    if b == 0.0 { self.set_error(); return; }
                    a / b
                }
                _ => b,
            };
            self.display = format_number(result);
            self.pending_op = None;
            self.pending_value = None;
            self.just_evaluated = true;
            self.after_equals = true;
        }
    }

    pub fn clear(&mut self) {
        self.display.clear();
        self.formula.clear();
        self.pending_op = None;
        self.pending_value = None;
        self.pending_sci = None;
        self.just_evaluated = false;
        self.after_equals = false;
        self.error = false;
        self.paren_stack.clear();
    }

    pub fn backspace(&mut self) {
        if self.error {
            self.clear();
            return;
        }
        if self.just_evaluated {
            self.display.clear();
            self.pending_sci = None;
            self.just_evaluated = false;
            return;
        }
        if self.display.is_empty() {
            if let Some(pos) = find_last_operator_pos(&self.formula) {
                self.formula.truncate(pos);
                let remaining = self.formula.clone();
                let result = evaluate_formula(&remaining);
                self.pending_value = Some(result);
                self.pending_op = extract_last_op(&remaining);
                self.pending_sci = None;
                self.display.clear();
            } else {
                self.formula.clear();
                self.pending_op = None;
                self.pending_value = None;
                self.pending_sci = None;
            }
            return;
        }
        self.display.pop();
        self.formula.pop();
    }

    pub fn negate(&mut self) {
        if self.error { return; }
        let v = self.current_value();
        let new_val = format_number(-v);
        self.replace_formula_tail(&new_val.clone());
        self.display = new_val;
    }

    pub fn percent(&mut self) {
        if self.error { return; }
        let v = self.current_value();
        let new_val = format_number(v / 100.0);
        self.replace_formula_tail(&new_val.clone());
        self.display = new_val;
        self.just_evaluated = true;
    }

    pub fn open_paren(&mut self) {
        if self.error { return; }
        self.paren_stack.push((self.pending_op, self.pending_value));
        self.pending_op = None;
        self.pending_value = None;
        self.formula.push('(');
        self.display.clear();
        self.just_evaluated = false;
        self.after_equals = false;
    }

    pub fn close_paren(&mut self) {
        if self.error || self.paren_stack.is_empty() { return; }
        if self.pending_op.is_some() {
            self.equals();
        }
        let result = self.current_value();
        let (outer_op, outer_val) = self.paren_stack.pop().unwrap();
        self.pending_op = outer_op;
        self.pending_value = outer_val;
        self.display = format_number(result);
        self.formula.push(')');
        self.just_evaluated = true;
        self.after_equals = false;
    }

    /// Evaluate any pending binary scientific operation (xʸ or ʸ√x).
    fn eval_pending_sci(&mut self) {
        if let Some((sci_op, base)) = self.pending_sci.take() {
            let exponent = self.current_value();
            let result = match sci_op {
                SciOp::Power => base.powf(exponent),
                SciOp::NthRoot => {
                    if exponent == 0.0 { self.set_error(); return; }
                    base.powf(1.0 / exponent)
                }
            };
            if result.is_nan() || result.is_infinite() {
                self.set_error();
            } else {
                self.display = format_number(result);
                self.just_evaluated = true;
            }
        }
    }

    pub fn apply_sci_named(&mut self, name: &str, f: impl Fn(f64) -> f64) {
        if self.error { return; }
        let v = self.current_value();
        let result = f(v);
        if result.is_nan() || result.is_infinite() {
            self.set_error();
        } else {
            let arg = if self.display.is_empty() { "0" } else { &self.display };
            let repr = format!("{}({})", name, arg);
            self.replace_formula_tail(&repr);
            self.display = format_number(result);
            self.just_evaluated = true;
            self.after_equals = true;
        }
    }

    pub fn apply_sci_postfix(&mut self, suffix: &str, f: impl Fn(f64) -> f64) {
        if self.error { return; }
        let v = self.current_value();
        let result = f(v);
        if result.is_nan() || result.is_infinite() {
            self.set_error();
        } else {
            let arg = if self.display.is_empty() { "0" } else { &self.display };
            let repr = format!("{}{}", arg, suffix);
            self.replace_formula_tail(&repr);
            self.display = format_number(result);
            self.just_evaluated = true;
            self.after_equals = true;
        }
    }

    pub fn apply_power(&mut self) {
        if self.error { return; }
        self.pending_sci = Some((SciOp::Power, self.current_value()));
        self.formula.push_str(" ^ ");
        self.just_evaluated = true;
    }

    pub fn apply_nth_root(&mut self) {
        if self.error { return; }
        self.pending_sci = Some((SciOp::NthRoot, self.current_value()));
        self.formula.push_str(" y√ ");
        self.just_evaluated = true;
    }

    pub fn apply_exp10(&mut self) {
        if self.error { return; }
        let v = self.current_value();
        let result = 10f64.powf(v);
        if result.is_nan() || result.is_infinite() {
            self.set_error();
        } else {
            let arg = if self.display.is_empty() { "0" } else { &self.display };
            let repr = format!("10^({})", arg);
            self.replace_formula_tail(&repr);
            self.display = format_number(result);
            self.just_evaluated = true;
            self.after_equals = true;
        }
    }

    pub fn apply_abs(&mut self) {
        if self.error { return; }
        let v = self.current_value();
        let result = v.abs();
        let arg = if self.display.is_empty() { "0" } else { &self.display };
        let repr = format!("|{}|", arg);
        self.replace_formula_tail(&repr);
        self.display = format_number(result);
        self.just_evaluated = true;
        self.after_equals = true;
    }

    pub fn replace_formula_tail(&mut self, new_repr: &str) {
        let tail_len = self.display.len();
        if self.formula.len() >= tail_len {
            self.formula.truncate(self.formula.len() - tail_len);
            self.formula.push_str(new_repr);
        }
    }

    pub fn set_error(&mut self) {
        self.display = "Error".into();
        self.error = true;
        self.pending_op = None;
        self.pending_value = None;
        self.pending_sci = None;
    }
}

fn find_last_operator_pos(formula: &str) -> Option<usize> {
    let mut depth = 0usize;
    let mut last_op_pos = None;
    for (i, ch) in formula.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth > 0 { depth -= 1; }
            }
            '+' | '−' | '×' | '÷' if depth == 0 => {
                last_op_pos = Some(i);
            }
            _ => {}
        }
    }
    last_op_pos
}

fn evaluate_formula(formula: &str) -> f64 {
    let tokens: Vec<String> = tokenize(formula);
    if tokens.is_empty() { return 0.0; }
    let (result, _) = parse_additive(&tokens, 0);
    result
}

fn extract_last_op(formula: &str) -> Option<char> {
    let tokens: Vec<String> = tokenize(formula);
    for token in tokens.iter().rev() {
        match token.as_str() {
            "+" => return Some('+'),
            "−" => return Some('-'),
            "×" => return Some('*'),
            "÷" => return Some('/'),
            _ => {}
        }
    }
    None
}

fn tokenize(formula: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    for ch in formula.chars() {
        match ch {
            '+' | '−' | '×' | '÷' | '(' | ')' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
                tokens.push(ch.to_string());
            }
            ' ' => {
                if !current.is_empty() {
                    tokens.push(current.clone());
                    current.clear();
                }
            }
            _ => {
                current.push(ch);
            }
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}

fn parse_additive(tokens: &[String], pos: usize) -> (f64, usize) {
    let (mut result, mut pos) = parse_multiplicative(tokens, pos);
    while pos < tokens.len() {
        match tokens[pos].as_str() {
            "+" => {
                pos += 1;
                let (rhs, next) = parse_multiplicative(tokens, pos);
                result += rhs;
                pos = next;
            }
            "−" => {
                pos += 1;
                let (rhs, next) = parse_multiplicative(tokens, pos);
                result -= rhs;
                pos = next;
            }
            _ => break,
        }
    }
    (result, pos)
}

fn parse_multiplicative(tokens: &[String], pos: usize) -> (f64, usize) {
    let (mut result, mut pos) = parse_atom(tokens, pos);
    while pos < tokens.len() {
        match tokens[pos].as_str() {
            "×" => {
                pos += 1;
                let (rhs, next) = parse_atom(tokens, pos);
                result *= rhs;
                pos = next;
            }
            "÷" => {
                pos += 1;
                let (rhs, next) = parse_atom(tokens, pos);
                if rhs != 0.0 {
                    result /= rhs;
                } else {
                    result = f64::NAN;
                }
                pos = next;
            }
            _ => break,
        }
    }
    (result, pos)
}

fn parse_atom(tokens: &[String], pos: usize) -> (f64, usize) {
    if pos >= tokens.len() { return (0.0, pos); }
    match tokens[pos].as_str() {
        "(" => {
            let (result, next) = parse_additive(tokens, pos + 1);
            let next = if next < tokens.len() && tokens[next] == ")" { next + 1 } else { next };
            (result, next)
        }
        _ => {
            let val = tokens[pos].parse::<f64>().unwrap_or(0.0);
            (val, pos + 1)
        }
    }
}
