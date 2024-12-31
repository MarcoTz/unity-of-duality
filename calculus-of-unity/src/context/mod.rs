pub mod context_judgement;
pub mod linear_context;

pub use context_judgement::ContextJudgement;
pub use linear_context::LinearContext;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Context {
    pub contexts: Vec<LinearContext>,
}

impl Context {
    pub fn as_linear(self) -> Option<LinearContext> {
        if self.contexts.len() == 1 {
            self.contexts.first().cloned()
        } else {
            None
        }
    }
}
