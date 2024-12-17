use std::fmt;

pub type Atom = String;

pub enum Formula {
    Atom(Atom),
    Falsum,
    And(Box<Formula>, Box<Formula>),
    Truth,
    Par(Box<Formula>, Box<Formula>),
    NegN(Box<Formula>),
}

impl Formula {
    pub fn and(l: Formula, r: Formula) -> Formula {
        Formula::And(Box::new(l), Box::new(r))
    }

    pub fn par(l: Formula, r: Formula) -> Formula {
        Formula::Par(Box::new(l), Box::new(r))
    }

    pub fn negn(f: Formula) -> Formula {
        Formula::NegN(Box::new(f))
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
            Formula::Falsum => f.write_str("⊥"),
            Formula::And(l, r) => write!(f, "({l}) & ({r})"),
            Formula::Truth => f.write_str("⊤"),
            Formula::Par(l, r) => write!(f, "({l}) ⅋ ({r})"),
            Formula::NegN(f1) => write!(f, "n¬{f1}"),
        }
    }
}
