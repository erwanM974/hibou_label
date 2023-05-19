



pub enum ExplorationFilterEliminationKind {
    MaxLoopInstanciation,
    MaxProcessDepth,
    MaxNodeNumber
}

impl std::string::ToString for ExplorationFilterEliminationKind {
    fn to_string(&self) -> String {
        match self {
            ExplorationFilterEliminationKind::MaxLoopInstanciation => {
                return "MaxLoop".to_string();
            },
            ExplorationFilterEliminationKind::MaxProcessDepth => {
                return "MaxDepth".to_string();
            },
            ExplorationFilterEliminationKind::MaxNodeNumber => {
                return "MaxNum".to_string();
            }
        }
    }
}