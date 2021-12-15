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

#[derive(Clone, PartialEq, Debug)]
pub enum AnalysisKind {
    Accept,
    Prefix,
    Hide,
    Simulate(bool)
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
            AnalysisKind::Simulate(sim_before) => {
                if *sim_before {
                    return "simulate multi-slices".to_string();
                } else {
                    return "simulate multi-prefixes".to_string();
                }
            }
        }
    }
}



#[derive(Clone, PartialEq, Debug)]
pub enum UseLocalAnalysis {
    No,
    OnlyFront,
    Yes
}


impl std::string::ToString for UseLocalAnalysis {
    fn to_string(&self) -> String {
        match self {
            UseLocalAnalysis::No => {
                return "No".to_string();
            },
            UseLocalAnalysis::OnlyFront => {
                return "OnlyFront".to_string();
            },
            UseLocalAnalysis::Yes => {
                return "Yes".to_string();
            }
        }
    }
}

