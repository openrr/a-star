#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum NodeStatus {
    Start,
    Goal,
    Open,
    Closed,
    Disable,
    None,
    Path,
}

impl Default for NodeStatus {
    fn default() -> Self {
        Self::None
    }
}

impl std::fmt::Display for NodeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeStatus::Start => write!(f, "S"),
            NodeStatus::Goal => write!(f, "G"),
            NodeStatus::Open => write!(f, "O"),
            NodeStatus::Closed => write!(f, "C"),
            NodeStatus::Disable => write!(f, "D"),
            NodeStatus::None => write!(f, "N"),
            NodeStatus::Path => write!(f, "P"),
        }
    }
}
