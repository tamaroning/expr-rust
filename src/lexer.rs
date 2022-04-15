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
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Ident {
    pub sym: String,
}

pub fn lexer() -> impl Parser<char, Vec<Token>, Error = Simple<char>> {
    let tk_misc = {
        use Token::*;
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
        use Token::*;
        just("fn").to(Fn).or(just("let").to(Let))
    };

    let tk_ident = text::ident()
        .padded()
        .map(|sym| Token::Ident(Ident { sym }));

    let tk_num = text::int(10).map(|s: String| Token::Num(s.parse().unwrap()));

    let token = tk_misc.or(tk_keyword).or(tk_ident).or(tk_num);

    token.padded().repeated()
}
