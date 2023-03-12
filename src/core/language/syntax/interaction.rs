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
use std::hash::Hash;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::syntax::action::{EmissionAction, ReceptionAction};


#[derive(Clone, PartialEq, Debug, Eq, PartialOrd, Ord, Hash)]
pub enum LoopKind {
    PInterleaving  = 1,
    WWeakSeq       = 2,
    HHeadFirstWS   = 3,
    SStrictSeq     = 4
}

#[derive(Clone, PartialEq, Debug, Eq, Hash, PartialOrd)]
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
    And(Box<Interaction>,Box<Interaction>),
    Sync(Vec<TraceAction>,Box<Interaction>,Box<Interaction>)
}


impl Interaction {

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
            &Interaction::Sync(_,ref i1, ref i2) => {
                return i1.express_empty() && i2.express_empty();
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
            &Interaction::Sync(_, ref i1, ref i2) => {
                let (mut em,mut rc) = i1.contained_model_actions();
                let (mut em2,mut rc2) = i2.contained_model_actions();
                em.extend(em2);
                rc.extend(rc2);
                return (em,rc);
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
            }, &Interaction::Sync(_, ref i1, ref i2) => {
                return i1.max_nested_loop_depth().max(i2.max_nested_loop_depth());
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn total_loop_num(&self) -> u32 {
        match self {
            &Interaction::Empty => {
                return 0;
            }, &Interaction::Emission(_) => {
                return 0;
            }, &Interaction::Reception(_) => {
                return 0;
            }, &Interaction::Strict(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Seq(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::CoReg(_, ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Par(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Alt(ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            }, &Interaction::Loop(_, ref i1) => {
                return 1 + i1.total_loop_num();
            }, &Interaction::Sync(_, ref i1, ref i2) => {
                return i1.total_loop_num() + i2.total_loop_num();
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }






}


