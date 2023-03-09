use crate::node_status::NodeStatus;
use crate::score::Score;

#[derive(Clone, Debug, Default)]
pub(crate) struct Node {
    pub status: NodeStatus,
    pub score: Option<Score>,
}

impl std::cmp::PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        if let Some(score1) = &self.score {
            if let Some(score2) = &other.score {
                score1.get_score() == score2.get_score()
            } else {
                false
            }
        } else {
            false
        }
    }
}

impl std::cmp::Eq for Node {}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if let Some(score1) = &self.score {
            other
                .score
                .as_ref()
                .map(|score2| score1.get_score().cmp(&score2.get_score()))
        } else {
            None
        }
    }
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if let Some(score1) = &self.score {
            if let Some(score2) = &other.score {
                score1.get_score().cmp(&score2.get_score())
            } else {
                std::cmp::Ordering::Greater
            }
        } else if other.score.is_some() {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        }
    }
}
