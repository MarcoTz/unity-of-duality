use super::{Context, LinearContext};
use crate::{
    types::{Type, TypeVar},
    Covar, Var,
};

#[derive(Clone, PartialEq, Eq)]
pub enum ContextJudgement {
    Value(Var, TypeVar),
    Continuation(Covar, Type),
    Covalue(Var, TypeVar),
    Expression(Covar, Type),
}

impl ContextJudgement {
    pub fn as_val(self) -> Option<(Var, TypeVar)> {
        if let ContextJudgement::Value(var, tyvar) = self {
            Some((var, tyvar))
        } else {
            None
        }
    }

    pub fn as_coval(self) -> Option<(Var, TypeVar)> {
        if let ContextJudgement::Covalue(covar, tyvar) = self {
            Some((covar, tyvar))
        } else {
            None
        }
    }

    pub fn as_cont(self) -> Option<(Covar, Type)> {
        if let ContextJudgement::Continuation(covar, ty) = self {
            Some((covar, ty))
        } else {
            None
        }
    }
}

impl From<ContextJudgement> for LinearContext {
    fn from(judg: ContextJudgement) -> LinearContext {
        LinearContext {
            judgements: vec![judg],
        }
    }
}

impl From<ContextJudgement> for Context {
    fn from(judg: ContextJudgement) -> Context {
        Context {
            contexts: vec![judg.into()],
        }
    }
}
