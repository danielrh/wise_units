use term::Term;

pub struct ComponentWithFactor {
    pub factor: u32,
    pub terms: Vec<Term>,
}

impl ComponentWithFactor {
    pub fn new() -> Self {
        ComponentWithFactor {
            factor: 1,
            terms: vec![],
        }
    }
}

impl Into<Vec<Term>> for ComponentWithFactor {
    fn into(mut self) -> Vec<Term> {
        let mut terms: Vec<Term> = Vec::with_capacity(self.terms.len() + 1);

        terms.append(&mut self.terms);

        if let Some(first_term) = terms.first_mut() {
            first_term.factor = self.factor;
        }

        terms
    }
}
