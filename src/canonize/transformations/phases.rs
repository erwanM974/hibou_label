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

use crate::canonize::transformations::transfokind::*;
use crate::canonize::transformations::transfodef::*;

pub struct InteractionTermTransformation {
    pub kind : TransformationKind,
    pub position : Position,
    pub result : Interaction
}

impl InteractionTermTransformation {
    pub fn new(kind : TransformationKind,
               position : Position,
               result : Interaction) -> InteractionTermTransformation {
        return InteractionTermTransformation{kind,position,result};
    }

    pub fn transformation_str_description(&self) -> String {
        return format!("{}@{}", self.kind.to_string(), position_to_text(&self.position))
    }
}

pub fn transfos_phase1<'lifetime>() -> Vec<(TransformationKind, &'lifetime dyn Fn(&Interaction) -> Option<Interaction>)> {
    return vec![
        (TransformationKind::SimplLeft,&simpl_left),
        (TransformationKind::SimplRight,&simpl_right),
        (TransformationKind::FlushRight,&flush_right),
        (TransformationKind::InvertPar,&invert_par_conditional),
        (TransformationKind::TriInvertParRF,&tri_invert_par_conditional_right_flushed),
        (TransformationKind::InvertAlt,&invert_alt_conditional),
        (TransformationKind::TriInvertAltRF,&tri_invert_alt_conditional_right_flushed),
        (TransformationKind::LoopSimpl,&loop_simpl),
        (TransformationKind::LoopUnNest,&loop_unnest),
        (TransformationKind::DeFactorizeL,&defactorize_left),
        (TransformationKind::DeFactorizeR,&defactorize_right)
    ];
}

pub fn transfos_phase2<'lifetime>() -> Vec<(TransformationKind, &'lifetime dyn Fn(&Interaction) -> Option<Interaction>)> {
    return vec![
        (TransformationKind::Deduplicate,&deduplicate),
        (TransformationKind::TriDeduplicateRF,&tri_deduplicate_right_flushed)
    ];
}

pub fn transfos_phase3<'lifetime>() -> Vec<(TransformationKind, &'lifetime dyn Fn(&Interaction) -> Option<Interaction>)> {
    return vec![
        (TransformationKind::SimplLeft,&loop_simpl),
        (TransformationKind::SimplRight,&simpl_right),
        (TransformationKind::FlushRight,&flush_right),
        (TransformationKind::InvertPar,&invert_par_conditional),
        (TransformationKind::TriInvertParRF,&tri_invert_par_conditional_right_flushed),
        (TransformationKind::InvertAlt,&invert_alt_conditional),
        (TransformationKind::TriInvertAltRF,&tri_invert_alt_conditional_right_flushed),
        (TransformationKind::LoopSimpl,&loop_simpl),
        (TransformationKind::LoopUnNest,&loop_unnest),
        (TransformationKind::FactorizePrefixX,&factorize_prefix_strict),
        (TransformationKind::FactorizePrefixS,&factorize_prefix_seq),
        (TransformationKind::FactorizePrefixP,&factorize_prefix_par),
        (TransformationKind::FactorizeSuffixX,&factorize_suffix_strict),
        (TransformationKind::FactorizeSuffixS,&factorize_suffix_seq),
        (TransformationKind::FactorizeSuffixP,&factorize_suffix_par)
    ];
}

