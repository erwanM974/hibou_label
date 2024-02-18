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
use rand::rngs::StdRng;

use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::metrics::InteractionMetrics;
use crate::experiments::interaction_random_gen::gen::generate_random_interaction;
use crate::experiments::interaction_random_gen::probas::InteractionSymbolsProbabilities;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::nfa_translation::canonize::canonize_interaction;
use crate::nfa_translation::experiments2::NfaGenerationExperiment2ResultMetrics;
use crate::process::canon::param::default::DefaultCanonizationProcess;



pub fn generate_canonical_random_interaction(gen_ctx : &GeneralContext,
                                             rng : &mut StdRng,
                                             max_depth : u32,
                                             min_symbols : u32,
                                             probas : &InteractionSymbolsProbabilities) -> Option<Interaction> {

    let i = generate_random_interaction(probas,
                                        0,
                                        max_depth,
                                        &gen_ctx,
                                        rng);

    let imetrics = InteractionMetrics::extract_from_interaction(&i);
    let isymbs = imetrics.symbols.iter().fold(0_u32,|x,(_,c)| x + c);
    println!("generated interaction of depth {:} with {:} symbols",
             imetrics.depth,
             isymbs
    );

    let ican = canonize_interaction(&gen_ctx,&i, DefaultCanonizationProcess::BasicWithToSeq);
    let icanmetrics = InteractionMetrics::extract_from_interaction(&ican);
    let icansymbs = icanmetrics.symbols.iter().fold(0_u32,|x,(_,c)| x + c);
    println!("canonized to interaction of depth {:} with {:} symbols",
             icanmetrics.depth,
             icansymbs
    );
    if icansymbs < min_symbols {
        println!("not enough symbols");
        return None;
    }
    if icansymbs > isymbs {
        println!("canonized has more symbols !!");
        draw_interaction(&gen_ctx,
                         &i,
                         &InteractionGraphicalRepresentation::AsSequenceDiagram,
                         &"temp".to_string(),
                         &"canerror".to_string(),
                         &"init".to_string());
        draw_interaction(&gen_ctx,
                         &ican,
                         &InteractionGraphicalRepresentation::AsSequenceDiagram,
                         &"temp".to_string(),
                         &"canerror".to_string(),
                         &"canned".to_string());
        panic!();
    }
    Some(ican)
}
