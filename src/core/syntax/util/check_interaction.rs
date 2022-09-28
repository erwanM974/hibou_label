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


use crate::core::syntax::interaction::*;
use crate::core::syntax::action::*;


pub struct InteractionCharacteristics {
    pub has_gates : bool,
    pub has_ands : bool,
    pub has_coregions : bool,
    pub has_loopP : bool,
    pub has_loopW : bool,
    pub has_loopS : bool
}

impl InteractionCharacteristics {

    pub fn new(has_gates : bool,
               has_ands : bool,
               has_coregions : bool,
               has_loopP : bool,
               has_loopW : bool,
               has_loopS : bool) -> InteractionCharacteristics {
        return InteractionCharacteristics{has_gates,has_ands,has_coregions,has_loopP,has_loopW,has_loopS};
    }

    pub fn new_empty() -> InteractionCharacteristics {
        return InteractionCharacteristics::new(false,
                                               false,
                                               false,
                                               false,
                                               false,
                                               false);
    }

    pub fn has_loop(&self) -> bool {
        return (self.has_loopS || self.has_loopW || self.has_loopP);
    }

    pub fn merge(&self, other : &InteractionCharacteristics) -> InteractionCharacteristics {
        return InteractionCharacteristics::new(
            self.has_gates || other.has_gates,
            self.has_ands || other.has_ands,
            self.has_coregions || other.has_coregions,
            self.has_loopP || other.has_loopP,
            self.has_loopW || other.has_loopW,
            self.has_loopS || other.has_loopS,
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
                        charac.has_loopS = true;
                    },
                    LoopKind::HHeadFirstWS => {
                        // ***
                    },
                    LoopKind::WWeakSeq => {
                        charac.has_loopW = true;
                    },
                    LoopKind::PInterleaving => {
                        charac.has_loopP = true;
                    }
                }
                return charac;
            },
            Interaction::CoReg(ref cr, ref i1, ref i2) => {
                let mut charac = i1.get_characteristics().merge( &i2.get_characteristics() );
                charac.has_coregions = true;
                return charac;
            },
            Interaction::And(ref i1, ref i2) => {
                let mut charac = i1.get_characteristics().merge( &i2.get_characteristics() );
                charac.has_ands = true;
                return charac;
            }
        }
    }

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
