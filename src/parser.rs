use crate::ast::*;
use crate::lexer::{Ident, TokenKind};
use crate::ty::Type;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<char, Program, Error = Simple<char>> {
    // tokenizer
    let tk_misc = {
        use TokenKind::*;
        just("(")
            .to(OpenParen)
            .or(just(")").to(CloseParen))
            .or(just("{").to(OpenBraces))
            .or(just("}").to(CloseBraces))
            .or(just(";").to(Semi))
            .or(just(":").to(Colon))
            .or(just("->").to(Arrow))
            .or(just("=").to(Eq))
            .or(just("+").to(Plus))
            .or(just("-").to(Minus))
            .or(just("*").to(Asterisk))
            .or(just("/").to(Slash))
    };

    let tk_keyword = {
        use TokenKind::*;
        just("fn").to(Fn).or(just("let").to(Let))
    };

    let tk_ident = text::ident()
        .padded()
        .map(|sym| TokenKind::Ident(Ident { sym }));

    let tk_num = text::int(10).map(|s: String| TokenKind::Num(s.parse().unwrap()));

    let token = tk_misc.or(tk_keyword).or(tk_ident).or(tk_num);

    // parser
    let num = tk_num.map(|TokenKind::Num(n)| n);
    let ident = tk_ident.map(|TokenKind::Ident(ident)| ident);

    let lit_expr_num = num.map(|n| LitExpr::Num(n));
    let lit_expr = lit_expr_num.map(|lit_expr| Expr::LitExpr(lit_expr));

    let ident_expr = tk_ident.map(|TokenKind::Num(ident)| Expr::IdentExpr(ident));

    let expr = ident_expr.or(lit_expr);

    let func = just(TokenKind::Fn)
        .ignore_then(tk_ident)
        .then_ignore(just(TokenKind::OpenParen))
        .then_ignore(just(TokenKind::CloseParen))
        .then_ignore(just(TokenKind::OpenBraces))
        .then(expr)
        .then_ignore(just(TokenKind::CloseBraces))
        .map(|(name, body)| Func {
            name,
            args: Vec::new(),
            ret_ty: Type::Unit, // TODO: support `-> Type`
            body: Box::new(body),
        });

    let item = func.map(|must_be_func| Item::Func(must_be_func));

    let program = item.repeated().map(|items| Program { items });

    program.then_ignore(end())
}
