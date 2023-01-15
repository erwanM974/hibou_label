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

pub fn transfos_phase1() -> Vec<InteractionTransformationKind> {
    return vec![
        InteractionTransformationKind::Deduplicate,
        InteractionTransformationKind::Simpl,
        InteractionTransformationKind::FlushRight,
        InteractionTransformationKind::InvertPar,
        InteractionTransformationKind::InvertAlt,
        InteractionTransformationKind::LoopSimpl,
        InteractionTransformationKind::LoopUnNest,
        // ***
        InteractionTransformationKind::DeFactorizeLeft,
        InteractionTransformationKind::DeFactorizeRight,
        // ***
        InteractionTransformationKind::SortActionContent
    ];
}

pub fn transfos_phase2() -> Vec<InteractionTransformationKind> {
    return vec![
        InteractionTransformationKind::Deduplicate,
        InteractionTransformationKind::Simpl,
        InteractionTransformationKind::FlushRight,
        InteractionTransformationKind::InvertPar,
        InteractionTransformationKind::InvertAlt,
        InteractionTransformationKind::LoopSimpl,
        InteractionTransformationKind::LoopUnNest,
        // ***
        InteractionTransformationKind::FactorizePrefixStrict,
        InteractionTransformationKind::FactorizePrefixSeq,
        InteractionTransformationKind::FactorizeSuffixStrict,
        InteractionTransformationKind::FactorizeSuffixSeq,
        InteractionTransformationKind::FactorizeCommutativePar,
        // ***
        InteractionTransformationKind::SortActionContent
    ];
}

