use crate::ast::*;
use crate::lexer::Token;
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
        // Terminators
        let lit_expr = {
            let lit_expr_num = num.map(|n| LitExpr::Num(n));
            lit_expr_num.map(|lit_expr| Expr::new(ExprKind::LitExpr(lit_expr)))
        };
        let ident_expr = ident.map(|ident| Expr::new(ExprKind::IdentExpr(ident)));

        let terminator_expr = ident_expr
            .clone()
            .or(lit_expr.clone())
            .map(|expr| Box::new(expr));

        // Non-terminators
        let binary_op_expr = {
            let binary_op = choice((
                just(Token::Plus),
                just(Token::Minus),
                just(Token::Asterisk),
                just(Token::Slash),
            ));

            terminator_expr
                .clone()
                .then(binary_op)
                .then(expr)
                .map(|((lhs, op), rhs)| {
                    let kind = ExprKind::BinaryOpExpr(match op {
                        Token::Plus => BinaryOpExpr::Add(lhs, rhs),
                        Token::Minus => BinaryOpExpr::Sub(lhs, rhs),
                        Token::Asterisk => BinaryOpExpr::Mul(lhs, rhs),
                        Token::Slash => BinaryOpExpr::Div(lhs, rhs),
                        _ => unreachable!(""),
                    });
                    Expr::new(kind)
                })
        };

        choice((binary_op_expr, ident_expr, lit_expr)).map(|expr| Box::new(expr))
    });

    let ty = choice((
        just(Token::I32).to(Type::I32),
        just(Token::OpenParen)
            .then(just(Token::CloseParen))
            .to(Type::Unit),
    ));

    let func = {
        choice((
            just(Token::Fn)
                .ignore_then(ident)
                .then_ignore(just(Token::OpenParen))
                .then_ignore(just(Token::CloseParen))
                .then_ignore(just(Token::Arrow))
                .then(ty)
                .then_ignore(just(Token::OpenBraces))
                .then(expr.clone())
                .then_ignore(just(Token::CloseBraces))
                .map(|((name, ret_ty), body)| Func {
                    name,
                    args: Vec::new(),
                    ret_ty,
                    body,
                    ty: Type::Unresolved,
                }),
            just(Token::Fn)
                .ignore_then(ident)
                .then_ignore(just(Token::OpenParen))
                .then_ignore(just(Token::CloseParen))
                .then_ignore(just(Token::OpenBraces))
                .then(expr)
                .then_ignore(just(Token::CloseBraces))
                .map(|(name, body)| Func {
                    name,
                    args: Vec::new(),
                    ret_ty: Type::Unit,
                    body,
                    ty: Type::Unresolved,
                }),
        ))
    };

    let item = func.map(|func| Item::Func(func));

    let program = item.repeated().map(|items| Program { items });

    program.then_ignore(end())
}
