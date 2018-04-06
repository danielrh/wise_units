use term::Term;

pub struct DotTerm {
    pub terms: Vec<Term>,
}

impl DotTerm {
    pub fn new() -> Self {
        DotTerm {
            terms: vec![],
        }
    }
}

impl Into<Vec<Term>> for DotTerm {
    fn into(self) -> Vec<Term> {
        self.terms
    }
}
