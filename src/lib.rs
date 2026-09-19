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
}
