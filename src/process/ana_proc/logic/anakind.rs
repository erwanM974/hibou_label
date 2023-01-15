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
                return "maximum loop number".to_string();
            },
            SimulationLoopCriterion::MaxDepth => {
                return "maximum loop depth".to_string();
            },
            SimulationLoopCriterion::SpecificNum(sn) => {
                return format!("loop num : {:}", sn);
            },
            SimulationLoopCriterion::None => {
                return String::new();
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum SimulationActionCriterion {
    SpecificNum(u32),
    None
}

impl std::string::ToString for SimulationActionCriterion {
    fn to_string(&self) -> String {
        match self {
            SimulationActionCriterion::SpecificNum(sn) => {
                return format!("action num : {:}", sn);
            },
            SimulationActionCriterion::None => {
                return String::new();
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct SimulationConfiguration {
    pub sim_before : bool,
    pub loop_crit : SimulationLoopCriterion,
    pub act_crit : SimulationActionCriterion
}

impl std::string::ToString for SimulationConfiguration {
    fn to_string(&self) -> String {
        return format!("sim_before : {:} | {:} | {:}", self.sim_before, self.loop_crit.to_string(), self.act_crit.to_string());
    }
}

impl SimulationConfiguration {

    pub fn get_reset_rem_loop(&self,
                       interaction : &Interaction) -> u32 {
        match self.loop_crit {
            SimulationLoopCriterion::MaxDepth => {
                return interaction.max_nested_loop_depth();
            },
            SimulationLoopCriterion::MaxNum => {
                return interaction.total_loop_num();
            },
            SimulationLoopCriterion::SpecificNum( sn ) => {
                return sn;
            },
            SimulationLoopCriterion::None => {
                return 0;
            }
        }
    }
    pub fn get_reset_rem_act(&self,
                              interaction : &Interaction) -> u32 {
        match self.act_crit {
            SimulationActionCriterion::SpecificNum( sn ) => {
                return sn;
            },
            SimulationActionCriterion::None => {
                return 0;
            }
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum AnalysisKind {
    Accept,
    Prefix,
    Hide,
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
            AnalysisKind::Hide => {
                return "hide".to_string();
            },
            AnalysisKind::Simulate(sim_config) => {
                return format!("simulate[{:}]", sim_config.to_string());
            }
        }
    }
}



#[derive(Clone, PartialEq, Debug)]
pub enum UseLocalAnalysis {
    No,
    Yes
}


impl std::string::ToString for UseLocalAnalysis {
    fn to_string(&self) -> String {
        match self {
            UseLocalAnalysis::No => {
                return "No".to_string();
            },
            UseLocalAnalysis::Yes => {
                return "Yes".to_string();
            }
        }
    }
}

