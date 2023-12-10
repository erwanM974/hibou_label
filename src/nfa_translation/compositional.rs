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


use std::collections::BTreeSet;
use std::time::{Duration, Instant};
use autour_core::nfa::nfa::AutNFA;
use autour_core::traits::access::AutAccessible;
use autour_core::traits::build::AutBuildable;
use autour_core::traits::transform::AutTransformable;

use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::nfa_translation::get_nfa_from_logger::get_nfa_from_interaction_exploration;
use crate::process::explo::loggers::nfait::printer::ActionNFAITPrinter;

pub fn interaction_has_only_strict_and_seq(int : &Interaction) -> bool {
    match int {
        Interaction::Emission(_) => true,
        Interaction::Reception(_) => true,
        Interaction::Empty => true,
        Interaction::Strict(i1,i2) => {
            interaction_has_only_strict_and_seq(i1) && interaction_has_only_strict_and_seq(i2)
        },
        Interaction::Seq(i1,i2) => {
            interaction_has_only_strict_and_seq(i1) && interaction_has_only_strict_and_seq(i2)
        },
        _ => false
    }
}


pub fn get_nfa_from_interaction_via_composition(gen_ctx : &GeneralContext,
                                            int : &Interaction,
                                                base_alphabet : Vec<BTreeSet<TraceAction>>) -> (AutNFA<usize>,Duration) {
    let now = Instant::now();
    let nfa = get_nfa_from_interaction_via_composition_rec(gen_ctx,int,base_alphabet);
    let elapsed_get_nfa = now.elapsed();
    (nfa,elapsed_get_nfa)
}

fn get_nfa_from_interaction_via_composition_rec(gen_ctx : &GeneralContext,
                                            int : &Interaction,
                                                alphabet : Vec<BTreeSet<TraceAction>>) -> AutNFA<usize> {
    if interaction_has_only_strict_and_seq(int) {
        let (nfa,_) = get_nfa_from_interaction_exploration(gen_ctx,int,alphabet);
        nfa
    } else {
        match int {
            Interaction::Alt(i1,i2) => {
                let nfa1 = get_nfa_from_interaction_via_composition_rec(
                    gen_ctx,
                    i1,
                    alphabet.clone());
                let nfa2 = get_nfa_from_interaction_via_composition_rec(gen_ctx,
                                                                        i2,
                                                                        alphabet.clone());
                nfa1.unite(nfa2).unwrap().trim()
            },
            Interaction::Par(i1,i2) => {
                let nfa1 = get_nfa_from_interaction_via_composition_rec(
                    gen_ctx,
                    i1,
                    alphabet.clone());
                let nfa2 = get_nfa_from_interaction_via_composition_rec(gen_ctx,
                                                                        i2,
                                                                        alphabet.clone());
                nfa1.interleave(nfa2).unwrap().trim()
            },
            Interaction::Seq(i1,i2) => {
                let nfa1 = get_nfa_from_interaction_via_composition_rec(
                    gen_ctx,
                    i1,
                    alphabet.clone());
                let nfa2 = get_nfa_from_interaction_via_composition_rec(gen_ctx,
                                                                        i2,
                                                                        alphabet.clone());
                nfa1.concatenate(nfa2).unwrap().trim()
            },
            Interaction::Strict(i1,i2) => {
                let nfa1 = get_nfa_from_interaction_via_composition_rec(
                    gen_ctx,
                    i1,
                    alphabet.clone());
                let nfa2 = get_nfa_from_interaction_via_composition_rec(gen_ctx,
                                                                        i2,
                                                                        alphabet.clone());
                nfa1.concatenate(nfa2).unwrap().trim()
            },
            Interaction::Loop(lk,i1) => {
                let nfa1 = get_nfa_from_interaction_via_composition_rec(
                    gen_ctx,
                    i1,
                    alphabet.clone());
                match lk {
                    LoopKind::SStrictSeq => {
                        nfa1.kleene().trim()
                    },
                    _ => {panic!()}
                }
            },
            _ => {panic!()}
        }
    }
}