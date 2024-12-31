pub mod context_judgement;
pub mod linear_context;

pub use context_judgement::ContextJudgement;
pub use linear_context::LinearContext;

pub struct Context {
    pub contexts: Vec<LinearContext>,
}
