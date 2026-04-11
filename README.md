# aleparser

Parses Aleph source code into an [`AlephTree`](https://github.com/aleph-lang/aleph-syntax-tree).
Built with [LALRPOP](https://github.com/lalrpop/lalrpop).

## Installation

```toml
[dependencies]
aleparser = "0.1"
```

## Usage

```rust
let ast = aleparser::parse("3 + 4".to_string());
```

## Example

Input:

```aleph
let x = 42
x + 1
```

Produces an `AlephTree::Stmts` containing a `Let` binding followed by an `Add` expression.

## Related

- [`aleph-syntax-tree`](https://github.com/aleph-lang/aleph-syntax-tree) — AST definition
- [`alephc`](https://github.com/aleph-lang/aleph) — uses this parser with `--features ale_parse`
