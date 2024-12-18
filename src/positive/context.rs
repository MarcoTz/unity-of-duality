use super::linear_context::{ContextJudgement, LinearContext};
use std::fmt;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Context {
    pub bindings: Vec<LinearContext>,
}

impl Context {
    pub fn append(self, other: Context) -> Context {
        let mut new_bindings = self.bindings;
        new_bindings.extend(other.bindings);
        Context {
            bindings: new_bindings,
        }
    }

    pub fn as_linear(self) -> Option<LinearContext> {
        if self.bindings.len() == 1 {
            self.bindings.first().cloned()
        } else {
            None
        }
    }
}

impl From<LinearContext> for Context {
    fn from(ctx: LinearContext) -> Context {
        Context {
            bindings: vec![ctx],
        }
    }
}

impl From<Vec<LinearContext>> for Context {
    fn from(bindings: Vec<LinearContext>) -> Context {
        Context { bindings }
    }
}

impl From<ContextJudgement> for Context {
    fn from(judgement: ContextJudgement) -> Context {
        Context {
            bindings: vec![judgement.into()],
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .bindings
                .iter()
                .map(|bnd| format!("{}", bnd))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
