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



use std::collections::{HashMap, HashSet};
use crate::core::execution::semantics::frontier::FrontierElement;

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationStepKind {
    BeforeStart,
    AfterEnd
}



pub enum AnalysisStepKind {
    EliminateNoLongerObserved(HashSet<usize>), // all the ids of all the co-localizations to eliminate
    Execute(FrontierElement, // frontier element to execute
    HashSet<usize>, // co-localisations on which multi-trace action consumption must be done
            HashMap<usize,SimulationStepKind>) // co-localisations on which simulation must be done and which kind
}


