extern crate proc_macro;
use proc_macro::TokenStream;
use quote::ToTokens;
use syn::Item;

#[proc_macro_attribute]
pub fn kernel(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let ts: proc_macro2::TokenStream = item.clone().into();
    for token in ts {
        println!("{:?}", token);
    }

    item
}

#[derive(Debug, Clone, PartialEq)]
enum Atom<'a> {
    String(std::borrow::Cow<'a, str>),
    Number(f64),
    Nil,
    Bool(bool),
    Ident(&'a str),
    Super,
    This,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Op {
    Minus,
    Plus,
    Star,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Bang,
    And,
    Or,
    Call,
    For,
    Class,
    Print,
    Return,
    Field,
    Var,
    While,
    Group,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenTree<'a> {
    Atom(Atom<'a>),
    Cons(Op, Vec<TokenTree<'a>>),
    Fn {
        name: &'a str,
        parameters: Box<TokenTree<'a>>,
        body: Box<TokenTree<'a>>,
    },
    Call,
    If,
    For,
}

#[derive(Debug, Clone, PartialEq)]
struct Token<'a> {
    token: TokenKind<'a>,
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind<'a> {
    Fn,
    Ident(&'a str),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Star,
    BangEqual,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Bang,
    Equal,
    String,
    Number(f64),
    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}
