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
pub enum TransformationKind {
    SimplLeft,
    SimplRight,
    FlushLeft,
    FlushRight,
    InvertAlt,
    InvertPar,
    TriInvertAltRF,
    TriInvertParRF,
    Deduplicate,
    TriDeduplicateRF,
    FactorizePrefixX,
    FactorizePrefixS,
    FactorizePrefixP,
    FactorizeSuffixX,
    FactorizeSuffixS,
    FactorizeSuffixP,
    DeFactorizeL,
    DeFactorizeR,
    LoopSimpl,
    LoopUnNest,
    StrictToPassing,
    SortEmissionTargets,
    MergeAndLeft1,
    MergeAndRight1,
    MergeAndLeft2,
    MergeAndRight2,
    MergeAction,
    MergeSkip
}

impl TransformationKind {
    pub fn to_string(&self) -> String {
        match self {
            &TransformationKind::SimplLeft => {
                return "SimplLeft".to_string();
            },
            &TransformationKind::SimplRight => {
                return "SimplRight".to_string();
            },
            &TransformationKind::FlushLeft => {
                return "FlushLeft".to_string();
            },
            &TransformationKind::FlushRight => {
                return "FlushRight".to_string();
            },
            &TransformationKind::InvertAlt => {
                return "InvertAlt".to_string();
            },
            &TransformationKind::InvertPar => {
                return "InvertPar".to_string();
            },
            &TransformationKind::TriInvertAltRF => {
                return "TriInvertAltRF".to_string();
            },
            &TransformationKind::TriInvertParRF => {
                return "TriInvertParRF".to_string();
            },
            &TransformationKind::Deduplicate => {
                return "Deduplicate".to_string();
            },
            &TransformationKind::TriDeduplicateRF => {
                return "TriDeduplicateRF".to_string();
            },
            &TransformationKind::FactorizePrefixX => {
                return "FactorizePrefixX".to_string();
            },
            &TransformationKind::FactorizePrefixS => {
                return "FactorizePrefixS".to_string();
            },
            &TransformationKind::FactorizePrefixP => {
                return "FactorizePrefixP".to_string();
            },
            &TransformationKind::FactorizeSuffixX => {
                return "FactorizeSuffixX".to_string();
            },
            &TransformationKind::FactorizeSuffixS => {
                return "FactorizeSuffixS".to_string();
            },
            &TransformationKind::FactorizeSuffixP => {
                return "FactorizeSuffixP".to_string();
            },
            &TransformationKind::DeFactorizeL => {
                return "DeFactorizeL".to_string();
            },
            &TransformationKind::DeFactorizeR => {
                return "DeFactorizeR".to_string();
            },
            &TransformationKind::LoopSimpl => {
                return "LoopSimpl".to_string();
            },
            &TransformationKind::LoopUnNest => {
                return "LoopUnNest".to_string();
            },
            &TransformationKind::StrictToPassing => {
                return "StrictToPassing".to_string();
            },
            &TransformationKind::SortEmissionTargets => {
                return "SortEmissionTargets".to_string();
            },
            &TransformationKind::MergeAndLeft1 => {
                return "MergeAndLeft1".to_string();
            },
            &TransformationKind::MergeAndRight1 => {
                return "MergeAndRight1".to_string();
            },
            &TransformationKind::MergeAndLeft2 => {
                return "MergeAndLeft2".to_string();
            },
            &TransformationKind::MergeAndRight2 => {
                return "MergeAndRight2".to_string();
            },
            &TransformationKind::MergeAction => {
                return "MergeAction".to_string();
            },
            &TransformationKind::MergeSkip => {
                return "MergeSkip".to_string();
            }
        }
    }
}

