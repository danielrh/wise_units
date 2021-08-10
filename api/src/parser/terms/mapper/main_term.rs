use super::{AstTerm, Finishable, Visit};
use crate::{
    invert::Invert,
    parser::{terms::term_parser::Rule, Error, Term},
};
use pest::iterators::Pair;

pub(super) struct MainTerm {
    pub(super) has_leading_slash: bool,
    pub(super) term: AstTerm,
}

impl Visit<Rule> for MainTerm {
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        // match first token
        match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::term => {
                    return Ok(Self {
                        has_leading_slash: false,
                        term: AstTerm::visit(first)?,
                    });
                }
                // Don't do anything, but because we proceed through this method, we can later
                // assume that we parsed a slash.
                Rule::slash => (),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }

        match pairs.next() {
            Some(second) => match second.as_rule() {
                Rule::term => Ok(Self {
                    has_leading_slash: true,
                    term: AstTerm::visit(second)?,
                }),
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }
}

impl From<MainTerm> for Vec<Term> {
    fn from(_main_term: MainTerm) -> Self {
        // main_term.terms
        todo!()
    }
}
