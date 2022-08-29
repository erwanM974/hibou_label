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

use crate::proc_refactoring::abstract_proc::AbstractStepKind;
use crate::proc_refactoring::canon_proc::conf::CanonizationConfig;
use crate::proc_refactoring::canon_proc::priorities::CanonizationPriorities;
use crate::proc_refactoring::canon_proc::transformations::phases::CanonizationTransformation;
use crate::proc_refactoring::canon_proc::transformations::transfokind::CanonizationTransformationKind;

pub enum CanonizationStepKind {
    Transform(CanonizationTransformation)
}



impl AbstractStepKind<CanonizationConfig> for CanonizationStepKind {

    fn get_priority(&self, process_priorities: &CanonizationPriorities) -> i32 {
        match self {
            CanonizationStepKind::Transform( transfo ) => {
                let mut priority : i32 = 0;
                match transfo.kind {
                    CanonizationTransformationKind::SimplLeft => {
                        priority += process_priorities.simpl;
                    },
                    CanonizationTransformationKind::SimplRight => {
                        priority += process_priorities.simpl;
                    },
                    CanonizationTransformationKind::FlushLeft => {
                        priority += process_priorities.flush;
                    },
                    CanonizationTransformationKind::FlushRight => {
                        priority += process_priorities.flush;
                    },
                    CanonizationTransformationKind::InvertAlt => {
                        priority += process_priorities.invert;
                    },
                    CanonizationTransformationKind::InvertPar => {
                        priority += process_priorities.invert;
                    },
                    CanonizationTransformationKind::TriInvertAltRF => {
                        priority += process_priorities.invert;
                    },
                    CanonizationTransformationKind::TriInvertParRF => {
                        priority += process_priorities.invert;
                    },
                    CanonizationTransformationKind::Deduplicate => {
                        priority += process_priorities.deduplicate;
                    },
                    CanonizationTransformationKind::TriDeduplicateRF => {
                        priority += process_priorities.deduplicate;
                    },
                    CanonizationTransformationKind::FactorizePrefixStrict => {
                        priority += process_priorities.factorize;
                    },
                    CanonizationTransformationKind::FactorizePrefixSeq => {
                        priority += process_priorities.factorize;
                    },
                    CanonizationTransformationKind::FactorizePrefixPar => {
                        priority += process_priorities.factorize;
                    },
                    CanonizationTransformationKind::FactorizeSuffixStrict => {
                        priority += process_priorities.factorize;
                    },
                    CanonizationTransformationKind::FactorizeSuffixSeq => {
                        priority += process_priorities.factorize;
                    },
                    CanonizationTransformationKind::FactorizeSuffixPar => {
                        priority += process_priorities.factorize;
                    },
                    CanonizationTransformationKind::DeFactorizeLeft => {
                        priority += process_priorities.defactorize;
                    },
                    CanonizationTransformationKind::DeFactorizeRight => {
                        priority += process_priorities.defactorize;
                    },
                    CanonizationTransformationKind::LoopSimpl => {
                        priority += process_priorities.simpl;
                    },
                    CanonizationTransformationKind::LoopUnNest => {
                        priority += process_priorities.simpl;
                    },
                    CanonizationTransformationKind::SortEmissionTargets => {
                        priority += process_priorities.simpl;
                    }
                }
                // ***
                return priority;
            }
        }
    }

}

