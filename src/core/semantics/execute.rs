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
use std::collections::{HashSet,HashMap};



use crate::core::syntax::position::*;
use crate::core::syntax::action::*;
use crate::core::syntax::interaction::*;

use crate::core::semantics::frontier::*;
use crate::core::semantics::prune::*;


use crate::core::general_context::GeneralContext;
use crate::core::error::HibouCoreError;

use crate::core::trace::*;



pub fn deploy_receptions(ms_id : usize, rem_targets : &mut Vec<usize>) -> Interaction {
    let rem_tlen = rem_targets.len();
    if rem_tlen == 0 {
        return Interaction::Empty;
    } else if rem_tlen == 1 {
        let lf_id : usize = rem_targets.remove(0);
        return Interaction::Action( ObservableAction{lf_id,act_kind:ObservableActionKind::Reception,ms_id} );
    } else if rem_tlen == 2 {
        let lf_id_1 : usize = rem_targets.remove(0);
        let act_1 = ObservableAction{lf_id:lf_id_1,act_kind:ObservableActionKind::Reception,ms_id};
        let i1 = Interaction::Action( act_1 );
        let lf_id_2 : usize = rem_targets.remove(0);
        let act_2 = ObservableAction{lf_id:lf_id_2,act_kind:ObservableActionKind::Reception,ms_id};
        let i2 = Interaction::Action( act_2 );
        return Interaction::Seq( Box::new(i1), Box::new(i2) );
    } else {
        let lf_id_1: usize = rem_targets.remove(0);
        let act_1 = ObservableAction { lf_id: lf_id_1, act_kind: ObservableActionKind::Reception, ms_id };
        let i1 = Interaction::Action(act_1);
        return Interaction::Seq(Box::new(i1), Box::new(deploy_receptions(ms_id, rem_targets)));
    }
}


pub fn execute(my_int : Interaction, my_pos : Position, tar_lf_id : usize) -> Interaction {
    match my_pos {
        Position::Epsilon => {
            match my_int {
                Interaction::Action(model_action) => {
                    match model_action.act_kind {
                        ObservableActionKind::Reception => {
                            return Interaction::Empty;
                        },
                        ObservableActionKind::Emission(got_trg) => {
                            let mut targets = got_trg;
                            return deploy_receptions(model_action.ms_id,&mut targets);
                        }
                    }
                },
                _ => {
                    panic!();
                }
            }
        },
        Position::Left(p1) => {
            match my_int {
                Interaction::Alt(i1,_) => {
                    return execute( *i1,*p1, tar_lf_id);
                },
                Interaction::Loop(lkind, i1) => {
                    let old_i1 = i1.clone();
                    let new_i1 = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return Interaction::Loop(lkind, old_i1 );
                    } else {
                        match &lkind {
                            ScheduleOperatorKind::Seq => {
                                let orig_i = Interaction::Loop(lkind, old_i1 );
                                return Interaction::Seq( Box::new(new_i1), Box::new(orig_i) );
                            },
                            ScheduleOperatorKind::Strict => {
                                let orig_i = Interaction::Loop(lkind, old_i1 );
                                return Interaction::Strict( Box::new(new_i1), Box::new(orig_i) );
                            },
                            ScheduleOperatorKind::Par => {
                                let orig_i = Interaction::Loop(lkind, old_i1 );
                                return Interaction::Par( Box::new(new_i1), Box::new(orig_i) );
                            }
                        }
                    }
                },
                Interaction::Strict(i1,i2) => {
                    let new_i1 = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return *i2;
                    } else {
                        return Interaction::Strict( Box::new(new_i1), i2);
                    }
                },
                Interaction::Seq(i1,i2) => {
                    let new_i1 = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return *i2;
                    } else {
                        return Interaction::Seq( Box::new(new_i1), i2);
                    }
                },
                Interaction::CoReg(cr,i1,i2) => {
                    let new_i1 = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return *i2;
                    } else {
                        return Interaction::CoReg( cr,Box::new(new_i1), i2);
                    }
                },
                Interaction::Par(i1,i2) => {
                    let new_i1 = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return *i2;
                    } else {
                        return Interaction::Par( Box::new(new_i1), i2);
                    }
                },
                _ => {
                    panic!();
                }
            }
        },
        Position::Right(p2) => {
            match my_int {
                Interaction::Alt(_,i2) => {
                    return execute( *i2,*p2, tar_lf_id);
                },
                Interaction::Strict(_,i2) => {
                    return execute( *i2,*p2, tar_lf_id);
                },
                Interaction::Seq(i1,i2) => {
                    let new_i1 = prune(*i1, tar_lf_id);
                    if new_i1 == Interaction::Empty {
                        return execute(*i2, *p2, tar_lf_id);
                    } else {
                        let new_i2 = execute(*i2,*p2, tar_lf_id);
                        if new_i2 == Interaction::Empty {
                            return new_i1;
                        } else {
                            return Interaction::Seq( Box::new(new_i1), Box::new(new_i2));
                        }
                    }
                },
                Interaction::CoReg(cr,i1,i2) => {
                    let new_i1 : Interaction;
                    let new_i2 = execute(*i2,*p2, tar_lf_id);
                    if cr.contains(&tar_lf_id) {
                        new_i1 = *i1;
                    } else {
                        new_i1 = prune(*i1, tar_lf_id);
                    }
                    // ***
                    if new_i2 == Interaction::Empty {
                        return new_i1;
                    } else {
                        return Interaction::CoReg( cr,Box::new(new_i1), Box::new(new_i2));
                    }
                },
                Interaction::Par(i1,i2) => {
                    let new_i2 = execute(*i2,*p2, tar_lf_id);
                    if new_i2 == Interaction::Empty {
                        return *i1;
                    } else {
                        return Interaction::Par( i1, Box::new(new_i2));
                    }
                },
                _ => {
                    panic!();
                }
            }
        }
    }
}