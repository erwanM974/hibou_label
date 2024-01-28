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

use std::fmt::Formatter;
use strum_macros::IntoStaticStr;

use crate::core::language::syntax::interaction::Interaction;
use crate::core::transformation::transfofunc::action::{transfo_sort_action_content, transfo_unfold_action};
use crate::core::transformation::transfofunc::alt_dedup::alt_dedup_equal::transfo_deduplicate;
use crate::core::transformation::transfofunc::defactorize::{transfo_defactorize_left, transfo_defactorize_right};
use crate::core::transformation::transfofunc::factorize::factorize_par::transfo_factorize_par;
use crate::core::transformation::transfofunc::factorize::factorize_prefix::{transfo_factorize_prefix_seq, transfo_factorize_prefix_strict};
use crate::core::transformation::transfofunc::factorize::factorize_suffix::{transfo_factorize_suffix_seq, transfo_factorize_suffix_strict};
use crate::core::transformation::transfofunc::flush::transfo_flush_right;
use crate::core::transformation::transfofunc::invert::{transfo_invert_alt_sorted, transfo_invert_par_sorted};
use crate::core::transformation::transfofunc::loop_simpl::{transfo_loop_empty_simpl, transfo_loop_unnest};
use crate::core::transformation::transfofunc::par_to_seq::transfo_par_to_seq;
use crate::core::transformation::transfofunc::simpl::transfo_simpl;
use crate::core::transformation::transfofunc::strict_to_seq::transfo_strict_to_seq;

#[derive(IntoStaticStr,Clone, PartialEq, Debug, Eq, Hash)]
pub enum InteractionTransformationKind {
    Simpl,
    FlushRight,
    InvertAlt,
    InvertPar,
    Deduplicate,
    FactorizePrefixStrict,
    FactorizePrefixSeq,
    FactorizeCommutativePar,
    FactorizeSuffixStrict,
    FactorizeSuffixSeq,
    DeFactorizeLeft,
    DeFactorizeRight,
    LoopSimpl,
    LoopUnNest,
    SortActionContent, // sort emission targets OR reception recipients
    StrictToSeq,
    ParToSeq,
    UnfoldActions,
    // ***
    /*MergeShiftLeft1,
    MergeShiftLeft2,
    MergeShiftRight1,
    MergeShiftRight2,
    MergeAction,
    MergeSkip,
    MergeSkipInvert,*/
}

impl std::fmt::Display for InteractionTransformationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let as_static_str : &'static str = self.into();
        write!(f,"{}", as_static_str)
    }
}


impl InteractionTransformationKind {
    pub fn to_string(&self) -> String {
        let as_static_str : &'static str = self.into();
        return as_static_str.to_string();
    }
    pub fn get_transformation(&self) -> fn(&Interaction) -> Vec<Interaction> {
        match self {
            InteractionTransformationKind::Simpl => {
                return transfo_simpl;
            }
            InteractionTransformationKind::FlushRight => {
                return transfo_flush_right;
            },
            InteractionTransformationKind::InvertAlt => {
                return transfo_invert_alt_sorted;
            },
            InteractionTransformationKind::InvertPar => {
                return transfo_invert_par_sorted;
            },
            InteractionTransformationKind::Deduplicate => {
                return transfo_deduplicate;
            },
            InteractionTransformationKind::FactorizePrefixStrict => {
                return transfo_factorize_prefix_strict;
            },
            InteractionTransformationKind::FactorizePrefixSeq => {
                return transfo_factorize_prefix_seq;
            },
            InteractionTransformationKind::FactorizeCommutativePar => {
                return transfo_factorize_par;
            },
            InteractionTransformationKind::FactorizeSuffixSeq => {
                return transfo_factorize_suffix_strict;
            },
            InteractionTransformationKind::FactorizeSuffixStrict => {
                return transfo_factorize_suffix_seq;
            },
            InteractionTransformationKind::DeFactorizeLeft => {
                return transfo_defactorize_left;
            },
            InteractionTransformationKind::DeFactorizeRight => {
                return transfo_defactorize_right;
            },
            InteractionTransformationKind::LoopSimpl => {
                return transfo_loop_empty_simpl;
            },
            InteractionTransformationKind::LoopUnNest => {
                return transfo_loop_unnest;
            },
            InteractionTransformationKind::SortActionContent => {
                return transfo_sort_action_content;
            },
            InteractionTransformationKind::StrictToSeq => {
                return transfo_strict_to_seq;
            },
            InteractionTransformationKind::ParToSeq => {
                return transfo_par_to_seq;
            },
            InteractionTransformationKind::UnfoldActions => {
                return transfo_unfold_action;
            }
            // ***
            /*
            InteractionTransformationKind::MergeShiftLeft1 => {
                return transfo_merge_shift_left_1;
            },
            InteractionTransformationKind::MergeShiftLeft2 => {
                return transfo_merge_shift_left_2;
            },
            InteractionTransformationKind::MergeShiftRight1 => {
                return transfo_merge_shift_right_1;
            },
            InteractionTransformationKind::MergeShiftRight2 => {
                return transfo_merge_shift_right_2;
            },*/
        }
    }
}

