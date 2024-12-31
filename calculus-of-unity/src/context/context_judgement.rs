use crate::{
    types::{Type, TypeVar},
    Covar, Var,
};

pub enum ContextJudgement {
    Value(Var, TypeVar),
    Continuation(Covar, Type),
}
