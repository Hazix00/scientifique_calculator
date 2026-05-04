# Graph Report - C:\Users\user\Documents\learning\rust\scientifique_calculator  (2026-05-05)

## Corpus Check
- 8 files · ~3,671 words
- Verdict: corpus is large enough that graph structure adds value.

## Summary
- 48 nodes · 124 edges · 9 communities detected
- Extraction: 66% EXTRACTED · 34% INFERRED · 0% AMBIGUOUS · INFERRED: 42 edges (avg confidence: 0.8)
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
5. `evaluate_formula()` - 4 edges
6. `tokenize()` - 4 edges
7. `parse_additive()` - 4 edges
8. `calc_button()` - 4 edges
9. `basic_row()` - 4 edges
10. `sci_row()` - 4 edges

## Surprising Connections (you probably didn't know these)
- `handle_basic()` --calls--> `basic_row()`  [INFERRED]
  C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\ops.rs → C:\Users\user\Documents\learning\rust\scientifique_calculator\src\ui\buttons.rs
- `handle_sci()` --calls--> `sci_row()`  [INFERRED]
  C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\ops.rs → C:\Users\user\Documents\learning\rust\scientifique_calculator\src\ui\buttons.rs
- `format_number()` --calls--> `handle_sci()`  [INFERRED]
  C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\mod.rs → C:\Users\user\Documents\learning\rust\scientifique_calculator\src\calculator\ops.rs

## Communities

### Community 0 - "Community 0"
Cohesion: 0.38
Nodes (8): evaluate_formula(), extract_last_op(), find_last_operator_pos(), parse_additive(), parse_atom(), parse_multiplicative(), SciOp, tokenize()

### Community 1 - "Community 1"
Cohesion: 0.31
Nodes (3): Calculator, render_display(), handle_basic()

### Community 2 - "Community 2"
Cohesion: 0.54
Nodes (2): format_number(), handle_sci()

### Community 3 - "Community 3"
Cohesion: 0.6
Nodes (1): Calculator

### Community 4 - "Community 4"
Cohesion: 0.53
Nodes (5): basic_row(), btn_colors(), BtnKind, calc_button(), sci_row()

### Community 5 - "Community 5"
Cohesion: 0.83
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
- **Thin community `Community 6`** (2 nodes): `main.rs`, `main()`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 7`** (1 nodes): `app.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.
- **Thin community `Community 8`** (1 nodes): `mod.rs`
  Too small to be a meaningful cluster - may be noise or needs more connections extracted.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Calculator` connect `Community 3` to `Community 0`, `Community 1`, `Community 2`, `Community 5`?**
  _High betweenness centrality (0.279) - this node is a cross-community bridge._
- **Why does `handle_sci()` connect `Community 2` to `Community 1`, `Community 3`, `Community 4`, `Community 5`?**
  _High betweenness centrality (0.088) - this node is a cross-community bridge._
- **Why does `sci_row()` connect `Community 4` to `Community 1`, `Community 2`?**
  _High betweenness centrality (0.087) - this node is a cross-community bridge._
- **Are the 11 inferred relationships involving `format_number()` (e.g. with `handle_sci()` and `.apply_op()`) actually correct?**
  _`format_number()` has 11 INFERRED edges - model-reasoned connections that need verification._
- **Are the 11 inferred relationships involving `handle_sci()` (e.g. with `.apply_sci_named()` and `.replace_formula_tail()`) actually correct?**
  _`handle_sci()` has 11 INFERRED edges - model-reasoned connections that need verification._
- **Are the 10 inferred relationships involving `handle_basic()` (e.g. with `.clear()` and `.backspace()`) actually correct?**
  _`handle_basic()` has 10 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Calculator`, `SciOp`, `BtnKind` to the rest of the system?**
  _3 weakly-connected nodes found - possible documentation gaps or missing edges._