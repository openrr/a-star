#[derive(Debug, Default, Clone, Copy)]
pub struct Node {
    pub heuristic: f64,
    pub real: f64,
}

impl Node {
    pub fn total_score(&self) -> f64 {
        self.heuristic + self.real
    }
}
