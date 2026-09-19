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
}
