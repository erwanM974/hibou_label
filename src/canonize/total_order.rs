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

use crate::core::semantics::execute::deploy_receptions;



fn action_lower_than(act1 : &ObservableAction, act2 : &ObservableAction) -> bool {
    match (&act1.act_kind,&act2.act_kind) {
        (&ObservableActionKind::Emission(_),&ObservableActionKind::Reception(_)) => {
            return true;
        },
        (&ObservableActionKind::Reception(_),&ObservableActionKind::Emission(_)) => {
            return false;
        },
        (&ObservableActionKind::Reception(orig1),&ObservableActionKind::Reception(orig2)) => {
            match (orig1,orig2) {
                (None,None) => {
                    // nothing
                },
                (gt_id1,None) => {
                    return false;
                },
                (None,gt_id2) => {
                    return true;
                },
                (gt_id1,gt_id2) => {
                    if gt_id1 < gt_id2 {
                        return true;
                    } else if gt_id1 > gt_id2 {
                        return false;
                    }
                }
            }
            if &act1.ms_id < &act2.ms_id {
                return true;
            } else if &act1.ms_id > &act2.ms_id{
                return false;
            }
            if &act1.lf_id < &act2.lf_id {
                return true;
            } else if &act1.lf_id > &act2.lf_id {
                return false;
            }
        },
        (&ObservableActionKind::Emission(ref targets_refs1),&ObservableActionKind::Emission(ref targets_refs2)) => {
            if &act1.ms_id < &act2.ms_id {
                return true;
            } else if &act1.ms_id > &act2.ms_id{
                return false;
            }
            if &act1.lf_id < &act2.lf_id {
                return true;
            } else if &act1.lf_id > &act2.lf_id {
                return false;
            }
            let max_tar_len = targets_refs1.len().max(targets_refs2.len());
            for i in 0..max_tar_len {
                // todo complete
            }
        }
    }
    panic!("cannot compare actions");
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
        &Interaction::Action(ref a1) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Action(ref a2) => {
                    return action_lower_than(a1,a2);
                },
                _ => {
                    return true;
                }
            }
        },
        &Interaction::Par(ref i11, ref i12) => {
            match i2 {
                &Interaction::Empty => {
                    return false;
                },
                &Interaction::Action(ref a2) => {
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
                &Interaction::Action(ref a2) => {
                    return false;
                },
                &Interaction::Par(ref i21, ref i22) => {
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
                &Interaction::Action(ref a2) => {
                    return false;
                },
                &Interaction::Par(ref i21, ref i22) => {
                    return false;
                },
                &Interaction::Seq(ref i21, ref i22) => {
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
                &Interaction::Action(ref a2) => {
                    return false;
                },
                &Interaction::Par(ref i21, ref i22) => {
                    return false;
                },
                &Interaction::Seq(ref i21, ref i22) => {
                    return false;
                },
                &Interaction::Strict(ref i21, ref i22) => {
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