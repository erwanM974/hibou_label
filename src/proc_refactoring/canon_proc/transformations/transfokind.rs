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


use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::position::Position;
use crate::rendering::textual::monochrome::position::position_to_text;

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum CanonizationTransformationKind {
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
    FactorizePrefixStrict,
    FactorizePrefixSeq,
    FactorizePrefixPar,
    FactorizeSuffixStrict,
    FactorizeSuffixSeq,
    FactorizeSuffixPar,
    DeFactorizeLeft,
    DeFactorizeRight,
    LoopSimpl,
    LoopUnNest,
    SortEmissionTargets
}

impl CanonizationTransformationKind {
    pub fn to_string(&self) -> String {
        match self {
            &CanonizationTransformationKind::SimplLeft => {
                return "SimplLeft".to_string();
            },
            &CanonizationTransformationKind::SimplRight => {
                return "SimplRight".to_string();
            },
            &CanonizationTransformationKind::FlushLeft => {
                return "FlushLeft".to_string();
            },
            &CanonizationTransformationKind::FlushRight => {
                return "FlushRight".to_string();
            },
            &CanonizationTransformationKind::InvertAlt => {
                return "InvertAlt".to_string();
            },
            &CanonizationTransformationKind::InvertPar => {
                return "InvertPar".to_string();
            },
            &CanonizationTransformationKind::TriInvertAltRF => {
                return "TriInvertAltRF".to_string();
            },
            &CanonizationTransformationKind::TriInvertParRF => {
                return "TriInvertParRF".to_string();
            },
            &CanonizationTransformationKind::Deduplicate => {
                return "Deduplicate".to_string();
            },
            &CanonizationTransformationKind::TriDeduplicateRF => {
                return "TriDeduplicateRF".to_string();
            },
            &CanonizationTransformationKind::FactorizePrefixStrict => {
                return "FactorizePrefixStrict".to_string();
            },
            &CanonizationTransformationKind::FactorizePrefixSeq => {
                return "FactorizePrefixSeq".to_string();
            },
            &CanonizationTransformationKind::FactorizePrefixPar => {
                return "FactorizePrefixPar".to_string();
            },
            &CanonizationTransformationKind::FactorizeSuffixStrict => {
                return "FactorizeSuffixStrict".to_string();
            },
            &CanonizationTransformationKind::FactorizeSuffixSeq => {
                return "FactorizeSuffixSeq".to_string();
            },
            &CanonizationTransformationKind::FactorizeSuffixPar => {
                return "FactorizeSuffixPar".to_string();
            },
            &CanonizationTransformationKind::DeFactorizeLeft => {
                return "DeFactorizeLeft".to_string();
            },
            &CanonizationTransformationKind::DeFactorizeRight => {
                return "DeFactorizeRight".to_string();
            },
            &CanonizationTransformationKind::LoopSimpl => {
                return "LoopSimpl".to_string();
            },
            &CanonizationTransformationKind::LoopUnNest => {
                return "LoopUnNest".to_string();
            },
            &CanonizationTransformationKind::SortEmissionTargets => {
                return "SortEmissionTargets".to_string();
            }
        }
    }
}

pub struct CanonizationTransformation {
    pub kind : CanonizationTransformationKind,
    pub position : Position,
    pub result : Interaction
}

impl CanonizationTransformation {
    pub fn new(kind : CanonizationTransformationKind,
               position : Position,
               result : Interaction) -> CanonizationTransformation {
        return CanonizationTransformation{kind,position,result};
    }

    pub fn transformation_str_description(&self) -> String {
        return format!("{}@{}", self.kind.to_string(), position_to_text(&self.position))
    }
}