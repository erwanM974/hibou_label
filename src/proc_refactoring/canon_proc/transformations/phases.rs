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


use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::general_context::GeneralContext;

use crate::rendering::textual::monochrome::position::position_to_text;

use crate::proc_refactoring::canon_proc::transformations::transfokind::*;
use crate::proc_refactoring::canon_proc::transformations::transfofunc::*;






pub fn transfos_phase1<'lifetime>() -> Vec<(CanonizationTransformationKind, &'lifetime dyn Fn(&Interaction) -> Option<Interaction>)> {
    return vec![
        (CanonizationTransformationKind::Deduplicate,&deduplicate),
        (CanonizationTransformationKind::TriDeduplicateRF,&tri_deduplicate_right_flushed),
        (CanonizationTransformationKind::SimplLeft,&simpl_left),
        (CanonizationTransformationKind::SimplRight,&simpl_right),
        (CanonizationTransformationKind::FlushRight,&flush_right),
        (CanonizationTransformationKind::InvertPar,&invert_par_conditional),
        (CanonizationTransformationKind::TriInvertParRF,&tri_invert_par_conditional_right_flushed),
        (CanonizationTransformationKind::InvertAlt,&invert_alt_conditional),
        (CanonizationTransformationKind::TriInvertAltRF,&tri_invert_alt_conditional_right_flushed),
        (CanonizationTransformationKind::LoopSimpl,&loop_simpl),
        (CanonizationTransformationKind::LoopUnNest,&loop_unnest),
        (CanonizationTransformationKind::DeFactorizeLeft,&defactorize_left),
        (CanonizationTransformationKind::DeFactorizeRight,&defactorize_right),
        // ***
        (CanonizationTransformationKind::SortEmissionTargets,&sort_emission_targets)
    ];
}

pub fn transfos_phase2<'lifetime>() -> Vec<(CanonizationTransformationKind, &'lifetime dyn Fn(&Interaction) -> Option<Interaction>)> {
    return vec![
        (CanonizationTransformationKind::Deduplicate,&deduplicate),
        (CanonizationTransformationKind::TriDeduplicateRF,&tri_deduplicate_right_flushed),
        (CanonizationTransformationKind::SimplLeft,&simpl_left),
        (CanonizationTransformationKind::SimplRight,&simpl_right),
        (CanonizationTransformationKind::FlushRight,&flush_right),
        (CanonizationTransformationKind::InvertPar,&invert_par_conditional),
        (CanonizationTransformationKind::TriInvertParRF,&tri_invert_par_conditional_right_flushed),
        (CanonizationTransformationKind::InvertAlt,&invert_alt_conditional),
        (CanonizationTransformationKind::TriInvertAltRF,&tri_invert_alt_conditional_right_flushed),
        (CanonizationTransformationKind::LoopSimpl,&loop_simpl),
        (CanonizationTransformationKind::LoopUnNest,&loop_unnest),
        (CanonizationTransformationKind::FactorizePrefixStrict,&factorize_prefix_strict),
        (CanonizationTransformationKind::FactorizePrefixSeq,&factorize_prefix_seq),
        (CanonizationTransformationKind::FactorizePrefixPar,&factorize_prefix_par),
        (CanonizationTransformationKind::FactorizeSuffixStrict,&factorize_suffix_strict),
        (CanonizationTransformationKind::FactorizeSuffixSeq,&factorize_suffix_seq),
        (CanonizationTransformationKind::FactorizeSuffixPar,&factorize_suffix_par),
        // ***
        (CanonizationTransformationKind::SortEmissionTargets,&sort_emission_targets)
    ];
}

