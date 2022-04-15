use crate::ast::*;
use crate::lexer::{Ident, Token};
use crate::ty::Type;
use chumsky::prelude::*;

pub fn parser() -> impl Parser<Token, Program, Error = Simple<Token>> {
    let num = filter_map(|span, t| match t {
        Token::Num(n) => Ok(n),
        _ => Err(Simple::custom(span, "")),
    });

    let ident = filter_map(|span, t| match t {
        Token::Ident(ident) => Ok(ident),
        _ => Err(Simple::custom(span, "")),
    });

    let expr = recursive(|expr| {
        let lit_expr = {
            let lit_expr_num = num.map(|n| LitExpr::Num(n));
            lit_expr_num.map(|lit_expr| Expr::new(ExprKind::LitExpr(lit_expr)))
        };

        let ident_expr = ident.map(|ident| Expr::new(ExprKind::IdentExpr(ident)));

        let binary_op_expr = {
            let binary_op = just(Token::Plus)
                .or(just(Token::Minus))
                .or(just(Token::Asterisk))
                .or(just(Token::Slash));

            expr.clone()
                .then(binary_op)
                .then(expr)
                .map(|((lhs, op), rhs)| {
                    let kind = ExprKind::BinaryOpExpr(match op {
                        Token::Plus => BinaryOpExpr::Add(lhs, rhs),
                        Token::Plus => BinaryOpExpr::Sub(lhs, rhs),
                        Token::Plus => BinaryOpExpr::Mul(lhs, rhs),
                        Token::Plus => BinaryOpExpr::Div(lhs, rhs),
                        _ => unreachable!(""),
                    });
                    Expr::new(kind)
                })
        };

        lit_expr
            .or(ident_expr)
            .or(binary_op_expr)
            .map(|expr| Box::new(expr))
    });

    let func = just(Token::Fn)
        .ignore_then(ident)
        .then_ignore(just(Token::OpenParen))
        .then_ignore(just(Token::CloseParen))
        .then_ignore(just(Token::OpenBraces))
        .then(expr)
        .then_ignore(just(Token::CloseBraces))
        .map(|(name, body)| Func {
            name,
            args: Vec::new(),
            ret_ty: Type::Unit, // TODO: support `-> Type`
            body,
        });

    let item = func.map(|func| Item::Func(func));

    let program = item.repeated().map(|items| Program { items });

    program.then_ignore(end())
}
