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



use std::collections::{HashMap, HashSet};
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::execution::trace::trace::TraceAction;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::process::ana_proc::logic::anakind::{SimulationActionCriterion, SimulationConfiguration, SimulationLoopCriterion};


#[derive(Clone, PartialEq, Debug)]
pub struct TraceAnalysisFlags {
    pub consumed : usize,
    pub hidden : bool,
    pub dirty4local : bool,
    pub simulated_before : u32,
    pub simulated_after : u32
}

impl TraceAnalysisFlags {
    pub fn new(
        consumed : usize,
        hidden : bool,
        dirty4local : bool,
        simulated_before : u32,
        simulated_after : u32) -> TraceAnalysisFlags {
        return TraceAnalysisFlags{consumed,hidden,dirty4local,simulated_before,simulated_after};
    }

    pub fn new_init() -> TraceAnalysisFlags {
        return TraceAnalysisFlags::new(0,false,true,0,0);
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct MultiTraceAnalysisFlags {
    pub canals : Vec<TraceAnalysisFlags>,
    pub rem_loop_in_sim : u32,
    pub rem_act_in_sim : u32
}

pub enum WasMultiTraceConsumedWithSimulation {
    No,
    OnlyAfterEnd,
    AsSlice
}

impl MultiTraceAnalysisFlags {

    pub fn new_init(canals_num : usize,
                    rem_loop_in_sim : u32,
                    rem_act_in_sim : u32) -> MultiTraceAnalysisFlags {
        let mut canals : Vec<TraceAnalysisFlags> = vec![];
        for i in 0..canals_num {
            canals.push(TraceAnalysisFlags::new_init());
        }
        return MultiTraceAnalysisFlags::new(canals,rem_loop_in_sim,rem_act_in_sim);
    }

    pub fn new(canals:Vec<TraceAnalysisFlags>,
               rem_loop_in_sim : u32,
               rem_act_in_sim : u32) -> MultiTraceAnalysisFlags {
        return MultiTraceAnalysisFlags{canals,rem_loop_in_sim,rem_act_in_sim};
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn is_any_component_empty(&self, multi_trace : &MultiTrace) -> bool {
        for (canal_id,canal_flags) in self.canals.iter().enumerate() {
            let trace= multi_trace.get(canal_id).unwrap();
            if trace.len() == canal_flags.consumed {
                return true;
            }
        }
        return false;
    }

    pub fn is_multi_trace_empty(&self, multi_trace : &MultiTrace) -> bool {
        for (canal_id,canal_flags) in self.canals.iter().enumerate() {
            let trace = multi_trace.get(canal_id).unwrap();
            if trace.len() > canal_flags.consumed {
                return false;
            }
        }
        return true;
    }

    pub fn is_any_component_hidden(&self) -> bool {
        for canal in &self.canals {
            if canal.hidden {
                return true;
            }
        }
        return false;
    }

    pub fn is_simulated(&self) -> WasMultiTraceConsumedWithSimulation {
        let mut got_sim_after = false;
        for canal in &self.canals {
            if canal.simulated_before > 0 {
                return WasMultiTraceConsumedWithSimulation::AsSlice;
            }
            if canal.simulated_after > 0 {
                got_sim_after = true;
            }
        }
        if got_sim_after {
            return WasMultiTraceConsumedWithSimulation::OnlyAfterEnd;
        } else {
            return WasMultiTraceConsumedWithSimulation::No;
        }
    }

    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********
    // ********** ********** ********** ********** ********** ********** **********

    pub fn update_on_hide(&self,
                          gen_ctx : &GeneralContext,
                          coloc_ids_to_hide : &HashSet<usize>) -> MultiTraceAnalysisFlags {
        let mut new_canal_flags : Vec<TraceAnalysisFlags> = Vec::new();
        // ***
        for (flag_id,old_flag) in self.canals.iter().enumerate() {
            let mut new_flag : TraceAnalysisFlags = old_flag.clone();
            if coloc_ids_to_hide.contains(&flag_id) {
                assert!(!new_flag.hidden);
                new_flag.hidden = true;
            }
            new_canal_flags.push(new_flag);
        }
        return MultiTraceAnalysisFlags::new(new_canal_flags,self.rem_loop_in_sim,self.rem_act_in_sim);
    }

    pub fn update_on_execution(&self,
                                sim_config : Option<&SimulationConfiguration>,
                                consu_set : &HashSet<usize>, // ids of canals on which concrete execution occur
                                sim_map : &HashMap<usize,SimulationStepKind>, // id of canals on which simulation (of which kind) occur
                                affected_colocs : &HashSet<usize>, // ids of canals containing lifelines affected by the execution of the action
                                loop_depth : u32, // loop depth of action that is executed
                                new_interaction : &Interaction) -> MultiTraceAnalysisFlags {
        // ***
        let mut new_canal_flags : Vec<TraceAnalysisFlags> = Vec::new();
        // ***
        for (flag_id,old_flag) in self.canals.iter().enumerate() {
            let mut new_flag : TraceAnalysisFlags = old_flag.clone();
            // ***
            if affected_colocs.contains(&flag_id) {
                new_flag.dirty4local = true;
            }
            // ***
            if consu_set.contains(&flag_id) {
                new_flag.consumed += 1;
            }
            // ***
            match sim_map.get(&flag_id) {
                None => {},
                Some( sim_kind ) => {
                    match sim_kind {
                        SimulationStepKind::BeforeStart => {
                            new_flag.simulated_before += 1;
                        },
                        SimulationStepKind::AfterEnd => {
                            new_flag.simulated_after += 1;
                        }
                    }
                }
            }
            // ***
            new_canal_flags.push( new_flag );
        }
        // ***
        let (rem_loop_in_sim,rem_act_in_sim) : (u32,u32);
        match sim_config {
            None => {
                rem_loop_in_sim = 0;
                rem_act_in_sim = 0;
            },
            Some( got_sim_config ) => {
                let (rem_loop,rem_act) = self.get_rem_act_loop_in_sim(got_sim_config,new_interaction,consu_set,loop_depth);
                rem_loop_in_sim = rem_loop;
                rem_act_in_sim = rem_act;
            }
        }
        // ***
        return MultiTraceAnalysisFlags::new(new_canal_flags,rem_loop_in_sim,rem_act_in_sim);
    }

    fn get_rem_act_loop_in_sim(&self,
                               sim_config : &SimulationConfiguration,
                               new_interaction : &Interaction,
                               consu_set : &HashSet<usize>,
                               loop_depth : u32) -> (u32,u32) {
        let rem_loop_in_sim : u32;
        match sim_config.loop_crit {
            SimulationLoopCriterion::MaxDepth => {
                if consu_set.len() > 0 {
                    rem_loop_in_sim = new_interaction.max_nested_loop_depth();
                } else {
                    let on_crit = new_interaction.max_nested_loop_depth();
                    let removed = self.rem_loop_in_sim - loop_depth;
                    rem_loop_in_sim = on_crit.min(removed);
                }
            },
            SimulationLoopCriterion::MaxNum => {
                if consu_set.len() > 0 {
                    rem_loop_in_sim = new_interaction.total_loop_num();
                } else {
                    let on_crit = new_interaction.total_loop_num();
                    let removed = self.rem_loop_in_sim - loop_depth;
                    rem_loop_in_sim = on_crit.min(removed);
                }
            },
            SimulationLoopCriterion::SpecificNum( sn ) => {
                if consu_set.len() > 0 {
                    rem_loop_in_sim = sn;
                } else {
                    let on_crit = sn;
                    let removed = self.rem_loop_in_sim - loop_depth;
                    rem_loop_in_sim = on_crit.min(removed);
                }
            },
            SimulationLoopCriterion::None => {
                rem_loop_in_sim = 0;
            }
        }
        // ***
        let rem_act_in_sim : u32;
        match sim_config.act_crit {
            SimulationActionCriterion::SpecificNum( sn ) => {
                if consu_set.len() > 0 {
                    rem_act_in_sim = sn;
                } else {
                    let on_crit = sn;
                    let removed = self.rem_act_in_sim - 1;
                    rem_act_in_sim = on_crit.min(removed);
                }
            },
            SimulationActionCriterion::None => {
                rem_act_in_sim = 0;
            }
        }
        // ***
        return (rem_loop_in_sim,rem_act_in_sim)
    }
}


