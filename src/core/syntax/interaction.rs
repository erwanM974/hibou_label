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

use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::core::syntax::action::*;
use crate::core::trace::{TraceAction, TraceActionKind};

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum LoopKind {
    PInterleaving  = 1,
    WWeakSeq       = 2,
    HHeadFirstWS   = 3,
    SStrictSeq     = 4
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Interaction {
    Empty,
    Emission(EmissionAction),
    Reception(ReceptionAction),
    Strict(Box<Interaction>,Box<Interaction>),
    Seq(Box<Interaction>,Box<Interaction>),
    CoReg(Vec<usize>,Box<Interaction>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Par(Box<Interaction>,Box<Interaction>),
    Loop(LoopKind,Box<Interaction>),
    And(Box<Interaction>,Box<Interaction>)
}


impl Interaction {

    pub fn get_leaf_action_kind(&self) -> TraceActionKind {
        match self {
            Interaction::Emission(_) => {
                return TraceActionKind::Emission;
            },
            Interaction::Reception(_) => {
                return TraceActionKind::Reception;
            },
            _ => {
                panic!("called as_leaf on something that's not a leaf : {:?}", self);
            }
        }
    }


    pub fn express_empty(&self) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            },
            &Interaction::Emission(_) => {
                return false;
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.recipients.len() == 0;
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            },
            &Interaction::Par(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return i1.express_empty() || i2.express_empty();
            },
            &Interaction::Loop(_, _) => {
                return true;
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn contained_model_actions(&self) -> (HashSet<&EmissionAction>,HashSet<&ReceptionAction>) {
        match &self {
            &Interaction::Empty => {
                return (hashset!{},hashset!{});
            },
            &Interaction::Emission(ref em_act) => {
                return (hashset!{em_act},hashset!{});
            },
            &Interaction::Reception(ref rc_act) => {
                return (hashset!{},hashset!{rc_act});
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let (mut em,mut rc) = i1.contained_model_actions();
                let (mut em2,mut rc2) = i2.contained_model_actions();
                em.extend(em2);
                rc.extend(rc2);
                return (em,rc);
            },
            &Interaction::Seq(ref i1, ref i2) => {
                let (mut em,mut rc) = i1.contained_model_actions();
                let (mut em2,mut rc2) = i2.contained_model_actions();
                em.extend(em2);
                rc.extend(rc2);
                return (em,rc);
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let (mut em,mut rc) = i1.contained_model_actions();
                let (mut em2,mut rc2) = i2.contained_model_actions();
                em.extend(em2);
                rc.extend(rc2);
                return (em,rc);
            },
            &Interaction::Par(ref i1, ref i2) => {
                let (mut em,mut rc) = i1.contained_model_actions();
                let (mut em2,mut rc2) = i2.contained_model_actions();
                em.extend(em2);
                rc.extend(rc2);
                return (em,rc);
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let (mut em,mut rc) = i1.contained_model_actions();
                let (mut em2,mut rc2) = i2.contained_model_actions();
                em.extend(em2);
                rc.extend(rc2);
                return (em,rc);
            },
            &Interaction::Loop(_, i1) => {
                return i1.contained_model_actions();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn contained_trace_actions(&self) -> HashSet<TraceAction> {
        match &self {
            &Interaction::Empty => {
                return HashSet::new();
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.get_all_atomic_actions();
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.get_all_atomic_actions();
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let mut content = i1.contained_trace_actions();
                content.extend( i2.contained_trace_actions() );
                return content;
            },
            &Interaction::Seq(ref i1, ref i2) => {
                let mut content = i1.contained_trace_actions();
                content.extend( i2.contained_trace_actions() );
                return content;
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut content = i1.contained_trace_actions();
                content.extend( i2.contained_trace_actions() );
                return content;
            },
            &Interaction::Par(ref i1, ref i2) => {
                let mut content = i1.contained_trace_actions();
                content.extend( i2.contained_trace_actions() );
                return content;
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let mut content = i1.contained_trace_actions();
                content.extend( i2.contained_trace_actions() );
                return content;
            },
            &Interaction::Loop(_, i1) => {
                return i1.contained_trace_actions();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn involved_lifelines(&self) -> HashSet<usize> {
        match &self {
            &Interaction::Empty => {
                return HashSet::new();
            },
            &Interaction::Emission(ref em_act) => {
                return em_act.lifeline_occupation();
            },
            &Interaction::Reception(ref rc_act) => {
                return rc_act.lifeline_occupation();
            },
            &Interaction::Strict(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Seq(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Par(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Alt(ref i1, ref i2) => {
                let mut content = i1.involved_lifelines();
                content.extend( i2.involved_lifelines() );
                return content;
            },
            &Interaction::Loop(_, i1) => {
                return i1.involved_lifelines();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn avoids_all_of(&self, lf_ids : &HashSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            },
            &Interaction::Emission(ref em_act) => {
                let occ = em_act.lifeline_occupation();
                if occ.is_disjoint(lf_ids) {
                    return true;
                } else {
                    return false;
                }
            },
            &Interaction::Reception(ref rc_act) => {
                let occ = rc_act.lifeline_occupation();
                if occ.is_disjoint(lf_ids) {
                    return true;
                } else {
                    return false;
                }
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::Par(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) && i2.avoids_all_of(lf_ids);
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return i1.avoids_all_of(lf_ids) || i2.avoids_all_of(lf_ids);
            },
            &Interaction::Loop(_, _) => {
                return true;
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn involves_any_of(&self, lf_ids : &HashSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                return false;
            },
            &Interaction::Emission(ref em_act) => {
                let occ = em_act.lifeline_occupation();
                if occ.is_disjoint(lf_ids) {
                    return false;
                } else {
                    return true;
                }
            },
            &Interaction::Reception(ref rc_act) => {
                let occ = rc_act.lifeline_occupation();
                if occ.is_disjoint(lf_ids) {
                    return false;
                } else {
                    return true;
                }
            },
            &Interaction::Strict(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Seq(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Par(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Alt(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            },
            &Interaction::Loop(_, ref i1) => {
                return i1.involves_any_of(lf_ids);
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn max_nested_loop_depth(&self) -> u32 {
        match self {
            &Interaction::Empty => {
                return 0;
            }, &Interaction::Emission(_) => {
                return 0;
            }, &Interaction::Reception(_) => {
                return 0;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            }, &Interaction::Loop(_, ref i1) => {
                return 1 + i1.max_nested_loop_depth();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }




    pub fn hide(&self, lfs_to_remove : &HashSet<usize>) -> Interaction {
        match self {
            Interaction::Empty => {
                return Interaction::Empty;
            },
            Interaction::Emission( ref em_act ) => {
                if lfs_to_remove.contains(&em_act.origin_lf_id) {
                    let mut has_lf_tars = false;
                    let mut target_lfs : Vec<usize> = Vec::new();
                    for tar_ref in &em_act.targets {
                        match tar_ref {
                            EmissionTargetRef::Lifeline( tar_lf_id ) => {
                                if !lfs_to_remove.contains( tar_lf_id ) {
                                    has_lf_tars = true;
                                    target_lfs.push(  *tar_lf_id );
                                }
                            },
                            EmissionTargetRef::Gate( tar_gt_id ) => {}
                        }
                    }
                    // ***
                    if has_lf_tars {
                        let hidden_act = ReceptionAction::new(None,em_act.ms_id,em_act.synchronicity.clone(),target_lfs);
                        return Interaction::Reception( hidden_act );
                    } else {
                        return Interaction::Empty;
                    }
                } else {
                    let mut targets : Vec<EmissionTargetRef> = Vec::new();
                    for tar_ref in &em_act.targets {
                        match tar_ref {
                            EmissionTargetRef::Lifeline( tar_lf_id ) => {
                                if !lfs_to_remove.contains( tar_lf_id ) {
                                    targets.push(  EmissionTargetRef::Lifeline( *tar_lf_id ) );
                                }
                            },
                            EmissionTargetRef::Gate( tar_gt_id ) => {
                                targets.push(  EmissionTargetRef::Gate( *tar_gt_id ) );
                            }
                        }
                    }
                    let hidden_act = EmissionAction::new(em_act.origin_lf_id,em_act.ms_id,em_act.synchronicity.clone(),targets);
                    return Interaction::Emission( hidden_act );
                }
            },
            Interaction::Reception( ref rc_act ) => {
                let mut has_lf_tars = false;
                let mut target_lfs : Vec<usize> = Vec::new();
                for tar_lf_id in &rc_act.recipients {
                    if !lfs_to_remove.contains( tar_lf_id ) {
                        has_lf_tars = true;
                        target_lfs.push(  *tar_lf_id );
                    }
                }
                // ***
                if has_lf_tars {
                    let hidden_act = ReceptionAction::new(rc_act.origin_gt_id.clone(),rc_act.ms_id,rc_act.synchronicity.clone(),target_lfs);
                    return Interaction::Reception( hidden_act );
                } else {
                    return Interaction::Empty;
                }
            },
            Interaction::Seq(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                return Interaction::Seq(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    }
                }
            },
            Interaction::CoReg(cr,i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                return Interaction::CoReg(cr.clone(), Box::new(i1hid), Box::new(i2hid) );
                            }
                        }
                    }
                }
            },
            Interaction::Strict(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                return Interaction::Strict(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    }
                }
            },
            Interaction::Alt(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        match &i2hid {
                            Interaction::Empty => {
                                return Interaction::Empty
                            },
                            _ => {
                                return Interaction::Alt(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    },
                    _ => {
                        return Interaction::Alt(Box::new(i1hid), Box::new(i2hid));
                    }
                }
            },
            Interaction::Par(i1,i2) => {
                let i1hid = i1.hide(lfs_to_remove);
                let i2hid = i2.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return i2hid;
                    },
                    _ => {
                        match &i2hid {
                            Interaction::Empty => {
                                return i1hid
                            },
                            _ => {
                                return Interaction::Par(Box::new(i1hid), Box::new(i2hid));
                            }
                        }
                    }
                }
            },
            Interaction::Loop(opkind,i1) => {
                let i1hid = i1.hide(lfs_to_remove);
                match &i1hid {
                    Interaction::Empty => {
                        return Interaction::Empty;
                    },
                    Interaction::Loop(opkind2,i11) => {
                        return Interaction::Loop((opkind.min(opkind2)).clone(), i11.clone());
                    },
                    _ => {
                        return Interaction::Loop(opkind.clone(),Box::new(i1hid) );
                    }
                }
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }




}


