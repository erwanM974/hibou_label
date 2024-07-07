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
use rand::prelude::{SliceRandom, StdRng};
use rand::Rng;

use crate::core::general_context::GeneralContext;
use crate::core::language::involve::involves::InvolvesLifelines;
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, EmissionTargetRef, ReceptionAction};
use crate::core::language::syntax::interaction::{Interaction, LoopKind};
use crate::core::language::syntax::metrics::{InteractionMetrics, SymbolKind};
use crate::experiments::interaction_random_gen::probas::{InteractionGenerationSymbol, InteractionSymbolsProbabilities};
use crate::nfa_translation::alphabet::get_alphabet_from_gen_ctx;
use crate::nfa_translation::experiments2::NfaMetrics;
use crate::nfa_translation::get_nfa_from_logger::get_nfa_from_interaction_exploration;


pub fn generate_random_action(signature : &GeneralContext,rng : &mut StdRng) -> Interaction {
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

pub fn generate_random_pattern(signature : &GeneralContext,
                               is_broadcast : bool,
                               rng : &mut StdRng) -> Interaction {
    let ms_id = rng.gen_range(0..signature.get_ms_num());
    let mut lifelines : Vec<usize> = (0..signature.get_lf_num()).collect();
    lifelines.shuffle(rng);
    let orig_lf_id = lifelines.pop().unwrap();
    let targets : Vec<EmissionTargetRef> = if is_broadcast {
        let number_of_targets = rng.gen_range(0..lifelines.len());
        lifelines[0..number_of_targets].iter()
            .map(|x| EmissionTargetRef::Lifeline(*x)).collect()
    } else {
        vec![EmissionTargetRef::Lifeline(lifelines.pop().unwrap())]
    };
    let emission = EmissionAction::new(
        orig_lf_id,
        ms_id,
        CommunicationSynchronicity::Asynchronous,
        targets
    );
    Interaction::Emission(emission)
}

pub fn generate_random_interaction(probas : &InteractionSymbolsProbabilities,
                                   //resolve_basic_at_low_depth : bool,
                                   depth : u32,
                                   max_depth : u32,
                                   signature : &GeneralContext,
                                   rng : &mut StdRng) -> Interaction {
    if depth >= max_depth {
        return generate_random_action(signature,rng);
    }
    let mut symbol = probas.get_random_symbol(rng);
    match symbol {
        InteractionGenerationSymbol::Transmission => {
            generate_random_pattern(signature,false,rng)
        },
        InteractionGenerationSymbol::Broadcast => {
            generate_random_pattern(signature,true,rng)
        },
        InteractionGenerationSymbol::Basic => {
            generate_random_interaction(
                &InteractionSymbolsProbabilities::default_basic(),
                depth,
                max_depth,
                signature,
                rng
            )
        },
        InteractionGenerationSymbol::Empty => {
            Interaction::Empty
        },
        InteractionGenerationSymbol::Action => {
            generate_random_action(signature,rng)
        },
        InteractionGenerationSymbol::LoopS => {
            let i1 = generate_random_interaction(probas,depth+1,max_depth,signature,rng);
            Interaction::Loop(LoopKind::SStrictSeq,Box::new(i1))
        },
        InteractionGenerationSymbol::LoopW => {
            let i1 = generate_random_interaction(probas,depth+1,max_depth,signature,rng);
            Interaction::Loop(LoopKind::WWeakSeq,Box::new(i1))
        },
        InteractionGenerationSymbol::LoopP => {
            let i1 = generate_random_interaction(probas,depth+1,max_depth,signature,rng);
            Interaction::Loop(LoopKind::PInterleaving,Box::new(i1))
        },
        x => {
            let i1 = Box::new(generate_random_interaction(probas,depth+1,max_depth,signature,rng));
            let i2 = Box::new(generate_random_interaction(probas,depth+1,max_depth,signature,rng));
            match x {
                InteractionGenerationSymbol::Strict => {
                    Interaction::Strict(i1,i2)
                },
                InteractionGenerationSymbol::Seq => {
                    Interaction::Seq(i1,i2)
                },
                InteractionGenerationSymbol::Par => {
                    Interaction::Par(i1,i2)
                },
                InteractionGenerationSymbol::Alt => {
                    Interaction::Alt(i1,i2)
                },
                InteractionGenerationSymbol::Coreg => {
                    let involved_in_both : Vec<usize> = i1.involved_lifelines().intersection(&i2.involved_lifelines())
                        .into_iter().cloned().collect();
                    if involved_in_both.is_empty() {
                        Interaction::Seq(i1,i2)
                    } else {
                        let lf_id = involved_in_both.get(rng.gen_range(0..involved_in_both.len())).unwrap();
                        Interaction::CoReg(vec![*lf_id],i1,i2)
                    }
                },
                _ => {
                    panic!()
                }
            }
        }
    }

}

#[cfg(test)]
mod tests {
    use rand::SeedableRng;
    use super::*;

    #[test]
    fn test() {
        let mut gen_ctx = GeneralContext::new();
        gen_ctx.add_lf("l1".to_string());
        gen_ctx.add_lf("l2".to_string());
        gen_ctx.add_msg("m1".to_string());
        gen_ctx.add_msg("m2".to_string());

        let mut rng = StdRng::seed_from_u64(0);
        let probas = InteractionSymbolsProbabilities::default_regular();
        let int = generate_random_interaction(&probas,0,3,&gen_ctx, &mut rng);
        println!("{:?}", int);
    }

}


