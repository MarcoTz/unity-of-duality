use std::fmt;

pub mod judgement;
pub mod linear_context;

pub use judgement::ContextJudgement;
pub use linear_context::LinearContext;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Context {
    pub contexts: Vec<LinearContext>,
}

impl Context {
    pub fn append(self, other: Context) -> Context {
        let mut new_contexts = self.contexts;
        new_contexts.extend(other.contexts);
        Context {
            contexts: new_contexts,
        }
    }

    pub fn as_linear(self) -> Option<LinearContext> {
        if self.contexts.len() == 1 {
            self.contexts.first().cloned()
        } else {
            None
        }
    }
}

impl fmt::Display for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(
            &self
                .contexts
                .iter()
                .map(|bnd| format!("{}", bnd))
                .collect::<Vec<String>>()
                .join(", "),
        )
    }
}
