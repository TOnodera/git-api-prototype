pub mod errors;
pub mod types;

pub struct Env {
    pub dir: Option<String>,
}

impl Env {
    pub fn new(dir: Option<String>) -> Self {
        Self { dir }
    }
}
