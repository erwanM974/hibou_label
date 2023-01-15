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


use crate::core::transformation::transfores::InteractionTransformationResult;
use crate::core::transformation::transfokind::InteractionTransformationKind;
use crate::process::abstract_proc::generic::AbstractStepKind;
use crate::process::canon_proc::interface::conf::CanonizationConfig;
use crate::process::canon_proc::interface::priorities::CanonizationPriorities;




pub enum CanonizationStepKind {
    Transform(InteractionTransformationResult),
    ChangePhase
}


impl AbstractStepKind<CanonizationConfig> for CanonizationStepKind {

    fn get_priority(&self, process_priorities: &CanonizationPriorities) -> i32 {
        match self {
            CanonizationStepKind::ChangePhase => {
                return 0;
            },
            CanonizationStepKind::Transform( transfo ) => {
                let mut priority : i32 = 0;
                match transfo.kind {
                    InteractionTransformationKind::Simpl => {
                        priority += process_priorities.simpl;
                    },
                    /*InteractionTransformationKind::FlushLeft => {
                        priority += process_priorities.flush;
                    },*/
                    InteractionTransformationKind::FlushRight => {
                        priority += process_priorities.flush;
                    },
                    InteractionTransformationKind::InvertAlt => {
                        priority += process_priorities.invert;
                    },
                    InteractionTransformationKind::InvertPar => {
                        priority += process_priorities.invert;
                    },
                    InteractionTransformationKind::Deduplicate => {
                        priority += process_priorities.deduplicate;
                    },
                    InteractionTransformationKind::FactorizePrefixStrict => {
                        priority += process_priorities.factorize;
                    },
                    InteractionTransformationKind::FactorizePrefixSeq => {
                        priority += process_priorities.factorize;
                    },
                    InteractionTransformationKind::FactorizeSuffixStrict => {
                        priority += process_priorities.factorize;
                    },
                    InteractionTransformationKind::FactorizeSuffixSeq => {
                        priority += process_priorities.factorize;
                    },
                    InteractionTransformationKind::FactorizeCommutativePar => {
                        priority += process_priorities.factorize;
                    },
                    InteractionTransformationKind::DeFactorizeLeft => {
                        priority += process_priorities.defactorize;
                    },
                    InteractionTransformationKind::DeFactorizeRight => {
                        priority += process_priorities.defactorize;
                    },
                    InteractionTransformationKind::LoopSimpl => {
                        priority += process_priorities.simpl;
                    },
                    /*InteractionTransformationKind::LoopAltSimpl => {
                        priority += process_priorities.simpl;
                    },*/
                    InteractionTransformationKind::LoopUnNest => {
                        priority += process_priorities.simpl;
                    },
                    InteractionTransformationKind::SortActionContent => {
                        priority += process_priorities.simpl;
                    },
                    /*
                    InteractionTransformationKind::MergeShiftLeft1 => {
                        // nothing
                    },
                    InteractionTransformationKind::MergeShiftLeft2 => {
                        // nothing
                    },
                    InteractionTransformationKind::MergeShiftRight1 => {
                        // nothing
                    },
                    InteractionTransformationKind::MergeShiftRight2 => {
                        // nothing
                    },
                    InteractionTransformationKind::MergeSkip => {
                        // nothing
                    },
                    InteractionTransformationKind::MergeSkipInvert => {
                        // nothing
                    },
                    InteractionTransformationKind::MergeAction => {
                        // nothing
                    }*/
                }
                // ***
                return priority;
            }
        }
    }

}

