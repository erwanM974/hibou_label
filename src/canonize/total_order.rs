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



use crate::core::syntax::interaction::{Interaction};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::general_context::GeneralContext;

use crate::core::trace::TraceActionKind;



fn emission_lower_than(em_act1 : &EmissionAction, em_act2 : &EmissionAction) -> bool {
    if &em_act1.ms_id < &em_act2.ms_id {
        return true;
    } else if &em_act1.ms_id > &em_act2.ms_id{
        return false;
    }
    // ***
    if &em_act1.origin_lf_id < &em_act2.origin_lf_id {
        return true;
    } else if &em_act1.origin_lf_id > &em_act2.origin_lf_id {
        return false;
    }
    // ***
    let max_tar_len = em_act1.targets.len().max(em_act2.targets.len());
    for i in 0..max_tar_len {
        match (em_act1.targets.get(i) , em_act2.targets.get(i) ) {
            ( Some( tar_ref1 ), Some(tar_ref2) ) => {
                if tar_ref1 < tar_ref2 {
                    return true;
                }
            },
            (None,Some(_)) => {
                return true;
            },
            (Some(_),None) => {},
            (None,None) => {}
        }
    }
    // ***
    return em_act1.synchronicity < em_act2.synchronicity;
}


fn reception_lower_than(rc_act1 : &ReceptionAction, rc_act2 : &ReceptionAction) -> bool {
    if &rc_act1.ms_id < &rc_act2.ms_id {
        return true;
    } else if &rc_act1.ms_id > &rc_act2.ms_id{
        return false;
    }
    // ***
    match (rc_act1.origin_gt_id,rc_act2.origin_gt_id) {
        (None,Some(_)) => {
            return true;
        },
        (Some(_),None) => {},
        (None,None) => {},
        (Some( gt_id1),Some(gt_id2)) => {
            if gt_id1 < gt_id2 {
                return true;
            }
        }
    }
    // ***
    let max_tar_len = rc_act1.recipients.len().max(rc_act2.recipients.len());
    for i in 0..max_tar_len {
        match (rc_act1.recipients.get(i) , rc_act2.recipients.get(i) ) {
            ( Some( tar_lf_id1 ), Some(tar_lf_id2) ) => {
                if tar_lf_id1 < tar_lf_id2 {
                    return true;
                }
            },
            (None,Some(_)) => {
                return true;
            },
            (Some(_),None) => {},
            (None,None) => {}
        }
    }
    // ***
    return rc_act1.synchronicity < rc_act2.synchronicity;
}


pub fn interaction_lower_than(i1 : &Interaction, i2 : &Interaction) -> bool {
    match i1 {
        &Interaction::Empty => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                _ => {
                    return true;
                }
            }
        },
        &Interaction::Emission(ref em_act1) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Emission(ref em_act2) => {
                    return emission_lower_than(em_act1,em_act2);
                },
                _ => {
                    return true;
                }
            }
        },
        &Interaction::Reception(ref rc_act1) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Emission(_) => {
                    return false;
                },
                &Interaction::Reception(ref rc_act2) => {
                    return reception_lower_than(rc_act1,rc_act2);
                },
                _ => {
                    return true;
                }
            }
        }
        &Interaction::Par(ref i11, ref i12) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Emission(_) => {
                    return false;
                },
                &Interaction::Reception(_) => {
                    return false;
                },
                &Interaction::Par(ref i21, ref i22) => {
                    if interaction_lower_than(i11,i21) {
                        return true;
                    } else {
                        if i11 != i21 {
                            return false;
                        } else {
                            return interaction_lower_than(i12,i22);
                        }
                    }
                },
                _ => {
                    return true;
                }
            }
        },
        &Interaction::Seq(ref i11, ref i12) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Emission(_) => {
                    return false;
                },
                &Interaction::Reception(_) => {
                    return false;
                },
                &Interaction::Par(_,_) => {
                    return false;
                },
                &Interaction::Seq(ref i21, ref i22) => {
                    if interaction_lower_than(i11,i21) {
                        return true;
                    } else {
                        if i11 != i21 {
                            return false;
                        } else {
                            return interaction_lower_than(i12,i22);
                        }
                    }
                },
                _ => {
                    return true;
                }
            }
        },
        &Interaction::Strict(ref i11, ref i12) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Emission(_) => {
                    return false;
                },
                &Interaction::Reception(_) => {
                    return false;
                },
                &Interaction::Par(_,_) => {
                    return false;
                },
                &Interaction::Seq(_,_) => {
                    return false;
                },
                &Interaction::Strict(ref i21, ref i22) => {
                    if interaction_lower_than(i11,i21) {
                        return true;
                    } else {
                        if i11 != i21 {
                            return false;
                        } else {
                            return interaction_lower_than(i12,i22);
                        }
                    }
                },
                _ => {
                    return true;
                }
            }
        },
        &Interaction::Alt(ref i11, ref i12) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Emission(_) => {
                    return false;
                },
                &Interaction::Reception(_) => {
                    return false;
                },
                &Interaction::Par(_,_) => {
                    return false;
                },
                &Interaction::Seq(_,_) => {
                    return false;
                },
                &Interaction::Strict(_,_) => {
                    return false;
                },
                &Interaction::Alt(ref i21, ref i22) => {
                    if interaction_lower_than(i11,i21) {
                        return true;
                    } else {
                        if i11 != i21 {
                            return false;
                        } else {
                            return interaction_lower_than(i12,i22);
                        }
                    }
                },
                _ => {
                    return true;
                }
            }
        },
        &Interaction::Loop(ref lk1, ref i11) => {
            match i2 {
                &Interaction::Loop(ref lk2, ref i21) => {
                    if lk1 < lk2 {
                        return true;
                    } else {
                        return interaction_lower_than(i11,i21);
                    }
                },
                _ => {
                    return false;
                }
            }
        },
        _ => {
            panic!();
        }
    }
}