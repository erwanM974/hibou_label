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

use crate::core::execution::trace::from_model::from_model::InteractionInterpretableAsTraceAction;
use crate::core::language::syntax::interaction::Interaction;

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationLoopCriterion {
    MaxNum,
    MaxDepth,
    SpecificNum(u32),
    None
}

impl std::string::ToString for SimulationLoopCriterion {
    fn to_string(&self) -> String {
        match self {
            SimulationLoopCriterion::MaxNum => {
                return "total number of loops in interaction".to_string();
            },
            SimulationLoopCriterion::MaxDepth => {
                return "maximum depth of nested loops in interaction".to_string();
            },
            SimulationLoopCriterion::SpecificNum(sn) => {
                return format!("specific number of loops : {:}", sn);
            },
            SimulationLoopCriterion::None => {
                return "no limit on loops".to_string();
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationActionCriterion {
    MaxNumOutsideLoops,
    SpecificNum(u32),
    None
}

impl std::string::ToString for SimulationActionCriterion {
    fn to_string(&self) -> String {
        match self {
            SimulationActionCriterion::MaxNumOutsideLoops => {
                return "number of actions outside loops".to_string();
            },
            SimulationActionCriterion::SpecificNum(sn) => {
                return format!("specific number of actions : {:}", sn);
            },
            SimulationActionCriterion::None => {
                return "no limit on actions".to_string();
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SimulationConfiguration {
    pub sim_before : bool,
    pub reset_crit_after_exec : bool,
    pub multiply_by_multitrace_length : bool,
    pub loop_crit : SimulationLoopCriterion,
    pub act_crit : SimulationActionCriterion
}

impl std::string::ToString for SimulationConfiguration {
    fn to_string(&self) -> String {
        return format!("sim before/slice : {:} | reset after exec : {:} | multiply by mu length : {:} | {:} | {:}",
                       self.sim_before,
                       self.reset_crit_after_exec,
                       self.multiply_by_multitrace_length,
                       self.loop_crit.to_string(),
                       self.act_crit.to_string());
    }
}

impl SimulationConfiguration {

    pub fn new(sim_before : bool,
               reset_crit_after_exec : bool,
               multiply_by_multitrace_length : bool,
               loop_crit : SimulationLoopCriterion,
               act_crit : SimulationActionCriterion) -> SimulationConfiguration {
        return SimulationConfiguration{sim_before,reset_crit_after_exec,multiply_by_multitrace_length,loop_crit,act_crit};
    }

    pub fn get_reset_rem_loop(&self,
                              multi_trace_len : usize,
                              interaction : &Interaction) -> u32 {
        let num : u32;
        match self.loop_crit {
            SimulationLoopCriterion::MaxDepth => {
                num = interaction.max_nested_loop_depth();
            },
            SimulationLoopCriterion::MaxNum => {
                num = interaction.total_loop_num();
            },
            SimulationLoopCriterion::SpecificNum( sn ) => {
                num = sn;
            },
            SimulationLoopCriterion::None => {
                num = 0;
            }
        }
        if self.multiply_by_multitrace_length {
            return num * (multi_trace_len as u32);
        } else {
            return num;
        }
    }
    pub fn get_reset_rem_act(&self,
                             multi_trace_len : usize,
                              interaction : &Interaction) -> u32 {
        let num : u32;
        match self.act_crit {
            SimulationActionCriterion::MaxNumOutsideLoops => {
                num = interaction.get_atomic_actions_number_outside_loops() as u32;
            },
            SimulationActionCriterion::SpecificNum( sn ) => {
                num = sn;
            },
            SimulationActionCriterion::None => {
                num = 0;
            }
        }
        if self.multiply_by_multitrace_length {
            return num * (multi_trace_len as u32);
        } else {
            return num;
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AnalysisKind {
    Accept,
    Prefix,
    Eliminate,
    Simulate(SimulationConfiguration)
}

impl AnalysisKind {

    pub fn get_sim_crits(&self) -> (bool,bool,bool) {
        let has_simulation : bool;
        let sim_crit_loop : bool;
        let sim_crit_act : bool;
        match self {
            AnalysisKind::Simulate(config) => {
                has_simulation = true;
                match &config.loop_crit {
                    SimulationLoopCriterion::None => {
                        sim_crit_loop = false;
                    },
                    _ => {
                        sim_crit_loop = true;
                    }
                }
                match &config.act_crit {
                    SimulationActionCriterion::None => {
                        sim_crit_act = false;
                    },
                    _ => {
                        sim_crit_act = true;
                    }
                }
            },
            _ => {
                has_simulation = false;
                sim_crit_loop = false;
                sim_crit_act = false;
            }
        }
        return (has_simulation,sim_crit_loop,sim_crit_act);
    }

    pub fn get_sim_config(&self) -> Option<&SimulationConfiguration> {
        match self {
            AnalysisKind::Simulate(config) => {
                return Some(config);
            },
            _ => {
                return None;
            }
        }
    }

    pub fn has_simulation(&self) -> bool {
        match self {
            AnalysisKind::Simulate(_) => {
                return true;
            },
            _ => {
                return false;
            }
        }
    }

    pub fn sim_before(&self) -> bool {
        match self {
            AnalysisKind::Simulate(config) => {
                return config.sim_before;
            },
            _ => {
                return false;
            }
        }
    }
}


impl std::string::ToString for AnalysisKind {
    fn to_string(&self) -> String {
        match self {
            AnalysisKind::Accept => {
                return "accept".to_string();
            },
            AnalysisKind::Prefix => {
                return "prefix".to_string();
            },
            AnalysisKind::Eliminate => {
                return "eliminate no-longer-observed".to_string();
            },
            AnalysisKind::Simulate(sim_config) => {
                return format!("simulate[{:}]", sim_config.to_string());
            }
        }
    }
}


