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
