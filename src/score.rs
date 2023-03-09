#[derive(Clone, Debug, Default)]
pub(crate) struct Score {
    pub(crate) real: u64,
    pub(crate) estimate: u64,
}

impl Score {
    pub fn get_score(&self) -> u64 {
        self.real + self.estimate
    }
}
