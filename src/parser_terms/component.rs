use parser_terms::{Annotatable, Annotation, Factor, Term};
use std::collections::BTreeMap;
use std::fmt;
use unit::Dimension;

#[derive(Debug, PartialEq)]
pub enum Component<'a> {
    AnnotatedAnnotatable(Annotatable<'a>, Annotation<'a>),
    Annotatable(Annotatable<'a>),
    Annotation(Annotation<'a>),
    Factor(Factor),
    Term(Box<Term<'a>>),
}

impl<'a> Component<'a> {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        match *self {
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => annotatable.composition(),
            Component::Annotatable(ref annotatable) => annotatable.composition(),
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.composition()
            }
            _ => BTreeMap::new(),
        }
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => {
                annotatable.is_special()
            }
            Component::Annotatable(ref annotatable) => annotatable.is_special(),
            _ => false,
        }
    }

    pub fn scalar(&self) -> f64 {
        match *self {
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.scalar()
            }
            Component::Factor(ref factor) => factor.0 as f64,
            Component::Annotation(_) => 1.0,
            Component::Annotatable(ref annotatable) => annotatable.scalar(),
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => {
                annotatable.scalar()
            }
        }
    }

    pub fn magnitude(&self) -> f64 {
        match *self {
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.magnitude()
            }
            Component::Factor(ref factor) => factor.0 as f64,
            Component::Annotation(_) => 1.0,
            Component::Annotatable(ref annotatable) => annotatable.magnitude(),
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => {
                annotatable.magnitude()
            }
        }
    }

    pub fn calculate_scalar(&self, magnitude: f64) -> f64 {
        match *self {
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.calculate_scalar(magnitude)
            }
            Component::Factor(ref factor) => factor.0 as f64 * magnitude,
            Component::Annotation(_) => magnitude,
            Component::Annotatable(ref annotatable) => annotatable.calculate_scalar(magnitude),
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => {
                annotatable.calculate_scalar(magnitude)
            }
        }
    }

    pub fn calculate_magnitude(&self, scalar: f64) -> f64 {
        match *self {
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                term.calculate_magnitude(scalar)
            }
            Component::Factor(ref factor) => factor.0 as f64 * scalar,
            Component::Annotation(_) => scalar,
            Component::Annotatable(ref annotatable) => annotatable.calculate_magnitude(scalar),
            Component::AnnotatedAnnotatable(ref annotatable, ref _annotation) => {
                annotatable.calculate_magnitude(scalar)
            }
        }
    }
}

impl<'a> fmt::Display for Component<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Component::AnnotatedAnnotatable(ref annotatable, ref annotation) => {
                write!(f, "{}{{{}}}", annotatable, annotation)
            }
            Component::Annotatable(ref annotatable) => write!(f, "{}", annotatable),
            Component::Annotation(ref annotation) => write!(f, "{}", annotation),
            Component::Factor(ref factor) => write!(f, "{}", factor),
            Component::Term(ref box_term) => {
                let ref term = *box_term;
                write!(f, "{}", term)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Component;
    use unit::base::Meter;
    use unit::prefix::Kilo;
    use parser::parse_Component;
    use parser_terms::{Annotatable, Annotation, Exponent, Factor, SimpleUnit, Term, UnitSign};

    #[test]
    fn validate_parsing_component_with_annotations() {
        let annotation = Annotation("%vol");

        assert_eq!(parse_Component("km-10{%vol}").unwrap(),
                   Component::AnnotatedAnnotatable(make_annotatable(), annotation));

        assert_eq!(parse_Component("km-10").unwrap(),
                   Component::Annotatable(make_annotatable()));

        let annotation = Annotation("wet'tis.");

        assert_eq!(parse_Component("{wet'tis.}").unwrap(),
                   Component::Annotation(annotation));
    }

    #[test]
    fn validate_parsing_component_with_factor() {
        assert_eq!(parse_Component("123").unwrap(),
                   Component::Factor(Factor(123)));
    }

    #[test]
    fn validate_parsing_component_with_term() {
        assert_eq!(parse_Component("(123)").unwrap(),
                   Component::Term(Box::new(Term::Basic(Component::Factor(Factor(123))))));
    }

    fn make_annotatable<'a>() -> Annotatable<'a> {
        let simple_unit = SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Meter));
        let negative_exp = Exponent(UnitSign::Negative, 10);

        Annotatable::UnitWithPower(simple_unit, negative_exp)
    }
}
