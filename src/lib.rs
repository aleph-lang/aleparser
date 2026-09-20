use lalrpop_util::lalrpop_mod;
use aleph_syntax_tree::syntax::AlephTree as at;

lalrpop_mod!(pub grammar);

/// Aleph parser
/// #Arguments
/// `source` - String to parse
///
/// # Return
/// This function return an AlephTree
pub fn parse(source: String) -> at {
    let ast = grammar::StmtsParser::new().parse(&source);
    match ast {
        Ok(res) => res,
        Err(e) => {
            println!("Can't parse {:?}", e);
            at::Unit
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aleph_syntax_tree::syntax::AlephTree as at;
    use aleph_syntax_tree::types::{Type, Variant};

    #[test]
    fn parses_a_sum_type_declaration() {
        let result = parse("type Shape = Circle(Float) | Rect(Float, Float)".to_string());
        assert_eq!(result, at::TypeDef {
            name: "Shape".to_string(),
            variants: vec![
                Variant { name: "Circle".to_string(), fields: vec![Type::Float] },
                Variant { name: "Rect".to_string(), fields: vec![Type::Float, Type::Float] },
            ],
        });
    }

    #[test]
    fn parses_a_zero_field_variant() {
        let result = parse("type Bool2 = T | F".to_string());
        assert_eq!(result, at::TypeDef {
            name: "Bool2".to_string(),
            variants: vec![
                Variant { name: "T".to_string(), fields: vec![] },
                Variant { name: "F".to_string(), fields: vec![] },
            ],
        });
    }

    #[test]
    fn parses_all_primitive_type_keywords() {
        let result = parse("type AllPrimitives = V(Int, Float, Bool, String, Bytes, Unit)".to_string());
        assert_eq!(result, at::TypeDef {
            name: "AllPrimitives".to_string(),
            variants: vec![
                Variant {
                    name: "V".to_string(),
                    fields: vec![Type::Int, Type::Float, Type::Bool, Type::String, Type::Bytes, Type::Unit],
                },
            ],
        });
    }

    #[test]
    fn unrecognized_type_name_falls_back_to_a_type_variable_reference() {
        let result = parse("type Wrapper = Box(Elem)".to_string());
        assert_eq!(result, at::TypeDef {
            name: "Wrapper".to_string(),
            variants: vec![
                Variant { name: "Box".to_string(), fields: vec![Type::Var { name: "Elem".to_string() }] },
            ],
        });
    }

    #[test]
    fn untyped_fun_still_parses_exactly_as_before() {
        let result = parse("fun square(n) = { n * n }".to_string());
        assert_eq!(result, at::LetRec {
            name: "square".to_string(),
            args: vec![Box::new(at::Ident { value: "n".to_string() })],
            body: Box::new(at::Mul {
                number_expr1: Box::new(at::Ident { value: "n".to_string() }),
                number_expr2: Box::new(at::Ident { value: "n".to_string() }),
            }),
        });
    }

    #[test]
    fn parses_a_typed_function_without_effects() {
        let result = parse("fun square(n: Int) -> Int = { n * n }".to_string());
        assert_eq!(result, at::Typed {
            inner: Box::new(at::LetRec {
                name: "square".to_string(),
                args: vec![Box::new(at::Typed {
                    inner: Box::new(at::Ident { value: "n".to_string() }),
                    ty: Type::Int,
                })],
                body: Box::new(at::Mul {
                    number_expr1: Box::new(at::Ident { value: "n".to_string() }),
                    number_expr2: Box::new(at::Ident { value: "n".to_string() }),
                }),
            }),
            ty: Type::Fun { params: vec![Type::Int], ret: Box::new(Type::Int) },
        });
    }

    #[test]
    fn parses_a_typed_function_with_effects() {
        use aleph_syntax_tree::effects::{Effect, EffectSet};
        let result = parse("fun square(n: Int) -> Int | pure = { n * n }".to_string());
        assert_eq!(result, at::WithEffects {
            inner: Box::new(at::Typed {
                inner: Box::new(at::LetRec {
                    name: "square".to_string(),
                    args: vec![Box::new(at::Typed {
                        inner: Box::new(at::Ident { value: "n".to_string() }),
                        ty: Type::Int,
                    })],
                    body: Box::new(at::Mul {
                        number_expr1: Box::new(at::Ident { value: "n".to_string() }),
                        number_expr2: Box::new(at::Ident { value: "n".to_string() }),
                    }),
                }),
                ty: Type::Fun { params: vec![Type::Int], ret: Box::new(Type::Int) },
            }),
            effects: EffectSet::from([Effect::Pure]),
        });
    }

    #[test]
    fn parses_a_zero_arg_typed_function_with_multiple_effects() {
        use aleph_syntax_tree::effects::{Effect, EffectSet};
        let result = parse("fun main() -> Unit | io, net = { 1 }".to_string());
        assert_eq!(result, at::WithEffects {
            inner: Box::new(at::Typed {
                inner: Box::new(at::LetRec {
                    name: "main".to_string(),
                    args: Vec::new(),
                    body: Box::new(at::Int { value: "1".to_string() }),
                }),
                ty: Type::Fun { params: Vec::new(), ret: Box::new(Type::Unit) },
            }),
            effects: EffectSet::from([Effect::Io, Effect::Net]),
        });
    }

    #[test]
    fn parses_the_act_effect_despite_act_also_being_a_cognitive_layer_keyword() {
        // Regression test: `"act"` is ALSO a pre-existing literal keyword
        // token (the cognitive-layer `act <intention>` rule). LALRPOP's
        // lexer always prefers a literal-string token over a same-length
        // regex token (Ident) for identical text, so `act` here would never
        // reach `Ident`'s match arm — EffectName needs its own dedicated
        // `"act" => Effect::Act` alternative, not just a match arm inside
        // the Ident-routed one. If this test ever fails, that's very likely
        // what regressed.
        use aleph_syntax_tree::effects::{Effect, EffectSet};
        let result = parse("fun f() -> Unit | act = { 1 }".to_string());
        assert_eq!(result, at::WithEffects {
            inner: Box::new(at::Typed {
                inner: Box::new(at::LetRec {
                    name: "f".to_string(),
                    args: Vec::new(),
                    body: Box::new(at::Int { value: "1".to_string() }),
                }),
                ty: Type::Fun { params: Vec::new(), ret: Box::new(Type::Unit) },
            }),
            effects: EffectSet::from([Effect::Act]),
        });
    }

    #[test]
    #[should_panic(expected = "unknown effect")]
    fn unrecognized_effect_name_panics_rather_than_silently_misrepresenting_the_signature() {
        parse("fun f() -> Unit | bogus = { 1 }".to_string());
    }

    #[test]
    fn match_arrow_syntax_still_parses_despite_the_new_typed_function_arrow_token() {
        // Regression test: this crate already had a `match e with : cond ->
        // result :` syntax using `-` and `>` as two SEPARATE literal tokens.
        // Adding a single `"->"` literal token for typed-function return
        // types (0.2) means LALRPOP's lexer now always tokenizes an in-source
        // `->` as that one token — the old MatchLine rule (still written as
        // `"-" ">"`) silently stopped matching real `->` input entirely,
        // breaking real-world .ale example files (aleph/test/dataset/ale/
        // testMatch.ale) even though no unit test caught it. Fixed by
        // updating MatchLine's own rule to expect the single "->" token
        // (same surface syntax, no change for anyone writing Aleph code).
        let result = parse("match x with : true -> 1: : false -> 0:".to_string());
        assert_eq!(result, at::Match {
            expr: Box::new(at::Ident { value: "x".to_string() }),
            case_list: vec![
                Box::new(at::MatchLine {
                    condition: Box::new(at::Bool { value: "true".to_string() }),
                    case_expr: Box::new(at::Int { value: "1".to_string() }),
                }),
                Box::new(at::MatchLine {
                    condition: Box::new(at::Bool { value: "false".to_string() }),
                    case_expr: Box::new(at::Int { value: "0".to_string() }),
                }),
            ],
        });
    }

    #[test]
    fn parses_a_typed_function_with_multiple_typed_params() {
        let result = parse("fun add(a: Int, b: Int) -> Int = { a + b }".to_string());
        assert_eq!(result, at::Typed {
            inner: Box::new(at::LetRec {
                name: "add".to_string(),
                args: vec![
                    Box::new(at::Typed { inner: Box::new(at::Ident { value: "a".to_string() }), ty: Type::Int }),
                    Box::new(at::Typed { inner: Box::new(at::Ident { value: "b".to_string() }), ty: Type::Int }),
                ],
                body: Box::new(at::Add {
                    number_expr1: Box::new(at::Ident { value: "a".to_string() }),
                    number_expr2: Box::new(at::Ident { value: "b".to_string() }),
                }),
            }),
            ty: Type::Fun { params: vec![Type::Int, Type::Int], ret: Box::new(Type::Int) },
        });
    }

    #[test]
    fn a_leading_untyped_param_stops_param_types_collection_even_if_later_params_are_typed() {
        // Documents the Fix-1 behavior: param_types is the TYPED PREFIX
        // only. Since `x` (the first param) has no annotation, param_types
        // is empty — even though `y` right after it IS individually typed
        // (y's own Typed{..} wrapping is still present in `args`, just not
        // reflected in the aggregate Fun.params list). This is a deliberate,
        // honest degrade, not a bug: alegen's gen_params will print `x, y:
        // Int` for this — see aleparser's design note on gen_params for why
        // there's no safe way to represent "untyped, then typed" any other
        // way without a placeholder that would itself be misleading.
        let result = parse("fun f(x, y: Int) -> Int = { x }".to_string());
        assert_eq!(result, at::Typed {
            inner: Box::new(at::LetRec {
                name: "f".to_string(),
                args: vec![
                    Box::new(at::Ident { value: "x".to_string() }),
                    Box::new(at::Typed { inner: Box::new(at::Ident { value: "y".to_string() }), ty: Type::Int }),
                ],
                body: Box::new(at::Ident { value: "x".to_string() }),
            }),
            ty: Type::Fun { params: Vec::new(), ret: Box::new(Type::Int) },
        });
    }
}
