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