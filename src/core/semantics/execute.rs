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
use crate::core::syntax::position::*;

use crate::core::trace::*;





fn execute_emission(em_act : &EmissionAction) -> Interaction {
    match em_act.synchronicity {
        CommunicationSynchronicity::Synchronous => {
            return Interaction::Empty;
        },
        CommunicationSynchronicity::Asynchronous => {
            let mut target_lf_ids : Vec<usize> = Vec::new();
            for tar_ref in &em_act.targets {
                match tar_ref {
                    EmissionTargetRef::Lifeline(tar_lf_id) => {
                        target_lf_ids.push(*tar_lf_id);
                    },
                    _ => {}
                }
            }
            if target_lf_ids.len() > 0 {
                let rc_act = ReceptionAction::new(None,em_act.ms_id,CommunicationSynchronicity::Asynchronous,target_lf_ids);
                return Interaction::Reception(rc_act);
            } else {
                return Interaction::Empty;
            }
        }
    }
}

fn execute_reception(rc_act : &ReceptionAction, sub_pos : &Option<usize>) -> Interaction {
    match rc_act.synchronicity {
        CommunicationSynchronicity::Synchronous => {
            return Interaction::Empty;
        },
        CommunicationSynchronicity::Asynchronous => {
            match sub_pos {
                None => {
                    panic!("asynchronous reception execution must have a designated sub-position");
                },
                Some( sub_pos_idx) => {
                    let mut target_lf_ids = rc_act.recipients.clone();
                    target_lf_ids.remove(*sub_pos_idx);
                    if target_lf_ids.len() > 0 {
                        let new_rc_act = ReceptionAction::new(rc_act.origin_gt_id,rc_act.ms_id,CommunicationSynchronicity::Asynchronous,target_lf_ids);
                        return Interaction::Reception(new_rc_act);
                    } else {
                        return Interaction::Empty;
                    }
                }
            }
        }
    }
}

pub struct ExecutionResult {
    pub interaction : Interaction,
    pub affected_lifelines : HashSet<usize>
}

impl ExecutionResult {
    pub fn new(interaction : Interaction,affected_lifelines : HashSet<usize>) -> ExecutionResult {
        return ExecutionResult{interaction,affected_lifelines};
    }
}

fn execute_interaction_leaf(my_int : &Interaction,
                            sub_pos : &Option<usize>,
                            tar_lf_ids : &HashSet<usize>,
                            get_affected : bool) -> ExecutionResult {
    match my_int {
        Interaction::Emission(em_act) => {
            if get_affected {
                return ExecutionResult::new(execute_emission(em_act), tar_lf_ids.clone());
            } else {
                return ExecutionResult::new(execute_emission(em_act), hashset!{});
            }
        },
        Interaction::Reception(rc_act) => {
            if get_affected {
                return ExecutionResult::new(execute_reception(rc_act,sub_pos), tar_lf_ids.clone());
            } else {
                return ExecutionResult::new(execute_reception(rc_act,sub_pos), hashset!{});
            }
        },
        _ => {
            panic!();
        }
    }
}

fn make_follow_up_loop(old_i1 : &Interaction, new_i1 : Interaction, lkind : &LoopKind, tar_lf_ids : &HashSet<usize>) -> Interaction {
    if new_i1 == Interaction::Empty {
        return Interaction::Loop(lkind.clone(), Box::new(old_i1.clone() ) );
    } else {
        match &lkind {
            LoopKind::SStrictSeq => {
                let orig_i = Interaction::Loop(lkind.clone(), Box::new(old_i1.clone() ) );
                return Interaction::Strict( Box::new(new_i1), Box::new(orig_i) );
            },
            LoopKind::HHeadFirstWS => {
                let orig_i = Interaction::Loop(lkind.clone(), Box::new(old_i1.clone() ) );
                return Interaction::Seq( Box::new(new_i1), Box::new(orig_i) );
            },
            LoopKind::WWeakSeq => {
                // ***
                let orig_i = Interaction::Loop(lkind.clone(), Box::new(old_i1.clone() ) );
                let new_right_int_wsloop = Interaction::Seq( Box::new(new_i1), Box::new(orig_i) );
                // ***
                let pruned_loop = prune(&Interaction::Loop(LoopKind::WWeakSeq, Box::new(old_i1.clone() ) ),&tar_lf_ids);
                if pruned_loop == Interaction::Empty {
                    return new_right_int_wsloop;
                } else {
                    return Interaction::Seq( Box::new(pruned_loop), Box::new(new_right_int_wsloop) );
                }
            },
            LoopKind::PInterleaving => {
                let orig_i = Interaction::Loop(lkind.clone(), Box::new(old_i1.clone() ) );
                return Interaction::Par( Box::new(new_i1), Box::new(orig_i) );
            }
        }
    }
}


fn execute_interaction_left(my_int : &Interaction, sub_p1 : &Position, tar_lf_ids : &HashSet<usize>,get_affected : bool) -> ExecutionResult {
    match my_int {
        Interaction::Alt(i1, i2) => {
            if get_affected {
                let mut affected = i1.involved_lifelines();
                affected.extend( i2.involved_lifelines() );
                let exres1 = execute_interaction( i1,sub_p1, tar_lf_ids,false);
                return ExecutionResult::new(exres1.interaction,affected);
            } else {
                return execute_interaction( i1,sub_p1, tar_lf_ids,false);
            }
        },
        Interaction::Loop(lkind, i1) => {
            let exres1 = execute_interaction(i1,sub_p1,tar_lf_ids,false);
            let new_i1 = exres1.interaction;
            let affected : HashSet<usize>;
            if get_affected {
                affected = i1.involved_lifelines();
            } else {
                affected = hashset!{};
            }
            let new_i = make_follow_up_loop(i1,new_i1,lkind,tar_lf_ids);
            return ExecutionResult::new(new_i,affected);
        },
        Interaction::Strict(i1,i2) => {
            let exres1 = execute_interaction(i1,sub_p1,tar_lf_ids,get_affected);
            let new_i1 = exres1.interaction;
            let affected = exres1.affected_lifelines;
            // ***
            let new_i : Interaction;
            if new_i1 == Interaction::Empty {
                new_i = *i2.clone();
            } else {
                new_i = Interaction::Strict( Box::new(new_i1), i2.clone());
            }
            return ExecutionResult::new(new_i,affected);
        },
        Interaction::Seq(i1,i2) => {
            let exres1 = execute_interaction(i1,sub_p1,tar_lf_ids,get_affected);
            let new_i1 = exres1.interaction;
            let affected = exres1.affected_lifelines;
            // ***
            let new_i : Interaction;
            if new_i1 == Interaction::Empty {
                new_i = *i2.clone();
            } else {
                new_i = Interaction::Seq( Box::new(new_i1), i2.clone());
            }
            return ExecutionResult::new(new_i,affected);
        },
        Interaction::CoReg(cr,i1,i2) => {
            let exres1 = execute_interaction(i1,sub_p1,tar_lf_ids,get_affected);
            let new_i1 = exres1.interaction;
            let affected = exres1.affected_lifelines;
            // ***
            let new_i : Interaction;
            if new_i1 == Interaction::Empty {
                new_i = *i2.clone();
            } else {
                new_i = Interaction::CoReg(cr.clone(), Box::new(new_i1), i2.clone());
            }
            return ExecutionResult::new(new_i,affected);
        },
        Interaction::Par(i1,i2) => {
            let exres1 = execute_interaction(i1,sub_p1,tar_lf_ids,get_affected);
            let new_i1 = exres1.interaction;
            let affected = exres1.affected_lifelines;
            // ***
            let new_i : Interaction;
            if new_i1 == Interaction::Empty {
                new_i = *i2.clone();
            } else {
                new_i = Interaction::Par( Box::new(new_i1), i2.clone());
            }
            return ExecutionResult::new(new_i,affected);
        },
        _ => {
            panic!();
        }
    }
}


fn execute_interaction_right(my_int : &Interaction, sub_p2 : &Position, tar_lf_ids : &HashSet<usize>,get_affected : bool) -> ExecutionResult {
    match my_int {
        Interaction::Alt(i1,i2) => {
            if get_affected {
                let mut affected = i1.involved_lifelines();
                affected.extend( i2.involved_lifelines() );
                let exres2 = execute_interaction( i2,sub_p2, tar_lf_ids,false);
                return ExecutionResult::new(exres2.interaction,affected);
            } else {
                return execute_interaction( i2,sub_p2, tar_lf_ids,false);
            }
        },
        Interaction::Par(i1,i2) => {
            let exres2 = execute_interaction(i2,sub_p2, tar_lf_ids,get_affected);
            let new_i2 = exres2.interaction;
            if new_i2 == Interaction::Empty {
                return ExecutionResult::new(*i1.clone(),exres2.affected_lifelines);
            } else {
                return ExecutionResult::new(Interaction::Par( i1.clone(), Box::new(new_i2)),exres2.affected_lifelines);
            }
        },
        Interaction::Strict(i1,i2) => {
            if get_affected {
                let mut affected = i1.involved_lifelines();
                let exres2 = execute_interaction( i2,sub_p2, tar_lf_ids,true);
                affected.extend(exres2.affected_lifelines);
                return ExecutionResult::new(exres2.interaction,affected);
            } else {
                return execute_interaction( i2,sub_p2, tar_lf_ids,false);
            }
        },
        Interaction::Seq(i1,i2) => {
            let new_i1 : Interaction;
            let new_i2 : Interaction;
            let affected : HashSet<usize>;
            if get_affected {
                let (got_i1,mut aff1) = prune_with_affected(i1,&tar_lf_ids);
                let exres2 = execute_interaction(i2,sub_p2,tar_lf_ids,true);
                new_i1 = got_i1;
                new_i2 = exres2.interaction;
                aff1.extend(exres2.affected_lifelines);
                affected = aff1;
            } else {
                new_i1 = prune(i1,&tar_lf_ids);;
                let exres2 = execute_interaction(i2,sub_p2,tar_lf_ids,false);
                new_i2 = exres2.interaction;
                affected = exres2.affected_lifelines;
            }
            // ***
            if new_i1 == Interaction::Empty {
                return ExecutionResult::new(new_i2,affected);
            } else {
                if new_i2 == Interaction::Empty {
                    return ExecutionResult::new(new_i1,affected);
                } else {
                    return ExecutionResult::new(Interaction::Seq( Box::new(new_i1), Box::new(new_i2)),affected);
                }
            }
        },
        Interaction::CoReg(cr,i1,i2) => {
            // ***
            let mut lfs_to_prune = tar_lf_ids.clone();
            for lf_id_in_coreg in cr {
                lfs_to_prune.remove(lf_id_in_coreg);
            }
            // ***
            let new_i1 : Interaction;
            let new_i2 : Interaction;
            let affected : HashSet<usize>;
            if get_affected {
                let got_i1;
                let mut aff1;
                if lfs_to_prune.len() > 0 {
                    let (Agot_i1,mut Aaff1) = prune_with_affected(i1,&tar_lf_ids);
                    got_i1 = Agot_i1;
                    aff1 = Aaff1;
                } else {
                    got_i1 = *i1.clone();
                    aff1 = hashset!{};
                }
                let exres2 = execute_interaction(i2,sub_p2,tar_lf_ids,true);
                new_i1 = got_i1;
                new_i2 = exres2.interaction;
                aff1.extend(exres2.affected_lifelines);
                affected = aff1;
            } else {
                if lfs_to_prune.len() > 0 {
                    new_i1 = prune(i1, &lfs_to_prune);
                } else {
                    new_i1 = *i1.clone();
                }
                let exres2 = execute_interaction(i2,sub_p2,tar_lf_ids,false);
                new_i2 = exres2.interaction;
                affected = exres2.affected_lifelines;
            }
            // ***
            if new_i1 == Interaction::Empty {
                return ExecutionResult::new(new_i2,affected);
            } else {
                if new_i2 == Interaction::Empty {
                    return ExecutionResult::new(new_i1,affected);
                } else {
                    return ExecutionResult::new(Interaction::CoReg( cr.clone(),Box::new(new_i1), Box::new(new_i2)),affected);
                }
            }
        },
        _ => {
            panic!("trying to execute right on {:?}", my_int);
        }
    }
}

pub fn execute_interaction(my_int : &Interaction,
               my_pos : &Position,
               tar_lf_ids : &HashSet<usize>,
                           get_affected : bool) -> ExecutionResult {
    match my_pos {
        Position::Epsilon(sub_pos) => {
            return execute_interaction_leaf(my_int,sub_pos,tar_lf_ids,get_affected);
        },
        Position::Left(p1) => {
            return execute_interaction_left(my_int,p1,tar_lf_ids,get_affected);
        },
        Position::Right(p2) => {
            return execute_interaction_right(my_int,p2,tar_lf_ids,get_affected);
        }
    }
}