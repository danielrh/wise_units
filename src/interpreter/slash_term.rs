use term::Term;

pub struct SlashTerm {
    pub terms: Vec<Term>,
}

impl SlashTerm {
    pub fn new() -> Self {
        SlashTerm {
            terms: vec![],
        }
    }
}

impl Into<Vec<Term>> for SlashTerm {
    fn into(self) -> Vec<Term> {
        self.terms
    }
}
