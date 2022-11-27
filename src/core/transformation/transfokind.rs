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


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum InteractionTransformationKind {
    Simpl,
    FlushLeft,
    FlushRight,
    InvertAlt,
    InvertPar,
    Deduplicate,
    FactorizePrefixStrict,
    FactorizePrefixSeq,
    FactorizeSuffixStrict,
    FactorizeSuffixSeq,
    FactorizeCommutativePar,
    DeFactorizeLeft,
    DeFactorizeRight,
    LoopSimpl,
    LoopAltSimpl,
    LoopUnNest,
    SortActionContent, // sort emission targets OR reception recipients
    // ***
    MergeShiftLeft1,
    MergeShiftLeft2,
    MergeShiftRight1,
    MergeShiftRight2,
    MergeAction,
    MergeSkip,
    MergeSkipInvert,
}


impl InteractionTransformationKind {
    pub fn to_string(&self) -> String {
        match self {
            &InteractionTransformationKind::Simpl => {
                return "Simpl".to_string();
            },
            &InteractionTransformationKind::FlushLeft => {
                return "FlushLeft".to_string();
            },
            &InteractionTransformationKind::FlushRight => {
                return "FlushRight".to_string();
            },
            &InteractionTransformationKind::InvertAlt => {
                return "InvertAlt".to_string();
            },
            &InteractionTransformationKind::InvertPar => {
                return "InvertPar".to_string();
            },
            &InteractionTransformationKind::Deduplicate => {
                return "Deduplicate".to_string();
            },
            &InteractionTransformationKind::FactorizePrefixStrict => {
                return "FactorizePrefixStrict".to_string();
            },
            &InteractionTransformationKind::FactorizePrefixSeq => {
                return "FactorizePrefixSeq".to_string();
            },
            &InteractionTransformationKind::FactorizeSuffixStrict => {
                return "FactorizeSuffixStrict".to_string();
            },
            &InteractionTransformationKind::FactorizeSuffixSeq => {
                return "FactorizeSuffixSeq".to_string();
            },
            &InteractionTransformationKind::FactorizeCommutativePar => {
                return "FactorizeCommutativePar".to_string();
            },
            &InteractionTransformationKind::DeFactorizeLeft => {
                return "DeFactorizeLeft".to_string();
            },
            &InteractionTransformationKind::DeFactorizeRight => {
                return "DeFactorizeRight".to_string();
            },
            &InteractionTransformationKind::LoopSimpl => {
                return "LoopSimpl".to_string();
            },
            &InteractionTransformationKind::LoopAltSimpl => {
                return "LoopAltSimpl".to_string();
            },
            &InteractionTransformationKind::LoopUnNest => {
                return "LoopUnNest".to_string();
            },
            &InteractionTransformationKind::SortActionContent => {
                return "SortActionContent".to_string();
            },
            &InteractionTransformationKind::MergeShiftLeft1 => {
                return "MergeShiftLeft1".to_string();
            },
            &InteractionTransformationKind::MergeShiftRight1 => {
                return "MergeShiftRight1".to_string();
            },
            &InteractionTransformationKind::MergeShiftLeft2 => {
                return "MergeShiftLeft2".to_string();
            },
            &InteractionTransformationKind::MergeShiftRight2 => {
                return "MergeShiftRight2".to_string();
            },
            &InteractionTransformationKind::MergeAction => {
                return "MergeAction".to_string();
            },
            &InteractionTransformationKind::MergeSkip => {
                return "MergeSkip".to_string();
            },
            &InteractionTransformationKind::MergeSkipInvert => {
                return "MergeSkipInvert".to_string();
            }
        }
    }
}

