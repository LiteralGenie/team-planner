#[derive(Clone, Copy, Debug)]
pub struct Literal {
    pub variable: i32,
    pub negate: bool,
}

impl Literal {
    pub fn dump(&self) -> i32 {
        match self.negate {
            true => -self.variable,
            false => self.variable,
        }
    }
}

pub struct Clause {
    pub literals: Vec<Literal>,
}

impl Clause {
    pub fn dump(&self) -> Vec<i32> {
        let lits = self.literals.iter().map(|l| l.dump());
        Vec::from_iter(lits)
    }

    pub fn add(&self, other: &Self) -> Clause {
        let mut literals = self.literals.clone();
        literals.extend(other.literals.iter());

        Clause {
            literals,
        }
    }
}

pub struct CnfBuilder {
    pub num_variables: i32,
    pub clauses: Vec<Clause>,
}

impl Default for CnfBuilder {
    fn default() -> Self {
        Self { num_variables: 0, clauses: Vec::new() }
    }
}

impl CnfBuilder {
    pub fn add_variable(&mut self) -> i32 {
        self.num_variables += 1;
        self.num_variables
    }

    pub fn dump(&self) -> Vec<Vec<i32>> {
        Vec::from_iter(
            self.clauses
                .as_slice()
                .iter()
                .map(|c| c.dump())
        )
    }
}
