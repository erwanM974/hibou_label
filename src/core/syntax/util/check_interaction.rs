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


use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::action::*;

impl Interaction {

    pub fn has_gates(&self) -> bool {
        match self {
            Interaction::Empty => {
                return false;
            },
            Interaction::Emission(ref em_act) => {
                for tar_ref in &em_act.targets {
                    match tar_ref {
                        EmissionTargetRef::Gate(_) => {
                            return true;
                        },
                        _ => {}
                    }
                }
                return false;
            },
            Interaction::Reception(ref rc_act) => {
                match rc_act.origin_gt_id {
                    None => {
                        return false;
                    },
                    Some(_) => {
                        return true;
                    }
                }
            },
            Interaction::Strict(ref i1, ref i2) => {
                return i1.has_gates() && i2.has_gates();
            },
            Interaction::Seq(ref i1, ref i2) => {
                return i1.has_gates() && i2.has_gates();
            },
            Interaction::Par(ref i1, ref i2) => {
                return i1.has_gates() && i2.has_gates();
            },
            Interaction::Alt(ref i1, ref i2) => {
                return i1.has_gates() && i2.has_gates();
            },
            Interaction::Loop(ref sk, ref i1) => {
                return i1.has_gates();
            },
            Interaction::CoReg(ref cr, ref i1, ref i2) => {
                return i1.has_gates() && i2.has_gates();
            },
            Interaction::And(ref i1, ref i2) => {
                return i1.has_gates() && i2.has_gates();
            }
        }
    }

    pub fn has_ands(&self) -> bool {
        match self {
            Interaction::Empty => {
                return false;
            },
            Interaction::Emission(_) => {
                return false;
            },
            Interaction::Reception(_) => {
                return false;
            },
            Interaction::Strict(ref i1, ref i2) => {
                return i1.has_ands() && i2.has_ands();
            },
            Interaction::Seq(ref i1, ref i2) => {
                return i1.has_ands() && i2.has_ands();
            },
            Interaction::Par(ref i1, ref i2) => {
                return i1.has_ands() && i2.has_ands();
            },
            Interaction::Alt(ref i1, ref i2) => {
                return i1.has_ands() && i2.has_ands();
            },
            Interaction::Loop(ref sk, ref i1) => {
                return i1.has_ands();
            },
            Interaction::CoReg(ref cr, ref i1, ref i2) => {
                return i1.has_ands() && i2.has_ands();
            },
            Interaction::And(ref i1, ref i2) => {
                return true;
            }
        }
    }

    pub fn has_coregions(&self) -> bool {
        match self {
            Interaction::Empty => {
                return false;
            },
            Interaction::Emission(_) => {
                return false;
            },
            Interaction::Reception(_) => {
                return false;
            },
            Interaction::Strict(ref i1, ref i2) => {
                return i1.has_coregions() && i2.has_coregions();
            },
            Interaction::Seq(ref i1, ref i2) => {
                return i1.has_coregions() && i2.has_coregions();
            },
            Interaction::Par(ref i1, ref i2) => {
                return i1.has_coregions() && i2.has_coregions();
            },
            Interaction::Alt(ref i1, ref i2) => {
                return i1.has_coregions() && i2.has_coregions();
            },
            Interaction::Loop(ref sk, ref i1) => {
                return i1.has_coregions();
            },
            Interaction::CoReg(ref cr, ref i1, ref i2) => {
                return true;
            },
            Interaction::And(ref i1, ref i2) => {
                return i1.has_coregions() && i2.has_coregions();
            }
        }
    }
}
