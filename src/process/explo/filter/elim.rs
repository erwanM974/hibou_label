use std::fmt;

pub enum ExplorationFilterEliminationKind {
    MaxLoopInstanciation,
    MaxProcessDepth,
    MaxNodeNumber
}

impl fmt::Display for ExplorationFilterEliminationKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExplorationFilterEliminationKind::MaxLoopInstanciation => {
                write!(f,"MaxLoop")
            },
            ExplorationFilterEliminationKind::MaxProcessDepth => {
                write!(f,"MaxDepth")
            },
            ExplorationFilterEliminationKind::MaxNodeNumber => {
                write!(f,"MaxNum")
            }
        }
    }
}