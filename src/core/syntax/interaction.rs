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

use std::cmp::Ordering;


use std::cmp;
use std::collections::{HashMap,HashSet};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

use crate::core::syntax::position::*;
use crate::core::syntax::action::*;
use crate::core::trace::TraceAction;

#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum LoopKind {
    PInterleaving  = 1,
    SWeakSeq       = 2,
    HHeadFirstWS   = 3,
    XStrictSeq     = 4
}

#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub enum Interaction {
    Empty,
    Action(ObservableAction),
    Strict(Box<Interaction>,Box<Interaction>),
    Seq(Box<Interaction>,Box<Interaction>),
    CoReg(Vec<usize>,Box<Interaction>,Box<Interaction>),
    Alt(Box<Interaction>,Box<Interaction>),
    Par(Box<Interaction>,Box<Interaction>),
    Loop(LoopKind,Box<Interaction>)
}


impl Interaction {

    pub fn as_leaf(&self) -> &ObservableAction {
        match self {
            Interaction::Action(act) => {
                return act;
            },
            _ => {
                panic!("called as_leaf on something that's not a leaf : {:?}", self);
            }
        }
    }

    pub fn get_outermost_loop_content(&self, my_pos : &Position) -> Option<(Interaction,Position)> {
        match my_pos {
            Position::Epsilon => {
                return None;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i1).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Loop(_ , ref i1) => {
                        return Some( ( *(i1.clone()) , *(sub_pos.clone()) ) );
                    },
                    _ => {
                        panic!();
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i2).get_outermost_loop_content( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }

    pub fn get_sub_interaction(&self, my_pos : &Position) -> &Interaction {
        match my_pos {
            Position::Epsilon => {
                return self;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Loop(_ , ref i1) => {
                        return (&*i1).get_sub_interaction( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Seq(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Alt(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return (&*i2).get_sub_interaction( &(*sub_pos) );
                    },
                    _ => {
                        panic!();
                    }
                }
            }
        }
    }

    pub fn express_empty(&self) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            }, &Interaction::Action(_) => {
                return false;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.express_empty() || i2.express_empty();
            }, &Interaction::Loop(_, _) => {
                return true;
            }
        }
    }

    pub fn contained_actions(&self) -> HashSet<TraceAction> {
        match &self {
            &Interaction::Empty => {
                return HashSet::new();
            }, &Interaction::Action(ref act) => {
                return act.get_all_atomic_actions();
            }, &Interaction::Strict(ref i1, ref i2) => {
                let mut content = i1.contained_actions();
                content.extend( i2.contained_actions() );
                return content;
            }, &Interaction::Seq(ref i1, ref i2) => {
                let mut content = i1.contained_actions();
                content.extend( i2.contained_actions() );
                return content;
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                let mut content = i1.contained_actions();
                content.extend( i2.contained_actions() );
                return content;
            }, &Interaction::Par(ref i1, ref i2) => {
                let mut content = i1.contained_actions();
                content.extend( i2.contained_actions() );
                return content;
            }, &Interaction::Alt(ref i1, ref i2) => {
                let mut content = i1.contained_actions();
                content.extend( i2.contained_actions() );
                return content;
            }, &Interaction::Loop(_, i1) => {
                return i1.contained_actions();
            }
        }
    }

    pub fn avoids(&self, lf_id : usize) -> bool {
        match self {
            &Interaction::Empty => {
                return true;
            }, &Interaction::Action(ref act) => {
                if act.occupation_after().contains(&lf_id) {
                    return false;
                } else {
                    return true;
                }
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.avoids(lf_id) && i2.avoids(lf_id);
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.avoids(lf_id) || i2.avoids(lf_id);
            }, &Interaction::Loop(_, _) => {
                return true;
            }
        }
    }

    pub fn involves(&self, lf_id : usize) -> bool {
        match self {
            &Interaction::Empty => {
                return false;
            }, &Interaction::Action(ref act) => {
                if act.occupation_after().contains(&lf_id) {
                    return true;
                } else {
                    return false;
                }
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.involves(lf_id) || i2.involves(lf_id);
            }, &Interaction::Loop(_, ref i1) => {
                return i1.involves(lf_id);
            }
        }
    }

    pub fn involves_any_of(&self, lf_ids : &HashSet<usize>) -> bool {
        match self {
            &Interaction::Empty => {
                return false;
            }, &Interaction::Action(ref act) => {
                for lf_id in lf_ids {
                    if act.occupation_after().contains(lf_id) {
                        return true;
                    }
                }
                return false;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.involves_any_of(lf_ids) || i2.involves_any_of(lf_ids);
            }, &Interaction::Loop(_, ref i1) => {
                return i1.involves_any_of(lf_ids);
            }
        }
    }

    pub fn max_nested_loop_depth(&self) -> u32 {
        match self {
            &Interaction::Empty => {
                return 0;
            }, &Interaction::Action(ref act) => {
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
            }
        }
    }

    pub fn get_loop_depth_at_pos(&self, my_pos : &Position) -> u32 {
        match my_pos {
            Position::Epsilon => {
                return 0;
            },
            Position::Left(sub_pos) => {
                match self {
                    &Interaction::Alt(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Seq(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Loop(_, ref i1) => {
                        return 1 + i1.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    _ => {
                        panic!("undefined pos");
                    }
                }
            },
            Position::Right(sub_pos) => {
                match self {
                    &Interaction::Alt(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Strict(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Seq(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::CoReg(_, ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    &Interaction::Par(ref i1, ref i2) => {
                        return i2.get_loop_depth_at_pos(&(*sub_pos) );
                    },
                    _ => {
                        panic!("undefined pos");
                    }
                }
            }
        }
    }



    pub fn hide(&self, lfs_to_remove : &HashSet<usize>) -> Interaction {
        match self {
            Interaction::Empty => {
                return Interaction::Empty;
            },
            Interaction::Action( ref act ) => {
                if lfs_to_remove.contains(&act.lf_id) {
                    match &act.act_kind {
                        ObservableActionKind::Reception => {
                            return Interaction::Empty;
                        },
                        ObservableActionKind::Emission( targets ) => {
                            let mut receptions : Vec<ObservableAction> = Vec::new();
                            for target_lf_id in targets {
                                if !lfs_to_remove.contains(target_lf_id) {
                                    receptions.push( ObservableAction{lf_id:*target_lf_id,
                                        act_kind:ObservableActionKind::Reception,
                                        ms_id:act.ms_id} );
                                }
                            }
                            return fold_reception_actions_with_seq(&mut receptions);
                        }
                    }
                } else {
                    match &act.act_kind {
                        ObservableActionKind::Reception => {
                            return Interaction::Action(act.clone());
                        },
                        ObservableActionKind::Emission( targets ) => {
                            let mut new_targets : Vec<usize> = Vec::new();
                            for target_lf_id in targets {
                                if !lfs_to_remove.contains(target_lf_id) {
                                    new_targets.push( *target_lf_id );
                                }
                            }
                            let new_act = ObservableAction{lf_id:act.lf_id,
                                act_kind:ObservableActionKind::Emission(new_targets),
                                ms_id:act.ms_id};
                            return Interaction::Action(new_act);
                        }
                    }
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
                    _ => {
                        return Interaction::Loop(opkind.clone(),Box::new(i1hid) );
                    }
                }
            }
        }
    }




}


pub fn fold_reception_actions_with_seq(receptions : &mut Vec<ObservableAction>) -> Interaction {
    let recepnum = receptions.len();
    if recepnum == 2 {
        let recep1 = receptions.pop().unwrap();
        let recep2 = receptions.pop().unwrap();
        return Interaction::Seq( Box::new(Interaction::Action(recep1)), Box::new(Interaction::Action(recep2)) );
    } else if recepnum == 1 {
        let recep1 = receptions.pop().unwrap();
        return Interaction::Action(recep1);
    } else if recepnum == 0 {
        return Interaction::Empty
    } else {
        let recep1 = receptions.pop().unwrap();
        return Interaction::Seq( Box::new(Interaction::Action(recep1)), Box::new( fold_reception_actions_with_seq(receptions) ) );
    }
}
