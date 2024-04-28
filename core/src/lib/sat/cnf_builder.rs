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

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Variable {
    id: i32,
    name: String,
}

#[derive(Clone)]
pub struct CnfBuilder {
    pub variables: Vec<Variable>,
    pub clauses: Vec<Clause>,
}

impl Default for CnfBuilder {
    fn default() -> Self {
        Self { variables: vec![], clauses: Vec::new() }
    }
}

impl CnfBuilder {
    pub fn add_variable(&mut self, name: String) -> i32 {
        let id = (self.variables.len() as i32) + 1;
        self.variables.push(Variable { id, name });

        id
    }

    pub fn dump(&self) -> Vec<Vec<i32>> {
        Vec::from_iter(
            self.clauses
                .as_slice()
                .iter()
                .map(|c| c.dump())
        )
    }

    pub fn get_name(&self, cnf_id: i32) -> String {
        // DIMACS 1-indexes the variable ids
        let idx = (cnf_id.abs() - 1) as usize;
        let name = self.variables.get(idx).unwrap().name.clone();

        return if cnf_id > 0 { name } else { format!("~{}", name) };
    }
}
