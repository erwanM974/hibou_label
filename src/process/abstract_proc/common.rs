/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/





pub enum FilterEliminationKind {
    MaxLoopInstanciation,
    MaxProcessDepth,
    MaxNodeNumber
}

impl std::string::ToString for FilterEliminationKind {
    fn to_string(&self) -> String {
        match self {
            FilterEliminationKind::MaxLoopInstanciation => {
                return "MaxLoop".to_string();
            },
            FilterEliminationKind::MaxProcessDepth => {
                return "MaxDepth".to_string();
            },
            FilterEliminationKind::MaxNodeNumber => {
                return "MaxNum".to_string();
            }
        }
    }
}


pub enum HibouSearchStrategy {
    BFS, // breadth first search
    DFS, // depth first search
    HCS  // high coverage search
}

impl std::string::ToString for HibouSearchStrategy {
    fn to_string(&self) -> String {
        match self {
            HibouSearchStrategy::BFS => {
                return "BreadthFirstSearch".to_string();
            },
            HibouSearchStrategy::DFS => {
                return "DepthFirstSearch".to_string();
            },
            HibouSearchStrategy::HCS => {
                return "HighCoverageSearch".to_string();
            }
        }
    }
}
