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

use crate::process::verdicts::CoverageVerdict;

use crate::rendering::graphviz::common::GraphvizColor;


impl CoverageVerdict {
    pub fn get_verdict_color(&self) -> GraphvizColor {
        match self {
            CoverageVerdict::Cov => {
                return GraphvizColor::blue3; // 0 0 205
            },
            CoverageVerdict::TooShort => {
                return GraphvizColor::cyan3; // 0 205 205
            },
            CoverageVerdict::MultiPref => {
                return GraphvizColor::slateblue3; // 105 89 205
            },
            CoverageVerdict::Slice => {
                return GraphvizColor::darkorchid3; // 154 50 205
            },
            CoverageVerdict::Inconc => {
                return GraphvizColor::deeppink3; // 205 16 118
            },
            CoverageVerdict::LackObs => {
                return GraphvizColor::orangered3; // 205 55 0
            },
            CoverageVerdict::Dead => {
                return GraphvizColor::firebrick; // 178 34 34
            },
            CoverageVerdict::Out => {
                return GraphvizColor::red3; // 205 0 0
            },
            CoverageVerdict::UnCov => {
                return GraphvizColor::red3;
            }
        }
    }
}





