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

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum InconcReason {
    LackObs,
    UsingLifelineRemovalWithCoLocalizations,
    FilteredNodes
}

impl InconcReason {

    pub fn get_explanation_string(&self) -> String {
        match self {
            InconcReason::LackObs => {
                return "due to a lack of observation in the multi-trace (events not at the end globally may be missing) -> rather use hiding or simulation".to_string();
            },
            InconcReason::UsingLifelineRemovalWithCoLocalizations => {
                return "due to having non-singleton co-localizations on the multi-trace while using the lifeline-removal-based algorithm. WeakPasses may be false positives because using lifeline elimination may remove strict orderings between events occurring on distinct lifelines".to_string();
            },
            InconcReason::FilteredNodes => {
                return "due to having set a filter which forcefully limited exploration of the graph : Fails may be false negative".to_string();
            }
        }
    }

}

impl std::string::ToString for InconcReason {

    fn to_string(&self) -> String {
        match self {
            InconcReason::LackObs => {
                return "LackObs".to_string();
            },
            InconcReason::UsingLifelineRemovalWithCoLocalizations => {
                return "UsingLifelineRemovalWithCoLocalizations".to_string();
            },
            InconcReason::FilteredNodes => {
                return "FilteredNodes".to_string();
            }
        }
    }

}


pub enum CoverageVerdict{
    Cov,
    TooShort,
    MultiPref,
    Slice,
    Inconc(InconcReason),
    Out(bool),   // bool for if it's known via local analysis
    OutSim(bool) // bool for if it's known via local analysis
}

impl std::string::ToString for CoverageVerdict {

    fn to_string(&self) -> String {
        match self {
            CoverageVerdict::Cov => {
                return "Cov".to_string();
            },
            CoverageVerdict::TooShort => {
                return "TooShort".to_string();
            },
            CoverageVerdict::MultiPref => {
                return "MultiPref".to_string();
            },
            CoverageVerdict::Slice => {
                return "Slice".to_string();
            },
            CoverageVerdict::Inconc(reason) => {
                return format!("Inconc {:}", reason.to_string());
            },
            CoverageVerdict::Out(ref loc) => {
                if *loc {
                    return "Out-l".to_string();
                } else {
                    return "Out".to_string();
                }
            },
            CoverageVerdict::OutSim(ref loc) => {
                if *loc {
                    return "OutSim-l".to_string();
                } else {
                    return "OutSim".to_string();
                }
            }
        }
    }

}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum GlobalVerdict {
    Fail,
    WeakFail,
    Inconc(InconcReason),
    WeakPass,
    Pass
}

impl std::string::ToString for GlobalVerdict {
    fn to_string(&self) -> String {
        match self {
            GlobalVerdict::Pass => {
                return "Pass".to_string();
            },
            GlobalVerdict::WeakPass => {
                return "WeakPass".to_string();
            },
            GlobalVerdict::Inconc(reason) => {
                return format!("Inconc {:}", reason.to_string());
            },
            GlobalVerdict::WeakFail => {
                return "WeakFail".to_string();
            },
            GlobalVerdict::Fail => {
                return "Fail".to_string();
            }
        }
    }
}

pub fn update_global_verdict_from_new_coverage_verdict(glo:GlobalVerdict,cov:CoverageVerdict) -> GlobalVerdict {
    match glo {
        GlobalVerdict::Pass => {
            return GlobalVerdict::Pass;
        },
        GlobalVerdict::WeakPass => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                _ => {
                    return GlobalVerdict::WeakPass;
                }
            }
        },
        GlobalVerdict::Inconc(glo_reason) => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                // ***
                CoverageVerdict::TooShort => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::MultiPref => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::Slice => {
                    return GlobalVerdict::WeakPass;
                },
                _ => {
                    return GlobalVerdict::Inconc(glo_reason);
                }
            }
        },
        GlobalVerdict::WeakFail => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                // ***
                CoverageVerdict::TooShort => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::MultiPref => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::Slice => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::Inconc(reason) => {
                    return GlobalVerdict::Inconc(reason);
                },
                _ => {
                    return GlobalVerdict::WeakFail;
                }
            }
        },
        GlobalVerdict::Fail => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                // ***
                CoverageVerdict::TooShort => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::MultiPref => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::Slice => {
                    return GlobalVerdict::WeakPass;
                },
                // ***
                CoverageVerdict::Inconc(reason) => {
                    return GlobalVerdict::Inconc(reason);
                },
                CoverageVerdict::OutSim(_) => {
                    return GlobalVerdict::WeakFail;
                },
                _ => {
                    return GlobalVerdict::Fail;
                }
            }
        }
    }
}
