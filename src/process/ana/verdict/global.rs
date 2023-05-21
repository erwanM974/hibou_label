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
use graph_process_manager_core::manager::verdict::AbstractGlobalVerdict;

use crate::process::ana::verdict::inconc::InconcReason;
use crate::process::ana::verdict::local::AnalysisLocalVerdict;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum AnalysisGlobalVerdict{
    Fail,
    WeakFail,
    Inconc(InconcReason),
    WeakPass,
    Pass
}

impl fmt::Display for AnalysisGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnalysisGlobalVerdict::Pass => {
                write!(f,"Pass")
            },
            AnalysisGlobalVerdict::WeakPass => {
                write!(f,"WeakPass")
            },
            AnalysisGlobalVerdict::Inconc(reason) => {
                write!(f,"Inconc {:}", reason.to_string())
            },
            AnalysisGlobalVerdict::WeakFail => {
                write!(f,"WeakFail")
            },
            AnalysisGlobalVerdict::Fail => {
                write!(f,"Fail")
            }
        }
    }
}

impl AbstractGlobalVerdict<AnalysisLocalVerdict> for AnalysisGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        true
    }

    fn get_baseline_verdict() -> Self {
        AnalysisGlobalVerdict::Fail
    }

    fn update_with_local_verdict(self, local_verdict: &AnalysisLocalVerdict) -> Self {
        match self {
            AnalysisGlobalVerdict::Pass => {
                self
            },
            AnalysisGlobalVerdict::WeakPass => {
                match local_verdict {
                    AnalysisLocalVerdict::Cov => {
                        AnalysisGlobalVerdict::Pass
                    },
                    _ => {
                        self
                    }
                }
            },
            AnalysisGlobalVerdict::Inconc(_) => {
                match local_verdict {
                    AnalysisLocalVerdict::Cov => {
                        AnalysisGlobalVerdict::Pass
                    },
                    // ***
                    AnalysisLocalVerdict::TooShort => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    AnalysisLocalVerdict::MultiPref => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    AnalysisLocalVerdict::Slice => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    _ => {
                        self
                    }
                }
            },
            AnalysisGlobalVerdict::WeakFail => {
                match local_verdict {
                    AnalysisLocalVerdict::Cov => {
                        AnalysisGlobalVerdict::Pass
                    },
                    // ***
                    AnalysisLocalVerdict::TooShort => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    AnalysisLocalVerdict::MultiPref => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    AnalysisLocalVerdict::Slice => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    AnalysisLocalVerdict::Inconc(reason) => {
                        AnalysisGlobalVerdict::Inconc(reason.clone())
                    },
                    _ => {
                        self
                    }
                }
            },
            AnalysisGlobalVerdict::Fail => {
                match local_verdict {
                    AnalysisLocalVerdict::Cov => {
                        AnalysisGlobalVerdict::Pass
                    },
                    // ***
                    AnalysisLocalVerdict::TooShort => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    AnalysisLocalVerdict::MultiPref => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    AnalysisLocalVerdict::Slice => {
                        AnalysisGlobalVerdict::WeakPass
                    },
                    // ***
                    AnalysisLocalVerdict::Inconc(reason) => {
                        AnalysisGlobalVerdict::Inconc(reason.clone())
                    },
                    AnalysisLocalVerdict::OutSim(_) => {
                        AnalysisGlobalVerdict::WeakFail
                    },
                    _ => {
                        self
                    }
                }
            }
        }
    }

    fn is_goal_reached(&self, goal: &Option<Self>) -> bool {
        match goal.as_ref() {
            None => {
                false
            },
            Some( target_goal ) => {
                self >= target_goal
            }
        }
    }

    fn update_knowing_nodes_were_filtered_out(self, has_filtered_nodes: bool) -> Self {
        if self == AnalysisGlobalVerdict::Fail && has_filtered_nodes {
            AnalysisGlobalVerdict::Inconc(InconcReason::FilteredNodes)
        } else {
            self
        }
    }
}


