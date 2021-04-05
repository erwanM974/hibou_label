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



use crate::core::syntax::interaction::{Interaction,ScheduleOperatorKind};
use crate::core::syntax::action::*;
use crate::core::syntax::position::Position;
use crate::core::general_context::GeneralContext;

use crate::core::trace::TraceActionKind;

use crate::core::semantics::execute::deploy_receptions;



fn action_lower_than(act1 : &ObservableAction, act2 : &ObservableAction) -> bool {
    if &act1.lf_id < &act2.lf_id {
        return true;
    } else {
        if &act1.lf_id > &act2.lf_id {
            return false;
        } else {
            match (&act1.act_kind,&act2.act_kind) {
                (&ObservableActionKind::Emission(ref targs),&ObservableActionKind::Reception) => {
                    assert!(targs.len() == 0); // interactions without syntactic sugar of emissions with targets
                    return true;
                },
                (&ObservableActionKind::Reception,&ObservableActionKind::Emission(ref targs)) => {
                    assert!(targs.len() == 0); // interactions without syntactic sugar of emissions with targets
                    return false;
                },
                (&ObservableActionKind::Reception,&ObservableActionKind::Reception) => {
                    if &act1.ms_id < &act2.ms_id {
                        return true;
                    } else {
                        return false;
                    }
                },
                (&ObservableActionKind::Emission(ref t1),&ObservableActionKind::Emission(ref t2)) => {
                    assert!(t1.len() == 0); // interactions without syntactic sugar of emissions with targets
                    assert!(t2.len() == 0);
                    if &act1.ms_id < &act2.ms_id {
                        return true;
                    } else {
                        return false;
                    }
                }
            }
        }
    }
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
        &Interaction::Loop(ref sk1, ref i11) => {
            match i2 {
                &Interaction::Loop(ref sk2, ref i21) => {
                    if sk1.lower_than(sk2) {
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