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



pub enum CoverageVerdict{
    Cov,
    TooShort,
    LackObs,
    UnCov,
    Out
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
            CoverageVerdict::LackObs => {
                return "LackObs".to_string();
            },
            CoverageVerdict::UnCov => {
                return "UnCov".to_string();
            },
            CoverageVerdict::Out => {
                return "Out".to_string();
            }
        }
    }

}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum GlobalVerdict {
    Fail,
    Inconc,
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
            GlobalVerdict::Inconc => {
                return "Inconc".to_string();
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
        GlobalVerdict::Inconc => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                CoverageVerdict::TooShort => {
                    return GlobalVerdict::WeakPass;
                },
                _ => {
                    return GlobalVerdict::Inconc;
                }
            }
        },
        GlobalVerdict::Fail => {
            match cov {
                CoverageVerdict::Cov => {
                    return GlobalVerdict::Pass;
                },
                CoverageVerdict::TooShort => {
                    return GlobalVerdict::WeakPass;
                },
                CoverageVerdict::LackObs => {
                    return GlobalVerdict::Inconc;
                },
                _ => {
                    return GlobalVerdict::Fail;
                }
            }
        }
    }
}
