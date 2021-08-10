use super::{Component, Error, Finishable, Term, Visit};
use crate::{invert::Invert, parser::terms::term_parser::Rule};
use log::trace;
use pest::iterators::Pair;

// pub(super) struct AstTerm {
//     pub(super) component: Component,
//     pub(super) terms: Vec<Term>,
// }
pub(super) enum AstTerm {
    TermDotComponent {
        term: Box<AstTerm>,
        component: Component,
    },
    TermSlashComponent {
        term: Box<AstTerm>,
        component: Component,
    },
    Component(Component),
}

impl AstTerm {
    pub(super) fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        trace!("AstTerm visit()");
        let mut pairs = pair.into_inner();

        let component = match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::component => Component::visit(first)?,
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        let op = match pairs.next() {
            Some(second) => match second.as_rule() {
                Rule::dot => SecondToken::Dot,
                Rule::slash => SecondToken::Slash,
                _ => unreachable!(),
            },
            None => return Ok(AstTerm::Component(component)),
        };

        match pairs.next() {
            Some(third) => match third.as_rule() {
                Rule::term => match op {
                    SecondToken::Dot => Ok(Self::TermDotComponent {
                        term: Box::new(AstTerm::visit(third)?),
                        component,
                    }),
                    SecondToken::Slash => Ok(Self::TermSlashComponent {
                        term: Box::new(AstTerm::visit(third)?),
                        component,
                    }),
                },
                _ => unreachable!(),
            },
            None => unreachable!(),
        }
    }
}

enum SecondToken {
    Dot,
    Slash,
}

// impl Finishable for AstTerm {
//     fn finish(self) -> Vec<Term> {
//         let mut component_terms = self.component.finish();
//         component_terms.extend(self.terms.into_iter());

//         component_terms
//     }
// }
