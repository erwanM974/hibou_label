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



use std::fmt;
use graph_process_manager_core::manager::config::AbstractProcessParameterization;
use crate::process::ana::param::anakind::AnalysisKind;


#[derive(Clone, Copy, PartialEq, Eq)]
pub enum LocalAnalysisLifelineSelectionPolicy {
    SelectAll,
    OnlyOnImpactedByLastStep
}

impl fmt::Display for LocalAnalysisLifelineSelectionPolicy {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalAnalysisLifelineSelectionPolicy::SelectAll => {
                write!(f,"Systematic On All Lifelines")
            },
            LocalAnalysisLifelineSelectionPolicy::OnlyOnImpactedByLastStep => {
                write!(f,"On Dirty Lifelines")
            }
        }
    }

}


pub struct LocalAnalysisParameterization {
    pub on_lifeline_policy : LocalAnalysisLifelineSelectionPolicy,
    pub max_look_ahead_depth : Option<u32>,
    pub modulo_each_X_steps : u32
}

impl fmt::Display for LocalAnalysisParameterization {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(depth) = self.max_look_ahead_depth {
            write!(f,"[select={:}, modulo_each={:}, max_look_ahead_depth={:}]", self.on_lifeline_policy, self.modulo_each_X_steps, depth)
        } else {
            write!(f,"[select={:}], modulo_each={:}, max_look_ahead_depth=none]", self.on_lifeline_policy, self.modulo_each_X_steps)
        }
    }

}

impl LocalAnalysisParameterization {
    pub fn new(
        on_lifeline_policy: LocalAnalysisLifelineSelectionPolicy,
        max_look_ahead_depth: Option<u32>,
        modulo_each_X_steps : u32) -> Self {
        Self { on_lifeline_policy, max_look_ahead_depth, modulo_each_X_steps }
    }
}


pub struct AnalysisParameterization {
    pub ana_kind : AnalysisKind,
    pub locana : Option<LocalAnalysisParameterization>,
    pub partial_order_reduction : bool
}



impl AnalysisParameterization {
    pub fn new(ana_kind: AnalysisKind,
               locana: Option<LocalAnalysisParameterization>,
               partial_order_reduction : bool) -> Self {
        AnalysisParameterization{ana_kind, locana, partial_order_reduction}
    }
}

impl AbstractProcessParameterization for AnalysisParameterization {
    fn get_param_as_strings(&self) -> Vec<String> {
        let mut got = vec![
            "process = analysis".to_string(),
            format!("analysis kind = {}", self.ana_kind.to_string())
        ];
        if let Some(locana_param) = &self.locana {
            got.push(
                format!("local analysis = {:}", locana_param)
            );
        } else {
            got.push(
                "local analysis = none".to_string()
            );
        }
        got.push(
            format!("partial order reduction = {:}", self.partial_order_reduction)
        );
        got
    }
}