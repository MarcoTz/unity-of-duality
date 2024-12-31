pub type Covar = String;

pub enum Coterm {
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
}
