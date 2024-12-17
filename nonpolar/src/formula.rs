use std::fmt;

pub type Atom = String;

pub enum Formula {
    Atom(Atom),
    Neg(Box<Formula>),
    True,
    And(Box<Formula>, Box<Formula>),
    False,
    Or(Box<Formula>, Box<Formula>),
}

impl Formula {
    pub fn neg(f: Formula) -> Formula {
        Formula::Neg(Box::new(f))
    }

    pub fn and(l: Formula, r: Formula) -> Formula {
        Formula::And(Box::new(l), Box::new(r))
    }

    pub fn or(l: Formula, r: Formula) -> Formula {
        Formula::Or(Box::new(l), Box::new(r))
    }
}

impl From<Atom> for Formula {
    fn from(at: Atom) -> Formula {
        Formula::Atom(at)
    }
}

impl fmt::Display for Formula {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Formula::Atom(at) => f.write_str(at),
            Formula::Neg(f1) => write!(f, "¬({f1})"),
            Formula::True => f.write_str("T"),
            Formula::And(l, r) => write!(f, "({l}) ∧ ({r})"),
            Formula::False => f.write_str("F"),
            Formula::Or(l, r) => write!(f, "({l}) ∨ ({r})"),
        }
    }
}
