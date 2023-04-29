use aleph_syntax_tree::syntax::AlephTree as at;

#[rust_sitter::grammar("ale")]
pub mod grammar {

    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Expression {
        SE(Box<SimplExpr>),
        U(Box<Unary>),
        #[rust_sitter::prec_left(1)]
        Not(
            #[rust_sitter::leaf(text = "!")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        And(
            Box<Expression>,
            #[rust_sitter::leaf(text = "&")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(2)]
        Or(
            Box<Expression>,
            Box<OrRight>,
        ),
        #[rust_sitter::prec_left(1)]
        Length(
            Box<OrRight>,
            #[rust_sitter::leaf(text = "|")] (),
        ),
        #[rust_sitter::prec_left(2)]
        EQ(
            Box<Expression>,
            #[rust_sitter::leaf(text = "==")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        GToE(
            Box<Expression>,
            Box<GToERight>,
        ),
        #[rust_sitter::prec_left(1)]
        LToE(
            Box<Expression>,
            #[rust_sitter::leaf(text = "<")] (),
            Box<LoGToERight>,
        ),
        #[rust_sitter::prec_left(2)]
        Add(
            Box<Expression>,
            #[rust_sitter::leaf(text = "+")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(2)]
        Sub(
            Box<Expression>,
            Box<Unary>,
        ),
        #[rust_sitter::prec_left(3)]
        Mul(
            Box<Expression>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(3)]
        Div(
            Box<Expression>,
            #[rust_sitter::leaf(text = "/")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        If(
            Box<Condition>,
            Box<SimplExpr>,
        ),
        #[rust_sitter::prec_left(1)]
        IfElse(
            Box<Condition>,
            Box<SimplExpr>,
            #[rust_sitter::leaf(text = ":")] (),
            Box<SimplExpr>,
        ),
        #[rust_sitter::prec_left(2)]
        While(
            Box<Condition>,
            #[rust_sitter::leaf(text = "*")] (),
            Box<SimplExpr>,
        ),
        #[rust_sitter::prec_left(1)]
        Import(
            #[rust_sitter::leaf(text = "import")] (),
            Box<SimplExpr>,
        ),
        #[rust_sitter::prec_left(3)]
        IdentConst(
            Box<SimplExpr>,
            Box<IdentSucc>,
        ),
        #[rust_sitter::prec_left(3)]
        LetRec(
            #[rust_sitter::leaf(text = "fun")] (),
            Box<SimplExpr>,
            Box<IdentSucc>,
            #[rust_sitter::leaf(pattern = r"=\s*\{")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
        #[rust_sitter::prec_left(3)]
        Class(
            #[rust_sitter::leaf(text = "class")] (),
            Box<SimplExpr>,
            #[rust_sitter::leaf(text = "{")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
        #[rust_sitter::prec_left(2)]
        Stmts(
            Box<Expression>,
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(1)]
        Unit(
            #[rust_sitter::leaf(text = ";")] (),
        ),
        #[rust_sitter::prec_left(1)]
        Tuple(
            Box<Tuple>,
        ),
        #[rust_sitter::prec_left(1)]
        Array(
            Box<Array>,
        ),
        #[rust_sitter::prec_left(1)]
        Match(
            #[rust_sitter::leaf(text = "match")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "with")] (),
            Box<MatchList>,
        ),
    }

    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum SimplExpr {
        Bool(#[rust_sitter::leaf(pattern = "true|false", transform = |v| v.parse().unwrap())] bool),
        Integer(#[rust_sitter::leaf(pattern = r"\d+", transform = |v| v.parse().unwrap())] i64),
        Float(#[rust_sitter::leaf(pattern = r"\d*\.\d*",transform = |v| v.parse().unwrap())] f64),
        String(#[rust_sitter::leaf(pattern = r#""[^\"]*""#, transform = |v| v.parse().unwrap())] String),
        Ident(#[rust_sitter::leaf(pattern = r"[a-z](\d|[A-Za-z]|_)*", transform = |v| v.parse().unwrap())] String),
        #[rust_sitter::prec_left(1)]
        LBexpRB(
            #[rust_sitter::leaf(text = "{")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "}")] (),
        ),
        Comment(#[rust_sitter::leaf(pattern = r#"//([^\n]*)\n"#, transform = |v| v.parse().unwrap())] String),
        CommentMulti(#[rust_sitter::leaf(pattern = r#"/\*([^\*/]*)\*/"#, transform = |v| v.parse().unwrap())] String),
    }
 
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum OrRight {
        #[rust_sitter::prec_left(11)]
        OrRight(
            #[rust_sitter::leaf(text = "|")] (),
            Box<Expression>,
        ),
    }

    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum GToERight {
        #[rust_sitter::prec_left(1)]
        GToERight(
            #[rust_sitter::leaf(text = ">")] (),
            Box<LoGToERight>,
        ),
    }

    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum LoGToERight {
        #[rust_sitter::prec_left(1)]
        LoGERight(
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
        LoGTRight(
            Box<Expression>,
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Condition {
        #[rust_sitter::prec_left(1)]
        Simpl(
            Box<Tuple>,
            #[rust_sitter::leaf(text = "?")] (),
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Unary {
        #[rust_sitter::prec_left(1)]
        Neg(
            #[rust_sitter::leaf(text = "-")] (),
            Box<NegRight>,
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum NegRight {
        #[rust_sitter::prec_left(2)]
        CaseExpr(
            Box<GToERight>,
        ),
        #[rust_sitter::prec_left(1)]
        Expr(
            Box<Expression>,
        ),
    }
        
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Tuple {
        #[rust_sitter::prec_left(1)]
        Elems(
           #[rust_sitter::leaf(text = "(")] (),
           Box<ExprList>,
           #[rust_sitter::leaf(text = ")")] (),
        ),
        #[rust_sitter::prec_left(10)]
        Empty(
           #[rust_sitter::leaf(text = "()")] (),
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum Array {
        #[rust_sitter::prec_left(1)]
        Elems(
           #[rust_sitter::leaf(text = "[")] (),
           Box<ExprList>,
           #[rust_sitter::leaf(text = "]")] (),
        ),
        #[rust_sitter::prec_left(10)]
        Empty(
           #[rust_sitter::leaf(text = "[]")] (),
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub struct ExprList {
        #[rust_sitter::repeat(non_empty = true)]
        #[rust_sitter::delimited(#[rust_sitter::leaf(text = ",")] ())]
        pub elems: Vec<Expression>,
    }
   
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum MatchList {
        #[rust_sitter::prec_left(1)]
        Node(
            Box<MatchList>,
            Box<MatchLine>,
        ),
        #[rust_sitter::prec_left(10)]
        Leaf(
            Box<MatchLine>,
        ),
    }

    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum MatchLine {
        #[rust_sitter::prec_left(10)]
        Line(
            #[rust_sitter::leaf(text = ":")] (),
            Box<Expression>,
            Box<Unary>,
            #[rust_sitter::leaf(text = ":")] (),
        ),
    }
 
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum IdentSucc {
        Let(
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
        LetP(
            #[rust_sitter::leaf(text = ":=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(10)]
        App(
            Box<Tuple>,
        ),
        #[rust_sitter::prec_left(10)]
        ClsApp(
            #[rust_sitter::leaf(text = ".")] (),
            Box<SimplExpr>,
            Box<Tuple>,
        ),
        #[rust_sitter::prec_left(10)]
        Get(
            Box<IdentSuccLeft>,
            #[rust_sitter::leaf(text = "]")] (),
        ),
        #[rust_sitter::prec_left(10)]
        Put(
            Box<IdentSuccLeft>,
            #[rust_sitter::leaf(pattern = r"]\s*=")] (),
            Box<PutRight>,
        ),
        #[rust_sitter::prec_left(10)]
        PutInsert(
            Box<IdentSuccLeft>,
            #[rust_sitter::leaf(pattern = r"\+]\s*=")] (),
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(10)]
        RemovePos(
            #[rust_sitter::leaf(text = "[/")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "]")] (),
        ),
        #[rust_sitter::prec_left(10)]
        RemoveVal(
            #[rust_sitter::leaf(text = "[-")] (),
            Box<Expression>,
            #[rust_sitter::leaf(text = "]")] (),
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum IdentSuccLeft {
        Get(
            #[rust_sitter::leaf(text = "[")] (),
            Box<Expression>,
        ),
    }
    
    #[rust_sitter::language]
    #[derive(Debug)]
    pub enum PutRight {
        Put(
            Box<Expression>,
        ),
        #[rust_sitter::prec_left(2)]
        EQ(
            #[rust_sitter::leaf(text = "=")] (),
            Box<Expression>,
        ),
    }

    #[rust_sitter::extra]
    #[derive(Debug)]
    struct Whitespace {
        #[rust_sitter::leaf(pattern = r"\s")]
        _whitespace: (),
    }

}

fn translate_simple_expr(tree: grammar::SimplExpr) -> at {
    match tree {
        grammar::SimplExpr::Bool(b) => at::Bool{value: b.to_string()},
        grammar::SimplExpr::LBexpRB(_,e,_) => translate(*e),
        grammar::SimplExpr::Integer(i) => at::Int{value: i.to_string()},
        grammar::SimplExpr::Float(f) => at::Float{value: f.to_string()},
        grammar::SimplExpr::String(s) => at::String{value: s},
        grammar::SimplExpr::Ident(s) => at::String{value: s},
        grammar::SimplExpr::Comment(s) => at::Comment{value: s}, 
        grammar::SimplExpr::CommentMulti(s) => at::CommentMulti{value: s},
    }
}

fn translate_se_string(tree: grammar::SimplExpr) -> String {
    match tree {
        grammar::SimplExpr::Bool(b) => b.to_string(),
        grammar::SimplExpr::LBexpRB(_,_,_) => "".to_string(),
        grammar::SimplExpr::Integer(i) => i.to_string(),
        grammar::SimplExpr::Float(f) => f.to_string(),
        grammar::SimplExpr::String(s) => s,
        grammar::SimplExpr::Ident(s) => s,
        grammar::SimplExpr::Comment(_) => "".to_string(), 
        grammar::SimplExpr::CommentMulti(_) => "".to_string(),
    }
}

fn translate_unary(tree: grammar::Unary, is_num: bool) -> at {
    match tree {
        grammar::Unary::Neg(_, nr) => match is_num {
            true => at::Neg{expr: Box::new(translate_neg_right(*nr))},
            false => translate_neg_right(*nr),
        },
    }
}

fn translate_neg_right(tree: grammar::NegRight) -> at {
    match tree {
        grammar::NegRight::CaseExpr(gtr) => match *gtr {
            grammar::GToERight::GToERight(_, logr) => match *logr {
                 grammar::LoGToERight::LoGERight(_, e) => translate(*e),
                 grammar::LoGToERight::LoGTRight(e) => translate(*e),
            },
        },
        grammar::NegRight::Expr(e) => translate(*e),
    }
}

fn translate_cond(tree: grammar::Condition) -> Box<at> {
    match tree {
        grammar::Condition::Simpl(t, _) => {
            let v = translate_tuple(*t);
            if v.is_empty() {
                Box::new(at::Unit{})
            } else { 
                v[0].clone()
            } 
        }
    }
}

fn translate_list(list: grammar::ExprList) -> Vec<Box<at>> {
    list.elems.into_iter().map(|e| Box::new(translate(e))).collect::<Vec<Box<at>>>()
}

fn translate_tuple(list: grammar::Tuple) -> Vec<Box<at>> {
    match list {
       grammar::Tuple::Empty(_) => Vec::new(),
       grammar::Tuple::Elems(_, l, _) => translate_list(*l),
    }
}

fn translate_array(list: grammar::Array) -> Vec<Box<at>> {
    match list {
       grammar::Array::Empty(_) => Vec::new(),
       grammar::Array::Elems(_, l, _) => translate_list(*l),
    }
}

fn get_expression_orr(tree: grammar::OrRight) -> grammar::Expression {
    match tree {
        grammar::OrRight::OrRight(_, e) => *e,
    }
}

fn translate_ident_succ_left(tree: grammar::IdentSuccLeft) -> grammar::Expression {
    match tree {
        grammar::IdentSuccLeft::Get(_, e) => *e,
    }
}

fn translate_match(tree: grammar::Expression, ml: grammar::MatchList) -> at {
   at::Match{expr: Box::new(translate(tree)), case_list: translate_match_list(ml)}
}

fn translate_match_list(list: grammar::MatchList) -> Vec<Box<at>> {
    match list {
        grammar::MatchList::Leaf(e) => vec![Box::new(translate_match_line(*e))],
        grammar::MatchList::Node(l, e) => {
            let mut v = translate_match_list(*l);
            v.push(Box::new(translate_match_line(*e)));
            v
        },
    }
}

fn translate_match_line(tree: grammar::MatchLine) -> at {
    match tree {
        grammar::MatchLine::Line(_, cond, u, _) => at::MatchLine{condition: Box::new(translate(*cond)), case_expr: Box::new(translate_unary(*u, false))},
    }
}

fn translate_gtr(ast: at, is_less: bool, tree: grammar::GToERight) -> at {
    match tree {
        grammar::GToERight::GToERight(_, logr) => translate_logtoe(ast, is_less, *logr),
    }
}

fn translate_logtoe(ast: at, is_less: bool, logr: grammar::LoGToERight) -> at {
   match logr {
        grammar::LoGToERight::LoGERight(_, e) => match is_less {
            true => at::LE{expr1: Box::new(ast), expr2: Box::new(translate(*e))}, 
            _ => at::LE{expr2: Box::new(ast), expr1: Box::new(translate(*e))},
        },
        grammar::LoGToERight::LoGTRight(e) => match is_less {
            true => at::Not{bool_expr: Box::new(at::LE{expr2: Box::new(ast), expr1: Box::new(translate(*e))})}, 
            _ => at::Not{bool_expr: Box::new(at::LE{expr1: Box::new(ast), expr2: Box::new(translate(*e))})},
        },
   }
}


fn translate_let(ident: grammar::SimplExpr, tree: grammar::Expression, is_pointer: bool) -> at {
    match tree {
        grammar::Expression::LToE(e1, _, ltr) => match *ltr {
            grammar::LoGToERight::LoGTRight(e) => match *e1 {
                grammar::Expression::Stmts(e11, e12) => match *e11 {
                    grammar::Expression::Stmts(e111, _e112) => match *e { 
                         grammar::Expression::Stmts(e1111, e1112) => at::Let{var: translate_se_string(ident), is_pointer: is_pointer.to_string(), value: Box::new(translate(*e111)), expr: Box::new(at::Stmts{expr1: Box::new(at::Not{bool_expr: Box::new(at::LE{expr2: Box::new(translate(*e12)), expr1: Box::new(translate(*e1111))})}), expr2: Box::new(translate(*e1112))})},
                         _ => at::Unit{},
                    },
                    _ => at::Unit{},
                },
                _ => at::Unit{},
            },
            grammar::LoGToERight::LoGERight(_, e) => match *e1 {
                grammar::Expression::Stmts(e11, e12) => match *e11 {
                    grammar::Expression::Stmts(e111, _e112) => match *e { 
                         grammar::Expression::Stmts(e1111, e1112) => at::Let{var: translate_se_string(ident), is_pointer: is_pointer.to_string(), value: Box::new(translate(*e111)), expr: Box::new(at::Stmts{expr1: Box::new(at::LE{expr1: Box::new(translate(*e12)), expr2: Box::new(translate(*e1111))}), expr2: Box::new(translate(*e1112))})},
                         _ => at::Unit{},
                    },
                    _ => at::Unit{},
                },
                _ => at::Unit{},
            },
        },
        grammar::Expression::GToE(e1, gtr) => match *gtr {
            grammar::GToERight::GToERight(_, logr) => match *logr {
                    grammar::LoGToERight::LoGTRight(e) => match *e1 {
                        grammar::Expression::Stmts(e11, e12) => match *e11 {
                            grammar::Expression::Stmts(e111, _e112) => match *e { 
                                grammar::Expression::Stmts(e1111, e1112) => at::Let{var: translate_se_string(ident), is_pointer: is_pointer.to_string(), value: Box::new(translate(*e111)), expr: Box::new(at::Stmts{expr1: Box::new(at::Not{bool_expr: Box::new(at::LE{expr1: Box::new(translate(*e12)), expr2: Box::new(translate(*e1111))})}), expr2: Box::new(translate(*e1112))})},
                                _ => at::Unit{},
                            },
                            _ => at::Unit{},
                        }
                        ne1 => translate(ne1),
                    },
                    grammar::LoGToERight::LoGERight(_, e) => match *e1 {
                        grammar::Expression::Stmts(e11, e12) => match *e11 {
                            grammar::Expression::Stmts(e111, _e112) => match *e { 
                               grammar::Expression::Stmts(e1111, e1112) => at::Let{var: translate_se_string(ident), is_pointer: is_pointer.to_string(), value: Box::new(translate(*e111)), expr: Box::new(at::Stmts{expr1: Box::new(at::LE{expr2: Box::new(translate(*e12)), expr1: Box::new(translate(*e1111))}), expr2: Box::new(translate(*e1112))})},
                                _ => at::Unit{},
                            },
                            _ => at::Unit{},
                        },
                        _ => at::Unit{},
                    },
            },
        },
        v => at::Let{var: translate_se_string(ident), is_pointer: is_pointer.to_string(), value: Box::new(translate(v)), expr: Box::new(at::Unit{})},
    }
}


fn translate(tree : grammar::Expression) -> at {
    match tree {
        grammar::Expression::Unit(_) => at::Unit{},
        grammar::Expression::SE(se) => translate_simple_expr(*se),
        grammar::Expression::U(u) => translate_unary(*u, true),
        grammar::Expression::Not(_, e) => at::Not{bool_expr: Box::new(translate(*e))},
        grammar::Expression::And(e1,_, e2) => at::And{bool_expr1 : Box::new(translate(*e1)), bool_expr2: Box::new(translate(*e2))},
        grammar::Expression::Or(e1, orr) => at::Or{bool_expr1 : Box::new(translate(*e1)), bool_expr2: Box::new(translate(get_expression_orr(*orr)))},
        grammar::Expression::EQ(e1, _, e2) => at::Eq{expr1 : Box::new(translate(*e1)), expr2: Box::new(translate(*e2))},
        grammar::Expression::LToE(e1,_, ltr) => translate_logtoe(translate(*e1), true, *ltr),
        grammar::Expression::GToE(e1, gtr) => translate_gtr(translate(*e1), false, *gtr),
        grammar::Expression::Add(e1, _, e2) => at::Add{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
        grammar::Expression::Sub(e1,u2) => match *u2 {
            grammar::Unary::Neg(_, e2) => match *e2 {
                grammar::NegRight::Expr(e) => match *e {
                    grammar::Expression::Stmts(e21, e22) => at::Stmts{expr1: Box::new(at::Sub{number_expr1: Box::new(translate(*e1)), number_expr2: Box::new(translate(*e21))}), expr2: Box::new(translate(*e22))},
                    e21 => at::Sub{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(e21))},
                },
                e21 => at::Sub{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate_neg_right(e21))},
            },
        },
        grammar::Expression::Mul(e1,_, e2) => at::Mul{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))},
        grammar::Expression::Div(e1,_, e2) => at::Div{number_expr1 : Box::new(translate(*e1)), number_expr2: Box::new(translate(*e2))}, 
        grammar::Expression::If(cond, then) => at::If{condition: translate_cond(*cond), then: Box::new(translate_simple_expr(*then)), els: Box::new(at::Unit
{})}, 
        grammar::Expression::IfElse(cond, then, _, els) => at::If{condition: translate_cond(*cond), then: Box::new(translate_simple_expr(*then)), els: Box::new(translate_simple_expr(*els))}, 
        grammar::Expression::While(cond, _, e2) => match *translate_cond(*cond) {
          at::Let{var, is_pointer, value, expr} => match *expr {
              at::Stmts{expr1, expr2} => at::While{init_expr: Box::new(at::Let{var: var, is_pointer: is_pointer, value: value, expr: Box::new(at::Unit{})}), condition: expr1, loop_expr: Box::new(translate_simple_expr(*e2)), post_expr: expr2},
              c =>  at::While{init_expr: Box::new(at::Unit{}), condition: Box::new(c), loop_expr: Box::new(translate_simple_expr(*e2)), post_expr: Box::new(at::Unit{})}, 
          },  
          c =>  at::While{init_expr: Box::new(at::Unit{}), condition: Box::new(c), loop_expr: Box::new(translate_simple_expr(*e2)), post_expr: Box::new(at::Unit{})}, 
        },
        grammar::Expression::Import(_, name) => at::Iprt{name: translate_se_string(*name)}, 
        grammar::Expression::IdentConst(ident, succ) => match *succ {
            grammar::IdentSucc::Let(_, value) => translate_let(*ident, *value, false),
            grammar::IdentSucc::LetP(_, value) => translate_let(*ident, *value, true),
            grammar::IdentSucc::App(param_list) => at::App{object_name: "".to_string(), fun: Box::new(at::String{value: translate_se_string(*ident)}), param_list: translate_tuple(*param_list)},
            grammar::IdentSucc::ClsApp(_, id_fun, param_list) => at::App{object_name: translate_se_string(*ident), fun: Box::new(at::String{value: translate_se_string(*id_fun)}), param_list: translate_tuple(*param_list)},
            grammar::IdentSucc::Get(isl, _) => at::Get{array_name: translate_se_string(*ident), elem: Box::new(translate(translate_ident_succ_left(*isl)))},
            grammar::IdentSucc::Put(isl, _, pr) => match *pr {
                grammar::PutRight::Put(v) => at::Put{array_name: translate_se_string(*ident), elem: Box::new(translate(translate_ident_succ_left(*isl))), value: Box::new(translate(*v)), insert: "false".to_string()},
                grammar::PutRight::EQ(_, v) => at::Eq{expr1 : Box::new(at::Get{array_name: translate_se_string(*ident), elem: Box::new(translate(translate_ident_succ_left(*isl)))}), expr2: Box::new(translate(*v))},
            },
            grammar::IdentSucc::PutInsert(isl, _, v) => at::Put{array_name: translate_se_string(*ident), elem: Box::new(translate(translate_ident_succ_left(*isl))), value: Box::new(translate(*v)), insert: "true".to_string()},
            grammar::IdentSucc::RemovePos(_, e, _) => at::Remove{array_name: translate_se_string(*ident), elem: Box::new(translate(*e)), is_value: "false".to_string()},
            grammar::IdentSucc::RemoveVal(_, e, _) => at::Remove{array_name: translate_se_string(*ident), elem: Box::new(translate(*e)), is_value: "true".to_string()},
        } 
        grammar::Expression::Tuple(list) => at::Tuple{elems: translate_tuple(*list)}, 
        grammar::Expression::Array(list) => at::Array{elems: translate_array(*list)}, 
        grammar::Expression::Stmts(e1, e2) => match (*e1, *e2) {
             (grammar::Expression::Unit(_), grammar::Expression::Unit(_)) => at::Unit{},
             (grammar::Expression::Unit(_), e21) => translate(e21),
             (e11, grammar::Expression::Unit(_)) => translate(e11),
             (e11, e21) => at::Stmts{expr1: Box::new(translate(e11)), expr2: Box::new(translate(e21))}, 
        },
        grammar::Expression::Length(orr, _) => match get_expression_orr(*orr) {
            grammar::Expression::SE(se) => at::Length{var: translate_se_string(*se)},
            _ => { println!("Error parsing Length!"); at::Unit{}},
        },
        grammar::Expression::Match(_, e, _, ml) => translate_match(*e, *ml), 
        grammar::Expression::LetRec(_, name, args, _, body, _) => match *args {
           grammar::IdentSucc::App(param_list) => at::LetRec{name: translate_se_string(*name), args: translate_tuple(*param_list), body: Box::new(translate(*body))},
           _ => at::LetRec{name: translate_se_string(*name), args: Vec::new(), body: Box::new(translate(*body))},
        }, 
        grammar::Expression::Class(_, name, _, body, _) => at::Clss{name: translate_se_string(*name), attribute_list: Vec::new(), body: Box::new(translate(*body))}, 
    }
}


/// Aleph parser
/// #Arguments
/// `source` - String to parse
///
/// # Return
/// This function return an AlephTree
pub fn parse(source: String) -> at {
    translate(grammar::parse(&source).unwrap())
}
