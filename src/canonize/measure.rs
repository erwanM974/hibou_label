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

pub fn get_interaction_measure(interaction : &Interaction, gen_ctx : &GeneralContext) -> Vec<u32> {
    let mut measure = vec![int_num_action_nodes(interaction), int_num_loop_nodes(interaction)];
    // ***
    let base_num : u32 = gen_ctx.get_lf_num() as u32;
    // ***
    measure.extend(get_interaction_measure_rec(interaction,base_num) );
    return measure;
}

fn get_atomic_action_measure(lf_id : usize, kind : &TraceActionKind, ms_id : usize) -> Vec<u32> {
    match kind {
        &TraceActionKind::Emission => {
            return vec![lf_id as u32, 1, ms_id as u32];
        },
        &TraceActionKind::Reception => {
            return vec![lf_id as u32, 2, ms_id as u32];
        }
    }
}

fn get_action_measure(act : &ObservableAction, base_num : u32) -> Vec<u32> {
    match &act.act_kind {
        &ObservableActionKind::Reception => {
            return get_atomic_action_measure(act.lf_id, &TraceActionKind::Reception, act.ms_id);
        },
        &ObservableActionKind::Emission(ref targets) => {
            let tars_len = targets.len();
            if tars_len == 0 {
                return get_atomic_action_measure(act.lf_id, &TraceActionKind::Emission, act.ms_id);
            } else {
                let deployed_recs = deploy_receptions(act.ms_id, &mut targets.clone());
                let as_atomic = ObservableAction{lf_id:act.lf_id,act_kind:ObservableActionKind::Emission(Vec::new()),ms_id:act.ms_id};
                let new_int = Interaction::Strict( Box::new(Interaction::Action(as_atomic)), Box::new(deployed_recs));
                return get_interaction_measure_rec(&new_int, base_num);
            }
        }
    }
}

fn get_interaction_measure_rec(interaction : &Interaction, base_num : u32) -> Vec<u32> {
    match interaction {
        &Interaction::Empty => {
            return vec![base_num + 1];
        },
        &Interaction::Action(ref act) => {
            return get_action_measure(act, base_num);
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let mut measure : Vec<u32> = vec![base_num + 2];
            measure.extend( get_interaction_measure_rec(i1, base_num) );
            measure.extend( get_interaction_measure_rec(i2, base_num) );
            return measure;
        },
        &Interaction::Seq(ref i1, ref i2) => {
            let mut measure : Vec<u32> = vec![base_num + 3];
            measure.extend( get_interaction_measure_rec(i1, base_num) );
            measure.extend( get_interaction_measure_rec(i2, base_num) );
            return measure;
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut measure : Vec<u32> = vec![base_num + 4];
            measure.extend( get_interaction_measure_rec(i1, base_num) );
            measure.extend( get_interaction_measure_rec(i2, base_num) );
            return measure;
        },
        &Interaction::CoReg(_,ref i1, ref i2) => {
            let mut measure : Vec<u32> = vec![base_num + 5];
            measure.extend( get_interaction_measure_rec(i1, base_num) );
            measure.extend( get_interaction_measure_rec(i2, base_num) );
            return measure;
        },
        &Interaction::Loop(ref lk, ref i1) => {
            let mut measure : Vec<u32> = Vec::new();
            match lk {
                ScheduleOperatorKind::Strict => {
                    measure.push( base_num + 6 );
                },
                ScheduleOperatorKind::Seq => {
                    measure.push( base_num + 7 );
                },
                ScheduleOperatorKind::Par => {
                    measure.push( base_num + 8 );
                }
            }
            measure.extend( get_interaction_measure_rec(i1, base_num) );
            return measure;
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut measure : Vec<u32> = vec![base_num + 9];
            measure.extend( get_interaction_measure_rec(i1, base_num) );
            measure.extend( get_interaction_measure_rec(i2, base_num) );
            return measure;
        }
    }
}

// ==============================================
// ====== Numbers of actions & loop terms =======
// ==============================================


fn int_num_action_nodes(interaction : &Interaction) -> u32 {
    match interaction {
        &Interaction::Empty => {
            return 0;
        }, &Interaction::Action(ref act) => {
            match &act.act_kind {
                ObservableActionKind::Reception => {
                    return 1;
                },
                ObservableActionKind::Emission(ref targets) => {
                    return 1 + (targets.len() as u32);
                }
            }
        }, &Interaction::Strict(ref i1, ref i2) => {
            return int_num_action_nodes(i1) + int_num_action_nodes(i2);
        }, &Interaction::Seq(ref i1, ref i2) => {
            return int_num_action_nodes(i1) + int_num_action_nodes(i2);
        }, &Interaction::CoReg(_, ref i1, ref i2) => {
            return int_num_action_nodes(i1) + int_num_action_nodes(i2);
        }, &Interaction::Par(ref i1, ref i2) => {
            return int_num_action_nodes(i1) + int_num_action_nodes(i2);
        }, &Interaction::Alt(ref i1, ref i2) => {
            return int_num_action_nodes(i1) + int_num_action_nodes(i2);
        }, &Interaction::Loop(_,ref i1) => {
            return int_num_action_nodes(i1);
        }
    }
}

fn int_num_loop_nodes(interaction : &Interaction) -> u32 {
    match interaction {
        &Interaction::Empty => {
            return 0;
        }, &Interaction::Action(ref act) => {
            return 0;
        }, &Interaction::Strict(ref i1, ref i2) => {
            return int_num_loop_nodes(i1) + int_num_loop_nodes(i2);
        }, &Interaction::Seq(ref i1, ref i2) => {
            return int_num_loop_nodes(i1) + int_num_loop_nodes(i2);
        }, &Interaction::CoReg(_, ref i1, ref i2) => {
            return int_num_loop_nodes(i1) + int_num_loop_nodes(i2);
        }, &Interaction::Par(ref i1, ref i2) => {
            return int_num_loop_nodes(i1) + int_num_loop_nodes(i2);
        }, &Interaction::Alt(ref i1, ref i2) => {
            return int_num_loop_nodes(i1) + int_num_loop_nodes(i2);
        }, &Interaction::Loop(_,ref i1) => {
            return 1+ int_num_loop_nodes(i1);
        }
    }
}