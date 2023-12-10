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

    pub fn reverse(&self) -> Interaction {
        match *self {
            Interaction::Empty => {
                Interaction::Empty
            },
            Interaction::Emission(ref em_act) => {
                Interaction::Emission(em_act.clone())
            },
            Interaction::Reception(ref rc_act) => {
                Interaction::Reception(rc_act.clone())
            },
            Interaction::Strict(ref i1, ref i2) => {
                Interaction::Strict(Box::new(i2.reverse()),Box::new(i1.reverse()))
            },
            Interaction::Seq(ref i1, ref i2) => {
                Interaction::Seq(Box::new(i2.reverse()),Box::new(i1.reverse()))
            },
            Interaction::CoReg(ref cr, ref i1, ref i2) => {
                Interaction::CoReg(cr.clone(),Box::new(i2.reverse()),Box::new(i1.reverse()))
            },
            Interaction::Par(ref i1, ref i2) => {
                Interaction::Par(Box::new(i2.reverse()),Box::new(i1.reverse()))
            },
            Interaction::Alt(ref i1, ref i2) => {
                Interaction::Alt(Box::new(i2.reverse()),Box::new(i1.reverse()))
            },
            Interaction::Loop(ref lk, ref i1) => {
                Interaction::Loop(lk.clone(), Box::new(i1.reverse()))
            },
            Interaction::Sync(ref s,ref i1, ref i2) => {
                Interaction::Sync(s.clone(),Box::new(i2.reverse()),Box::new(i1.reverse()))
            },
            _ => {
                panic!("non-conform interaction");
            }
        }
    }

    pub fn express_empty(&self) -> bool {
        match *self {
            Interaction::Empty => {
                true
            },
            Interaction::Emission(_) => {
                false
            },
            Interaction::Reception(ref rc_act) => {
                rc_act.recipients.len() == 0
            },
            Interaction::Strict(ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::Seq(ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::CoReg(_, ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::Par(ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
            },
            Interaction::Alt(ref i1, ref i2) => {
                i1.express_empty() || i2.express_empty()
            },
            Interaction::Loop(_, _) => {
                true
            },
            Interaction::Sync(_,ref i1, ref i2) => {
                i1.express_empty() && i2.express_empty()
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







}


