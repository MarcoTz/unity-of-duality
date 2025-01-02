use super::{Covar, Var};

#[derive(Clone, PartialEq, Eq)]
pub enum Coterm {
    Var(Var),
    Covar(Covar),
    Counit,
    Fst(Box<Coterm>),
    Snd(Box<Coterm>),
    LPair(Box<Coterm>, Box<Coterm>),
}

impl Coterm {
    pub fn fst(ct: Coterm) -> Coterm {
        Coterm::Fst(Box::new(ct))
    }

    pub fn snd(ct: Coterm) -> Coterm {
        Coterm::Snd(Box::new(ct))
    }

    pub fn lpair(fst: Coterm, snd: Coterm) -> Coterm {
        Coterm::LPair(Box::new(fst), Box::new(snd))
    }

    pub fn as_covar(self) -> Option<Covar> {
        if let Coterm::Covar(cv) = self {
            Some(cv)
        } else {
            None
        }
    }

    pub fn as_var(self) -> Option<Var> {
        if let Coterm::Var(v) = self {
            Some(v)
        } else {
            None
        }
    }

    pub fn as_fst(self) -> Option<Coterm> {
        if let Coterm::Fst(t) = self {
            Some(*t)
        } else {
            None
        }
    }

    pub fn as_snd(self) -> Option<Coterm> {
        if let Coterm::Snd(t) = self {
            Some(*t)
        } else {
            None
        }
    }
}
