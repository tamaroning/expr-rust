use chumsky::prelude::*;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Token {
    // (
    OpenParen,
    // )
    CloseParen,
    // {
    OpenBraces,
    // }
    CloseBraces,
    // ;
    Semi,
    // :
    Colon,
    // ->
    Arrow,
    // =
    Eq,
    // +
    Plus,
    // -
    Minus,
    // *
    Asterisk,
    // /
    Slash,
    // identifier
    Ident(Ident),
    // number
    Num(i32),

    //Keywords
    // fn
    Fn,
    // let
    Let,
    // i32
    I32,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Ident {
    pub sym: String,
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let tk_misc = {
        use Token::*;
        choice((
            just("(").to(OpenParen),
            just(")").to(CloseParen),
            just("{").to(OpenBraces),
            just("}").to(CloseBraces),
            just(";").to(Semi),
            just(":").to(Colon),
            just("->").to(Arrow),
            just("=").to(Eq),
            just("+").to(Plus),
            just("-").to(Minus),
            just("*").to(Asterisk),
            just("/").to(Slash),
        ))
    };

    let tk_keyword = {
        use Token::*;
        choice((just("fn").to(Fn), just("let").to(Let), just("i32").to(I32)))
    };

    let tk_ident = text::ident()
        .padded()
        .map(|sym| Token::Ident(Ident { sym }));

    let tk_num = text::int(10).map(|s: String| Token::Num(s.parse().unwrap()));

    let token = tk_misc.or(tk_keyword).or(tk_ident).or(tk_num);

    token.padded().repeated()
}
