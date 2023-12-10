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


use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use rand::prelude::{SliceRandom, ThreadRng};
use rand::Rng;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, ReceptionAction};
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::syntax::metrics::{InteractionMetrics, SymbolKind};
use crate::experiments::probas::{InteractionGenerationSymbol, InteractionSymbolsProbabilities};
use crate::nfa_translation::alphabet::get_alphabet_from_gen_ctx;
use crate::nfa_translation::experiments2::NfaMetrics;
use crate::nfa_translation::get_nfa_from_logger::get_nfa_from_interaction_exploration;


pub fn generate_random_action(signature : &GeneralContext,rng : &mut ThreadRng) -> Interaction {
    let ms_id = rng.gen_range(0..signature.get_ms_num());
    let mut lifelines : Vec<usize> = (0..signature.get_lf_num()).collect();
    lifelines.shuffle(rng);
    if rng.gen_bool(0.5) {
        let emission = EmissionAction::new(
            *lifelines.get(0).unwrap(),
            ms_id,
            CommunicationSynchronicity::Asynchronous,vec![]
        );
        Interaction::Emission(emission)
    } else {
        let reception = ReceptionAction::new(
            None,
            ms_id,
            CommunicationSynchronicity::Asynchronous,
            vec![*lifelines.get(0).unwrap()]
        );
        Interaction::Reception(reception)
    }
}

pub fn generate_random_interaction(probas : &InteractionSymbolsProbabilities,
                                   depth : u32,
                                   gen_depth : u32,
                                   signature : &GeneralContext,
                                   rng : &mut ThreadRng) -> Interaction {
    if depth >= gen_depth {
        return generate_random_action(signature,rng);
    }
    let mut symbol = probas.get_random_symbol(rng);
    if depth <= gen_depth/2 {
        while symbol == InteractionGenerationSymbol::Basic {
            println!("should not resolve basic interaction at low depth");
            symbol = probas.get_random_symbol(rng);
        }
    }
    match symbol {
        InteractionGenerationSymbol::Basic => {
            let alphabet = get_alphabet_from_gen_ctx(&signature);
            let mut i = Interaction::Empty;
            loop {
                let got_i = generate_random_interaction(
                    &InteractionSymbolsProbabilities::default_basic(),
                    depth,
                    gen_depth,
                    signature, rng
                );
                let (nfa, _) = get_nfa_from_interaction_exploration(signature,
                                                                    &i,
                                                                    alphabet.clone());
                if nfa.transitions.len() > 1000 {
                    println!("basic interaction has more than 1000 states, retrying...");
                    continue;
                } else {
                    i = got_i;
                    break;
                }
            };
            i
        }
        InteractionGenerationSymbol::Empty => {
            Interaction::Empty
        }
        InteractionGenerationSymbol::Action => {
            generate_random_action(signature,rng)
        }
        InteractionGenerationSymbol::LoopS => {
            let i1 = generate_random_interaction(probas,depth+1,gen_depth,signature,rng);
            Interaction::Loop(LoopKind::SStrictSeq,Box::new(i1))
        }
        x => {
            let i1 = Box::new(generate_random_interaction(probas,depth+1,gen_depth,signature,rng));
            let i2 = Box::new(generate_random_interaction(probas,depth+1,gen_depth,signature,rng));
            match x {
                InteractionGenerationSymbol::Strict => {
                    Interaction::Strict(i1,i2)
                }
                InteractionGenerationSymbol::Seq => {
                    Interaction::Seq(i1,i2)
                }
                InteractionGenerationSymbol::Par => {
                    Interaction::Par(i1,i2)
                }
                InteractionGenerationSymbol::Alt => {
                    Interaction::Alt(i1,i2)
                }
                _ => {
                    panic!()
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let mut gen_ctx = GeneralContext::new();
        gen_ctx.add_lf("l1".to_string());
        gen_ctx.add_lf("l2".to_string());
        gen_ctx.add_msg("m1".to_string());
        gen_ctx.add_msg("m2".to_string());

        let mut rng = rand::thread_rng();
        let probas = InteractionSymbolsProbabilities::default();
        let int = generate_random_interaction(&probas,0,3,&gen_ctx, &mut rng);
        println!("{:?}", int);
    }

}


