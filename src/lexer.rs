use chumsky::prelude::*;

#[derive(Debug, Clone, Hash)]
pub enum TokenKind {
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

#[derive(Debug, Clone, Hash)]
pub struct Ident {
    pub sym: String,
}

pub fn lexer() -> impl Parser<char, Vec<TokenKind>, Error = Simple<char>> {
    let misc = {
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

    let keyword = {
        use TokenKind::*;
        just("fn").to(Fn).or(just("let").to(Let))
    };

    let ident = text::ident()
        .padded()
        .map(|sym| TokenKind::Ident(Ident { sym }));

    let num = text::int(10).map(|s: String| TokenKind::Num(s.parse().unwrap()));

    let token = misc.or(keyword).or(ident).or(num);

    token.padded().repeated().then_ignore(end())
}
