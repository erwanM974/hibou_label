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



use crate::core::language::syntax::interaction::Interaction;
use crate::core::transformation::transfofunc::action::*;
use crate::core::transformation::transfofunc::dedupl::*;
use crate::core::transformation::transfofunc::defactorize::*;
use crate::core::transformation::transfofunc::factorize_par::factorize_par;
use crate::core::transformation::transfofunc::factorize_prefix::*;
use crate::core::transformation::transfofunc::factorize_suffix::*;
use crate::core::transformation::transfofunc::flush::*;
use crate::core::transformation::transfofunc::invert::*;
use crate::core::transformation::transfofunc::loop_alt_simpl::loop_alt_simpl;
use crate::core::transformation::transfofunc::loop_simpl::*;
use crate::core::transformation::transfofunc::simpl::*;
use crate::core::transformation::transfokind::InteractionTransformationKind;


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum CanonizationPhase {
    FirstDefactorize,
    SecondFactorize
}

impl CanonizationPhase {
    pub fn to_string(&self) -> String {
        match self {
            &CanonizationPhase::FirstDefactorize => {
                return "1:Defactorize".to_string();
            },
            &CanonizationPhase::SecondFactorize => {
                return "2:Factorize".to_string();
            }
        }
    }
}

pub fn transfos_phase1<'lifetime>() -> Vec<(InteractionTransformationKind, &'lifetime dyn Fn(&Interaction) -> Vec<Interaction>)> {
    return vec![
        (InteractionTransformationKind::Deduplicate,&deduplicate),
        (InteractionTransformationKind::Simpl,&simpl),
        (InteractionTransformationKind::FlushRight,&flush_right),
        (InteractionTransformationKind::InvertPar,&invert_par_sorted),
        (InteractionTransformationKind::InvertAlt,&invert_alt_sorted),
        (InteractionTransformationKind::LoopSimpl,&loop_empty_simpl),
        (InteractionTransformationKind::LoopUnNest,&loop_unnest),
        (InteractionTransformationKind::LoopAltSimpl,&loop_alt_simpl),
        // ***
        (InteractionTransformationKind::DeFactorizeLeft,&defactorize_left),
        (InteractionTransformationKind::DeFactorizeRight,&defactorize_right),
        // ***
        (InteractionTransformationKind::SortActionContent,&sort_action_content)
    ];
}

pub fn transfos_phase2<'lifetime>() -> Vec<(InteractionTransformationKind, &'lifetime dyn Fn(&Interaction) -> Vec<Interaction>)> {
    return vec![
        (InteractionTransformationKind::Deduplicate,&deduplicate),
        (InteractionTransformationKind::Simpl,&simpl),
        (InteractionTransformationKind::FlushRight,&flush_right),
        (InteractionTransformationKind::InvertPar,&invert_par_sorted),
        (InteractionTransformationKind::InvertAlt,&invert_alt_sorted),
        (InteractionTransformationKind::LoopSimpl,&loop_empty_simpl),
        (InteractionTransformationKind::LoopUnNest,&loop_unnest),
        (InteractionTransformationKind::LoopAltSimpl,&loop_alt_simpl),
        // ***
        (InteractionTransformationKind::FactorizePrefixStrict,&factorize_prefix_strict),
        (InteractionTransformationKind::FactorizePrefixSeq,&factorize_prefix_seq),
        (InteractionTransformationKind::FactorizeSuffixStrict,&factorize_suffix_strict),
        (InteractionTransformationKind::FactorizeSuffixSeq,&factorize_suffix_seq),
        (InteractionTransformationKind::FactorizeCommutativePar,&factorize_par),
        // ***
        (InteractionTransformationKind::SortActionContent,&sort_action_content)
    ];
}

