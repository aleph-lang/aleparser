# aleparser

Parses Aleph source code into an [`AlephTree`](https://github.com/aleph-lang/aleph-syntax-tree).
Built with [LALRPOP](https://github.com/lalrpop/lalrpop).

## Installation

```toml
[dependencies]
aleparser = "0.2"
```

## Usage

```rust
let ast = aleparser::parse("3 + 4".to_string());
```

## Typed, effect-annotated functions and sum types (0.2+)

On top of everything below, this crate parses:

```aleph
type Shape = Circle(Float) | Rect(Float, Float)

fun area(s: Shape) -> Float | pure = {
    1
}
```

into `AlephTree::TypeDef`/`Typed`/`WithEffects` nodes (see
[`aleph-syntax-tree`](https://github.com/aleph-lang/aleph-syntax-tree)'s
`types`/`effects` modules). Two things worth knowing:

- **`type` is now a reserved keyword.** Code that previously used `type` as
  a plain identifier will no longer parse (this was already true of
  `fun`/`match`/`import`/etc.; `type` now joins that list).
- **An unrecognized effect name panics** rather than returning a parse
  error (e.g. `fun f() -> Unit | typo = { ... }`) — `parse()` doesn't yet
  have the fallible-action plumbing to turn this into a proper `Result`.
  Not a concern for well-formed input; do not feed this parser untrusted
  effect names without a wrapping `catch_unwind` for now.

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
