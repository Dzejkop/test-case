pub mod expr_case;
pub mod ignore_case;
pub mod panic_case;
pub mod pattern_case;

use crate::expected::expr_case::ExprCase;
use crate::expected::ignore_case::IgnoreCase;
use crate::expected::panic_case::PanicCase;
use crate::expected::pattern_case::PatternCase;
use std::fmt;
use syn::parse::{Parse, ParseStream};
use syn::{Attribute, Error, Expr, LitStr, Pat};

mod kw {
    syn::custom_keyword!(matches);
    syn::custom_keyword!(panics);
    syn::custom_keyword!(inconclusive);
}

#[cfg_attr(test, derive(Debug, PartialEq))]
pub enum Expected {
    Pattern(PatternCase),
    Panic(PanicCase),
    Ignore(IgnoreCase),
    Expr(ExprCase),
}

pub trait Case {
    fn body(&self) -> Option<Expr>;
    fn attr(&self) -> Option<Attribute>;
}

impl fmt::Display for Expected {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expected::Pattern(v) => write!(f, "{}", v),
            Expected::Panic(v) => write!(f, "{}", v),
            Expected::Ignore(v) => write!(f, "{}", v),
            Expected::Expr(v) => write!(f, "{}", v),
        }
    }
}

impl Parse for Expected {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let lookahead = input.lookahead1();

        if lookahead.peek(kw::matches) {
            let _kw = input.parse::<kw::matches>()?;
            return Ok(Expected::new_pattern(input.parse()?));
        }

        if lookahead.peek(kw::panics) {
            let _kw = input.parse::<kw::panics>()?;
            return Ok(Expected::new_panic(input.parse()?));
        }

        if lookahead.peek(kw::inconclusive) {
            let _kw = input.parse::<kw::inconclusive>()?;
            return Ok(Expected::new_ignore(input.parse()?));
        }

        Ok(Expected::new_expr(input.parse()?))
    }
}

impl Expected {
    pub fn new_pattern(pat: Pat) -> Self {
        Expected::Pattern(PatternCase::new(pat))
    }

    pub fn new_panic(lit_str: LitStr) -> Self {
        Expected::Panic(PanicCase::new(lit_str))
    }

    pub fn new_ignore(expr: Box<Expr>) -> Self {
        Expected::Ignore(IgnoreCase::new(expr))
    }

    pub fn new_expr(expr: Box<Expr>) -> Self {
        Expected::Expr(ExprCase::new(expr))
    }

    pub fn case(&self) -> &dyn Case {
        match self {
            Expected::Pattern(e) => e,
            Expected::Panic(e) => e,
            Expected::Ignore(e) => e,
            Expected::Expr(e) => e,
        }
    }
}