use super::{Covar, Var};

#[derive(Clone)]
pub enum Coterm {
    Var(Var),
    Covar(Covar),
}

impl Coterm {
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
}
