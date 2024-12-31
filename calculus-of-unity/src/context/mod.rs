pub mod context_judgement;
pub mod linear_context;

pub use context_judgement::ContextJudgement;
pub use linear_context::LinearContext;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Context {
    pub contexts: Vec<LinearContext>,
}

impl Context {
    pub fn combine(self, other: Context) -> Context {
        let mut contexts = self.contexts;
        contexts.extend(other.contexts);
        Context { contexts }
    }

    pub fn as_linear(self) -> Option<LinearContext> {
        if self.contexts.len() == 1 {
            self.contexts.first().cloned()
        } else {
            None
        }
    }
}
