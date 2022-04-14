use chumsky::prelude::*;

#[derive(Debug)]
struct Program {
    items: Vec<Item>,
}

#[derive(Debug)]
enum Item {
    Func(Func),
}

#[derive(Debug)]
struct Func {
    name: Ident,
    args: Vec<String>,
    body: Box<Expr>,
}

#[derive(Debug)]
enum Expr {
    LitExpr(LitExpr),
    IdentExpr(Ident),
    BinaryOpExpr(BinaryOpExpr),
    UnaryOpExpr(UnaryOpExpr),
    /*
    Call(String, Vec<Expr>),
    Let {
        name: String,
        rhs: Box<Expr>,
        then: Box<Expr>,
    },
    */
}

#[derive(Debug)]
enum LitExpr {
    Num(f64),
}

#[derive(Debug)]
enum UnaryOpExpr {
    Neg(Box<Expr>),
}

#[derive(Debug)]
enum BinaryOpExpr {
    Add(Box<Expr>, Box<Expr>),
    Sub(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
    Div(Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
struct Ident {
    sym: String,
}

fn parser() -> impl Parser<char, Expr, Error = Simple<char>> {
    let open_paren = just("(").padded();
    let close_paren = just(")").padded();
    let open_braces = just("{").padded();
    let close_braces = just("}").padded();
    let arrow = just("->").padded();
    let r#fn = just("fn").padded();

    let ident = text::ident().padded().map(|sym| Ident { sym });

    let num = text::int(10)
        .padded()
        .map(|s: String| LitExpr::Num(s.parse().unwrap()));

    let ident_expr = ident.map(|ident| Expr::IdentExpr(ident));
    let lit_expr = num.map(|lit_expr| Expr::LitExpr(lit_expr));

    let expr = ident_expr.or(lit_expr);

    let func = r#fn
        .ignore_then(ident)
        .then_ignore(open_paren)
        .then_ignore(close_paren)
        .then_ignore(open_braces)
        .then(expr)
        .then_ignore(close_braces)
        .map(|(name, body)| Func {
            name,
            args: Vec::new(),
            body: Box::new(body),
        });

    let item = func.map(|must_be_func| Item::Func(must_be_func));

    let program = item.repeated().map(|items| Program { items });

    program.then_ignore(end())
}

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();

    match parser().parse(src) {
        Ok(ast) => {}
        Err(parse_errs) => parse_errs
            .into_iter()
            .for_each(|e| println!("Parse error: {}", e)),
    }
}
