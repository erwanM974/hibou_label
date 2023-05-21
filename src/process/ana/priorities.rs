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
use graph_process_manager_core::delegate::priorities::AbstractPriorities;
use crate::core::execution::trace::trace::TraceAction;
use crate::process::ana::step::AnalysisStepKind;


pub struct AnalysisPriorities {
    pub emission : i32,
    pub reception : i32,
    pub multi_rdv : i32,
    pub in_loop : i32,
    pub elim : i32,
    pub simu : i32
}

impl AnalysisPriorities {

    pub fn new(emission : i32,
               reception : i32,
               multi_rdv : i32,
               in_loop : i32,
               elim : i32,
               simu : i32) -> AnalysisPriorities {
        return AnalysisPriorities{emission,reception,multi_rdv,in_loop,elim,simu};
    }

    pub fn default() -> AnalysisPriorities {
        return AnalysisPriorities::new(0,0,0,0,1,-1);
    }
}

impl fmt::Display for AnalysisPriorities {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,
               "[emission={:},reception={:},multi-rdv={:},loop={:},elim={:},simu={:}]",
            self.emission,
            self.reception,
            self.multi_rdv,
            self.in_loop,
            self.elim,
            self.simu)
    }
}





impl AbstractPriorities<AnalysisStepKind> for AnalysisPriorities {
    fn get_priority_of_step(&self, step: &AnalysisStepKind) -> i32 {
        match *step {
            AnalysisStepKind::EliminateNoLongerObserved(ref to_elim) => {
                return self.elim*(to_elim.len() as i32);
            },
            AnalysisStepKind::Execute(ref frt_elt,ref consu_set,ref sim_map) => {
                let mut priority : i32 = 0;
                // ***
                let (num_em,num_rc) = TraceAction::get_actions_kinds(&frt_elt.target_actions);
                priority += num_em*self.emission;
                priority += num_rc*self.reception;
                // ***
                priority += self.multi_rdv * ( frt_elt.target_actions.len() as i32);
                priority += self.in_loop * ( frt_elt.max_loop_depth as i32);
                priority += self.simu * (sim_map.len() as i32);
                // ***
                return priority;
            }
        }
    }
}

