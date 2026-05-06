# Graph Report - C:\Users\user\Documents\learning\rust\scientifique_calculator  (2026-05-06)

## Corpus Check
- 10 files · ~4,771 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 68 nodes · 161 edges · 9 communities detected
- Extraction: 63% EXTRACTED · 37% INFERRED · 0% AMBIGUOUS · INFERRED: 59 edges (avg confidence: 0.8)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- [[_COMMUNITY_Community 0|Community 0]]
- [[_COMMUNITY_Community 1|Community 1]]
- [[_COMMUNITY_Community 2|Community 2]]
- [[_COMMUNITY_Community 3|Community 3]]
- [[_COMMUNITY_Community 4|Community 4]]
- [[_COMMUNITY_Community 5|Community 5]]
- [[_COMMUNITY_Community 6|Community 6]]
- [[_COMMUNITY_Community 7|Community 7]]
- [[_COMMUNITY_Community 8|Community 8]]

## God Nodes (most connected - your core abstractions)
1. `Calculator` - 20 edges
2. `format_number()` - 12 edges
3. `handle_sci()` - 12 edges
4. `handle_basic()` - 11 edges
5. `calc()` - 7 edges
6. `evaluate_formula()` - 6 edges
7. `test_backspace_after_paren()` - 6 edges
8. `tokenize()` - 4 edges
9. `parse_additive()` - 4 edges
10. `test_division_by_zero()` - 4 edges

## Surprising Connections (you probably didn't know these)
- `handle_basic()` --calls--> `basic_row()`  [INFERRED]
  C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\ops.rs → C:\Users\user\Documents\learning\rust\scientifique_calculator\src\ui\buttons.rs
- `handle_sci()` --calls--> `sci_row()`  [INFERRED]
  C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\ops.rs → C:\Users\user\Documents\learning\rust\scientifique_calculator\src\ui\buttons.rs
- `formula_result()` --calls--> `evaluate_formula()`  [INFERRED]
  C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\state\tests.rs → C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\state.rs
- `format_number()` --calls--> `handle_sci()`  [INFERRED]
  C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\mod.rs → C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\ops.rs

## Communities

### Community 0 - "Community 0"
Cohesion: 0.41
Nodes (3): format_number(), handle_sci(), Calculator

### Community 1 - "Community 1"
Cohesion: 0.3
Nodes (6): Calculator, render_display(), handle_basic(), calc(), test_backspace_after_paren(), test_division_by_zero()

### Community 2 - "Community 2"
Cohesion: 0.15
Nodes (2): formula_result(), test_formula_parser_complex()

### Community 3 - "Community 3"
Cohesion: 0.33
Nodes (9): evaluate_formula(), extract_last_op(), find_last_operator_pos(), parse_additive(), parse_atom(), parse_multiplicative(), SciOp, tokenize() (+1 more)

### Community 4 - "Community 4"
Cohesion: 0.53
Nodes (5): basic_row(), btn_colors(), BtnKind, calc_button(), sci_row()

### Community 5 - "Community 5"
Cohesion: 1.0
Nodes (0): 

### Community 6 - "Community 6"
Cohesion: 1.0
Nodes (0): 

### Community 7 - "Community 7"
Cohesion: 1.0
Nodes (0): 

### Community 8 - "Community 8"
Cohesion: 1.0
Nodes (0): 

## Knowledge Gaps
- **3 isolated node(s):** `Calculator`, `SciOp`, `BtnKind`
  These have ≤1 connection - possible missing edges or undocumented components.
- **Thin community `Community 5`** (2 nodes): `main()`, `build.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 6`** (2 nodes): `main.rs`, `main()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 7`** (1 nodes): `app.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 8`** (1 nodes): `mod.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Calculator` connect `Community 0` to `Community 1`, `Community 3`?**
  _High betweenness centrality (0.145) - this node is a cross-community bridge._
- **Why does `calc()` connect `Community 1` to `Community 0`, `Community 2`?**
  _High betweenness centrality (0.116) - this node is a cross-community bridge._
- **Why does `evaluate_formula()` connect `Community 3` to `Community 1`, `Community 2`?**
  _High betweenness centrality (0.076) - this node is a cross-community bridge._
- **Are the 11 inferred relationships involving `format_number()` (e.g. with `handle_sci()` and `.apply_op()`) actually correct?**
  _`format_number()` has 11 INFERRED edges - model-reasoned connections that need verification._
- **Are the 11 inferred relationships involving `handle_sci()` (e.g. with `.apply_sci_named()` and `.replace_formula_tail()`) actually correct?**
  _`handle_sci()` has 11 INFERRED edges - model-reasoned connections that need verification._
- **Are the 10 inferred relationships involving `handle_basic()` (e.g. with `.clear()` and `.backspace()`) actually correct?**
  _`handle_basic()` has 10 INFERRED edges - model-reasoned connections that need verification._
- **Are the 6 inferred relationships involving `calc()` (e.g. with `.push_digit()` and `.apply_op()`) actually correct?**
  _`calc()` has 6 INFERRED edges - model-reasoned connections that need verification._