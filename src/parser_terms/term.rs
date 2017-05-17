use parser_terms::Component;
use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
use std::fmt;
use unit::Dimension;

#[derive(Debug, PartialEq)]
pub enum Term<'a> {
    DotCombined(Component<'a>, Box<Term<'a>>),
    SlashCombined(Component<'a>, Box<Term<'a>>),
    Basic(Component<'a>),
}

impl<'a> Term<'a> {
    pub fn composition(&self) -> BTreeMap<Dimension, i32> {
        match *self {
            Term::Basic(ref component) => component.composition(),
            Term::SlashCombined(ref component, ref box_term) => {
                let mut component_composition = component.composition();
                let ref term = *box_term;
                let term_composition = term.composition();

                for (term_dim_name, term_value) in term_composition {
                    match component_composition.entry(term_dim_name) {
                        Entry::Vacant(e) => { e.insert(-term_value); },
                        Entry::Occupied(mut e) => { *e.get_mut() -= term_value; }
                    }
                }

                component_composition
            },
            Term::DotCombined(ref component, ref box_term) => {
                let mut component_composition = component.composition();
                let ref term = *box_term;
                let term_composition = term.composition();

                for (term_dim_name, term_value) in term_composition {
                    match component_composition.entry(term_dim_name) {
                        Entry::Vacant(e) => { e.insert(term_value); },
                        Entry::Occupied(mut e) => { *e.get_mut() += term_value; }
                    }
                }

                component_composition
            }
        }
    }

    fn composition_string(&self) -> String {
        let composition = self.composition();

        composition.into_iter()
            .map(|(k, v)| match v {
                1 => k.to_string(),
                _ => format!("{}{}", k, v),
            })
            .collect::<Vec<String>>()
            .as_slice()
            .join(".")
    }

    pub fn is_compatible_with<'b>(&self, other: &Term<'b>) -> bool {
        let me = self.composition();
        let other = other.composition();

        me == other
    }

    pub fn is_special(&self) -> bool {
        match *self {
            Term::Basic(ref component) => component.is_special(),
            _ => false
        }
    }

    pub fn scalar(&self) -> f64 {
        match *self {
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.scalar() * term.scalar()
            },
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                component.scalar() / term.scalar()
            },
            Term::Basic(ref component) => component.scalar()
        }
    }
}

impl<'a> fmt::Display for Term<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Term::DotCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                write!(f, "{}.{}", &component, &term)
            },
            Term::SlashCombined(ref component, ref box_term) => {
                let ref term = *box_term;
                write!(f, "{}/{}", component, term)
            },
            Term::Basic(ref component) => { write!(f, "{}", component) },
        }
    }
}

#[cfg(test)]
mod tests {
    use parser::*;
    use parser_terms::*;
    use parser_terms::UnitSign::*;
    use std::collections::BTreeMap;
    use unit::Dimension;
    use unit::base::{Gram, Meter, Second};
    use unit::prefix::Kilo;

    #[test]
    fn validate_parsing_term_with_dot() {
        assert_eq!(
            parse_Term("g.m").unwrap(),
            Term::DotCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(Box::new(Gram))
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(Box::new(Meter))
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_parsing_term_with_slash() {
        assert_eq!(
            parse_Term("kg/s").unwrap(),
            Term::SlashCombined(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::PrefixedAtom(Box::new(Kilo), Box::new(Gram))
                    )
                ),
                Box::new(
                    Term::Basic(
                        Component::Annotatable(
                            Annotatable::Unit(
                                SimpleUnit::Atom(Box::new(Second))
                            )
                        )
                    )
                )
            )
        );
    }

    #[test]
    fn validate_parsing_term_basic_with_prefix_and_exponent() {
        assert_eq!(
            parse_Term("kg2").unwrap(),
            Term::Basic(
                Component::Annotatable(
                    Annotatable::UnitWithPower(
                        SimpleUnit::PrefixedAtom(
                            Box::new(Kilo),
                            Box::new(Gram)
                        ),
                        Exponent(Positive, 2)
                    )
                )
            )
        );
    }

    #[test]
    fn validate_parsing_term_basic_with_exponent() {
        assert_eq!(
            parse_Term("g2").unwrap(),
            Term::Basic(
                Component::Annotatable(
                    Annotatable::UnitWithPower(
                        SimpleUnit::Atom(Box::new(Gram)),
                        Exponent(Positive, 2)
                    )
                )
            )
        );
    }

    #[test]
    fn validate_parsing_term_basic() {
        assert_eq!(
            parse_Term("g").unwrap(),
            Term::Basic(
                Component::Annotatable(
                    Annotatable::Unit(
                        SimpleUnit::Atom(Box::new(Gram))
                    )
                )
            )
        );
    }

    #[test]
    fn validate_composition() {
        let term = parse_Term("m").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(term.composition(), map);

        let term = parse_Term("m2/s").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        map.insert(Dimension::Time, -1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("s/m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, -2);
        map.insert(Dimension::Time, 1);
        assert_eq!(term.composition(), map);
    }

    #[test]
    fn validate_composition_with_prefix() {
        let term = parse_Term("km").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("km2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(term.composition(), map);

        let term = parse_Term("km2/s").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        map.insert(Dimension::Time, -1);
        assert_eq!(term.composition(), map);

        let term = parse_Term("s/km2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, -2);
        map.insert(Dimension::Time, 1);
        assert_eq!(term.composition(), map);
    }

    #[test]
    fn validate_composition_with_dimless() {
        let term = parse_Term("[pi].m2").unwrap();
        let mut map: BTreeMap<Dimension, i32> = BTreeMap::new();
        map.insert(Dimension::Length, 2);
        assert_eq!(term.composition(), map);
    }

    #[test]
    fn validate_composition_string() {
        let term = parse_Term("m").unwrap();
        assert_eq!(term.composition_string(), "L".to_string());

        let term = parse_Term("m2").unwrap();
        assert_eq!(term.composition_string(), "L2".to_string());

        let term = parse_Term("m2/s").unwrap();
        assert_eq!(term.composition_string(), "L2.T-1".to_string());

        let term = parse_Term("s/m2").unwrap();
        assert_eq!(term.composition_string(), "L-2.T".to_string());
    }

    #[test]
    fn validate_composition_string_with_prefix() {
        let term = parse_Term("km").unwrap();
        assert_eq!(term.composition_string(), "L".to_string());

        let term = parse_Term("km2").unwrap();
        assert_eq!(term.composition_string(), "L2".to_string());

        let term = parse_Term("km2/s").unwrap();
        assert_eq!(term.composition_string(), "L2.T-1".to_string());

        let term = parse_Term("s/km2").unwrap();
        assert_eq!(term.composition_string(), "L-2.T".to_string());
    }

    #[test]
    fn validate_is_compatible_with() {
        let me = parse_Term("m2").unwrap();
        let other = parse_Term("m3/m").unwrap();
        assert!(me.is_compatible_with(&other))
    }

    #[test]
    fn validate_is_compatible_with_with_prefix() {
        let me = parse_Term("m").unwrap();
        let other = parse_Term("km").unwrap();
        assert!(me.is_compatible_with(&other))
    }
}
