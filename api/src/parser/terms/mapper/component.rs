use super::{Annotatable, Annotation, AstTerm, Error, Factor, Finishable, Term, Visit};
use crate::parser::terms::term_parser::Rule;
use pest::iterators::Pair;

pub(super) enum Component {
    AA {
        annotatable: Annotatable,
        annotation: String,
    },
    Annotatable(Annotatable),
    Annotation(String),
    Factor(u32),
    Term(Box<AstTerm>),
}

impl Visit<Rule> for Component {
    fn visit(pair: Pair<'_, Rule>) -> Result<Self, Error> {
        let mut pairs = pair.into_inner();

        let annotatable = match pairs.next() {
            Some(first) => match first.as_rule() {
                Rule::annotatable => Annotatable::visit(first)?,
                Rule::annotation => return Ok(Self::Annotation(Annotation::visit(first)?)),
                Rule::factor => return Ok(Self::Factor(Factor::visit(first)?)),
                Rule::term => return Ok(Self::Term(Box::new(AstTerm::visit(first)?))),
                _ => unreachable!(),
            },
            None => unreachable!(),
        };

        match pairs.next() {
            Some(second) => match second.as_rule() {
                Rule::annotation => Ok(Self::AA {
                    annotatable,
                    annotation: Annotation::visit(second)?,
                }),
                _ => unreachable!(),
            },
            None => Ok(Self::Annotatable(annotatable)),
        }
    }
}

// impl Finishable for Component {
//     fn finish(self) -> Vec<Term> {
//         let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

//         let self_term = if let Some(annotatable) = self.annotatable {
//             match annotatable {
//                 Annotatable::PrefixedWithExponent {
//                     prefix,
//                     atom,
//                     exponent,
//                 } => Term::FPAEA {
//                     factor: self.factor,
//                     prefix,
//                     atom,
//                     exponent,
//                     annotation: self.annotation,
//                 },
//                 Annotatable::Prefixed { prefix, atom } => Term {
//                     factor: self.factor,
//                     prefix: Some(prefix),
//                     atom: Some(atom),
//                     exponent: None,
//                     annotation: self.annotation,
//                 },
//                 Annotatable::BasicWithExponent { atom, exponent } => Term {
//                     factor: self.factor,
//                     prefix: None,
//                     atom: Some(atom),
//                     exponent: Some(exponent),
//                     annotation: self.annotation,
//                 },
//                 Annotatable::Basic { atom } => Term {
//                     factor: self.factor,
//                     prefix: None,
//                     atom: Some(atom),
//                     exponent: None,
//                     annotation: self.annotation,
//                 },
//                 Annotatable::Unity => Term {
//                     factor: self.factor,
//                     prefix: None,
//                     atom: None,
//                     exponent: None,
//                     annotation: self.annotation,
//                 },
//             }
//         } else {
//             Term {
//                 factor: self.factor,
//                 prefix: None,
//                 atom: None,
//                 exponent: None,
//                 annotation: self.annotation,
//             }
//         };

//         terms.push(self_term);
//         terms.extend_from_slice(&self.terms);

//         terms
//     }
// }
