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
use std::hash::Hash;


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
        return Interaction::Action( ObservableAction{lf_id,act_kind:ObservableActionKind::Reception(None),ms_id} );
    } else if rem_tlen == 2 {
        let lf_id_1 : usize = rem_targets.remove(0);
        let act_1 = ObservableAction{lf_id:lf_id_1,act_kind:ObservableActionKind::Reception(None),ms_id};
        let i1 = Interaction::Action( act_1 );
        let lf_id_2 : usize = rem_targets.remove(0);
        let act_2 = ObservableAction{lf_id:lf_id_2,act_kind:ObservableActionKind::Reception(None),ms_id};
        let i2 = Interaction::Action( act_2 );
        return Interaction::Seq( Box::new(i1), Box::new(i2) );
    } else {
        let lf_id_1: usize = rem_targets.remove(0);
        let act_1 = ObservableAction { lf_id: lf_id_1, act_kind: ObservableActionKind::Reception(None), ms_id };
        let i1 = Interaction::Action(act_1);
        return Interaction::Seq(Box::new(i1), Box::new(deploy_receptions(ms_id, rem_targets)));
    }
}


pub fn execute(my_int : Interaction,
               my_pos : Position,
               tar_lf_id : usize) -> (Interaction,HashSet<usize>) {
    match my_pos {
        Position::Epsilon => {
            match my_int {
                Interaction::Action(model_action) => {
                    match model_action.act_kind {
                        ObservableActionKind::Reception(_) => {
                            return (Interaction::Empty,HashSet::new());
                        },
                        ObservableActionKind::Emission(target_refs) => {
                            let mut targets : Vec<usize> = Vec::new();
                            for targ_ref in target_refs {
                                match targ_ref {
                                    EmissionTargetRef::Lifeline(tar_lf_id) => {
                                        targets.push(tar_lf_id);
                                    }, _ => {}
                                }
                            }
                            return (deploy_receptions(model_action.ms_id,&mut targets),HashSet::new());
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
                Interaction::Alt(i1, i2) => {
                    let mut affected = i1.involved_lifelines();
                    let (i1_prime,_) = execute( *i1,*p1, tar_lf_id);
                    affected.extend( i2.involved_lifelines() );
                    return (i1_prime,affected);
                },
                Interaction::Loop(lkind, i1) => {
                    let old_i1 = i1.clone();
                    let (new_i1,_) = execute( *i1,*p1, tar_lf_id);
                    let mut affected = new_i1.involved_lifelines();
                    affected.insert( tar_lf_id );
                    // ***
                    if new_i1 == Interaction::Empty {
                        return (Interaction::Loop(lkind, old_i1 ),affected);
                    } else {
                        match &lkind {
                            LoopKind::SStrictSeq => {
                                let orig_i = Interaction::Loop(lkind, old_i1 );
                                return (Interaction::Strict( Box::new(new_i1), Box::new(orig_i) ),affected);
                            },
                            LoopKind::HHeadFirstWS => {
                                let orig_i = Interaction::Loop(lkind, old_i1 );
                                return (Interaction::Seq( Box::new(new_i1), Box::new(orig_i) ),affected);
                            },
                            LoopKind::WWeakSeq => {
                                // ***
                                let orig_i = Interaction::Loop(lkind, old_i1.clone() );
                                let new_right_int_wsloop = Interaction::Seq( Box::new(new_i1), Box::new(orig_i) );
                                // ***
                                let pruned_loop = prune(Interaction::Loop(LoopKind::WWeakSeq, old_i1 ),tar_lf_id);
                                if pruned_loop == Interaction::Empty {
                                    return (new_right_int_wsloop,affected);
                                } else {
                                    return (Interaction::Seq( Box::new(pruned_loop), Box::new(new_right_int_wsloop) ),affected);
                                }
                            },
                            LoopKind::PInterleaving => {
                                let orig_i = Interaction::Loop(lkind, old_i1 );
                                return (Interaction::Par( Box::new(new_i1), Box::new(orig_i) ),affected);
                            }
                        }
                    }
                },
                Interaction::Strict(i1,i2) => {
                    let (new_i1,affected) = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return (*i2,affected);
                    } else {
                        return (Interaction::Strict( Box::new(new_i1), i2),affected);
                    }
                },
                Interaction::Seq(i1,i2) => {
                    let (new_i1,affected) = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return (*i2,affected);
                    } else {
                        return (Interaction::Seq( Box::new(new_i1), i2),affected);
                    }
                },
                Interaction::CoReg(cr,i1,i2) => {
                    let (new_i1,affected) = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return (*i2,affected);
                    } else {
                        return (Interaction::CoReg( cr,Box::new(new_i1), i2),affected);
                    }
                },
                Interaction::Par(i1,i2) => {
                    let (new_i1,affected) = execute( *i1,*p1, tar_lf_id);
                    // ***
                    if new_i1 == Interaction::Empty {
                        return (*i2,affected);
                    } else {
                        return (Interaction::Par( Box::new(new_i1), i2),affected);
                    }
                },
                _ => {
                    panic!();
                }
            }
        },
        Position::Right(p2) => {
            match my_int {
                Interaction::Alt(i1,i2) => {
                    let mut affected = i2.involved_lifelines();
                    let (i2_prime,_) = execute( *i2,*p2, tar_lf_id);
                    affected.extend( i1.involved_lifelines() );
                    return (i2_prime,affected);
                },
                Interaction::Strict(i1,i2) => {
                    let (i2_prime,mut affected) = execute( *i2,*p2, tar_lf_id);
                    affected.extend( i1.involved_lifelines() );
                    return (i2_prime,affected);
                },
                Interaction::Seq(i1,i2) => {
                    let (new_i2,mut affected) = execute(*i2,*p2, tar_lf_id);
                    affected.extend( i1.involved_lifelines() );
                    let new_i1 = prune(*i1, tar_lf_id);
                    //
                    if new_i1 == Interaction::Empty {
                        return (new_i2,affected);
                    } else {
                        if new_i2 == Interaction::Empty {
                            return (new_i1,affected);
                        } else {
                            return (Interaction::Seq( Box::new(new_i1), Box::new(new_i2)),affected);
                        }
                    }
                },
                Interaction::CoReg(cr,i1,i2) => {
                    let (new_i2,mut affected) = execute(*i2,*p2, tar_lf_id);
                    // ***
                    let new_i1 : Interaction;
                    if cr.contains(&tar_lf_id) {
                        new_i1 = *i1;
                    } else {
                        affected.extend( i1.involved_lifelines() );
                        new_i1 = prune(*i1, tar_lf_id);
                    }
                    // ***

                    // ***
                    if new_i1 == Interaction::Empty {
                        return (new_i2,affected);
                    } else {
                        if new_i2 == Interaction::Empty {
                            return (new_i1,affected);
                        } else {
                            return (Interaction::CoReg( cr,Box::new(new_i1), Box::new(new_i2)),affected);
                        }
                    }
                },
                Interaction::Par(i1,i2) => {
                    let (new_i2,affected) = execute(*i2,*p2, tar_lf_id);
                    if new_i2 == Interaction::Empty {
                        return (*i1,affected);
                    } else {
                        return (Interaction::Par( i1, Box::new(new_i2)),affected);
                    }
                },
                _ => {
                    panic!();
                }
            }
        }
    }
}