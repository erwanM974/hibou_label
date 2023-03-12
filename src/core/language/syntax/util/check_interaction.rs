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







use crate::core::language::syntax::action::EmissionTargetRef;
use crate::core::language::syntax::interaction::{Interaction, LoopKind};

pub struct InteractionCharacteristics {
    pub has_gates : bool,
    pub has_ands : bool,
    pub has_coregions : bool,
    pub has_loop_p : bool,
    pub has_loop_w : bool,
    pub has_loop_s : bool,
    pub has_sync : bool
}

impl InteractionCharacteristics {

    pub fn new(has_gates : bool,
               has_ands : bool,
               has_coregions : bool,
               has_loop_p : bool,
               has_loop_w : bool,
               has_loop_s : bool,
               has_sync : bool) -> InteractionCharacteristics {
        return InteractionCharacteristics{has_gates,has_ands,has_coregions,has_loop_p,has_loop_w,has_loop_s,has_sync};
    }

    pub fn new_empty() -> InteractionCharacteristics {
        return InteractionCharacteristics::new(false,
                                               false,
                                               false,
                                               false,
                                               false,
                                               false,
                                               false);
    }

    pub fn has_loop(&self) -> bool {
        return self.has_loop_s || self.has_loop_w || self.has_loop_p;
    }

    pub fn merge(&self, other : &InteractionCharacteristics) -> InteractionCharacteristics {
        return InteractionCharacteristics::new(
            self.has_gates || other.has_gates,
            self.has_ands || other.has_ands,
            self.has_coregions || other.has_coregions,
            self.has_loop_p || other.has_loop_p,
            self.has_loop_w || other.has_loop_w,
            self.has_loop_s || other.has_loop_s,
            self.has_sync || other.has_sync,
        )
    }
}


impl Interaction {

    pub fn get_characteristics(&self) -> InteractionCharacteristics {
        match self {
            Interaction::Empty => {
                return InteractionCharacteristics::new_empty();
            },
            Interaction::Emission(ref em_act) => {
                let mut has_gates : bool = false;
                for tar_ref in &em_act.targets {
                    match tar_ref {
                        EmissionTargetRef::Gate(_) => {
                            has_gates = true;
                        },
                        _ => {}
                    }
                }
                // ***
                let mut charac = InteractionCharacteristics::new_empty();
                charac.has_gates = has_gates;
                return charac;
            },
            Interaction::Reception(ref rc_act) => {
                let has_gates : bool;
                match rc_act.origin_gt_id {
                    None => {
                        has_gates = false;
                    },
                    Some(_) => {
                        has_gates = true;
                    }
                }
                // ***
                let mut charac = InteractionCharacteristics::new_empty();
                charac.has_gates = has_gates;
                return charac;
            },
            Interaction::Strict(ref i1, ref i2) => {
                return i1.get_characteristics().merge( &i2.get_characteristics() );
            },
            Interaction::Seq(ref i1, ref i2) => {
                return i1.get_characteristics().merge( &i2.get_characteristics() );
            },
            Interaction::Par(ref i1, ref i2) => {
                return i1.get_characteristics().merge( &i2.get_characteristics() );
            },
            Interaction::Alt(ref i1, ref i2) => {
                return i1.get_characteristics().merge( &i2.get_characteristics() );
            },
            Interaction::Loop(ref sk, ref i1) => {
                let mut charac = i1.get_characteristics();
                match sk {
                    LoopKind::SStrictSeq => {
                        charac.has_loop_s = true;
                    },
                    LoopKind::HHeadFirstWS => {
                        // ***
                    },
                    LoopKind::WWeakSeq => {
                        charac.has_loop_w = true;
                    },
                    LoopKind::PInterleaving => {
                        charac.has_loop_p = true;
                    }
                }
                return charac;
            },
            Interaction::CoReg(_, ref i1, ref i2) => {
                let mut charac = i1.get_characteristics().merge( &i2.get_characteristics() );
                charac.has_coregions = true;
                return charac;
            },
            Interaction::And(ref i1, ref i2) => {
                let mut charac = i1.get_characteristics().merge( &i2.get_characteristics() );
                charac.has_ands = true;
                return charac;
            },
            Interaction::Sync(_, ref i1, ref i2) => {
                let mut charac = i1.get_characteristics().merge( &i2.get_characteristics() );
                charac.has_sync = true;
                return charac;
            }
        }
    }

}
