use term::Term;

pub struct Component {
    pub terms: Vec<Term>,
}

impl Component {
    pub fn new() -> Self {
        Component {
            terms: vec![],
        }
    }
}

impl Into<Vec<Term>> for Component {
    fn into(self) -> Vec<Term> {
        self.terms
    }
}
