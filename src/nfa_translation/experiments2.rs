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


use std::collections::{BTreeSet, HashSet};
use std::fmt::{format, Formatter, write};
use std::time::Instant;
use autour_core::dfa::dfa::AutDFA;
use autour_core::nfa::nfa::AutNFA;
use autour_core::traits::transform::AutTransformable;
use autour_core::traits::translate::AutTranslatable;
use itertools::max;
use rand::rngs::ThreadRng;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::syntax::metrics::{InteractionMetrics, SymbolKind};
use crate::experiments::doors_interactions_generation::generate_doors_interactions;
use crate::experiments::parstrict_interaction_generation::{generate_par_strict_interaction, NextActionSpec};
use crate::experiments::random_interaction_generation::{generate_random_interaction};
use crate::experiments::probas::InteractionSymbolsProbabilities;
use crate::io::output::draw_interactions::interface::{draw_interaction, InteractionGraphicalRepresentation};
use crate::nfa_translation::alphabet::get_alphabet_from_gen_ctx;
use crate::nfa_translation::canonize::canonize_interaction;
use crate::nfa_translation::compositional::get_nfa_from_interaction_via_composition;
use crate::nfa_translation::get_nfa_from_logger::get_nfa_from_interaction_exploration;
use crate::process::canon::param::default::DefaultCanonizationProcess;


pub struct NfaMetrics {
    pub median_time : u128,
    pub num_states : u32,
    pub num_edges : u32
}

impl NfaMetrics {
    pub fn new(median_time: u128, num_states: u32, num_edges: u32) -> Self {
        Self { median_time, num_states, num_edges }
    }
    pub fn add_csv_title_line(nfa_name : &str, results : &mut String) {
        results.push_str(&format!("{:}medtime,",nfa_name));
        results.push_str(&format!("{:}numstates,",nfa_name));
        results.push_str(&format!("{:}numedges,",nfa_name));
    }
    pub fn add_csv_line(&self, results : &mut String) {
        results.push_str(&self.median_time.to_string());
        results.push_str(",");
        results.push_str(&self.num_states.to_string());
        results.push_str(",");
        results.push_str(&self.num_edges.to_string());
        results.push_str(",");
    }
}


pub enum GeneratedInteractionKind {
    Random,
    ParStrict(u32),
    Doors
}

impl std::fmt::Display for GeneratedInteractionKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            GeneratedInteractionKind::Random => {
                write!(f, "Random")
            },
            GeneratedInteractionKind::ParStrict(ref numpar) => {
                write!(f, "Par{:}Strict", numpar)
            },
            GeneratedInteractionKind::Doors => {
                write!(f, "Doors")
            }
        }
    }
}

pub struct NfaGenerationExperiment2ResultMetrics {
    pub name : String,
    pub kind : GeneratedInteractionKind,
    pub interaction_metrics : InteractionMetrics,
    pub nfa_operational : NfaMetrics,
    pub nfa_compositional : NfaMetrics
}

impl NfaGenerationExperiment2ResultMetrics {
    pub fn new(name: String, kind : GeneratedInteractionKind,interaction_metrics: InteractionMetrics, nfa_operational: NfaMetrics, nfa_compositional: NfaMetrics) -> Self {
        Self { name, kind, interaction_metrics, nfa_operational, nfa_compositional }
    }

    pub fn add_csv_title_line(results : &mut String) {
        results.push_str("name,");
        results.push_str("kind,");
        InteractionMetrics::add_csv_title_line(results);
        NfaMetrics::add_csv_title_line("operat",results);
        NfaMetrics::add_csv_title_line("compo",results);
    }

    pub fn add_csv_line(&self, results : &mut String) {
        results.push_str(&self.name);
        results.push_str(",");
        results.push_str(&format!("{:},", self.kind));
        self.interaction_metrics.add_csv_line(results);
        self.nfa_operational.add_csv_line(results);
        self.nfa_compositional.add_csv_line(results);
    }
}


fn median(numbers: Vec<u128>) -> u128 {
    let mut numbers = numbers;
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}



pub fn get_nfa_metrics(gen_ctx : &GeneralContext,
                       i : &Interaction,
                       alphabet : &Vec<BTreeSet<TraceAction>>,
                       num_tries_for_median : u32,
                       stop_if_opnfa_more_than : Option<u32>) -> Option<(NfaMetrics,NfaMetrics)> {
    let mut nfa_operational = AutNFA::new_void_object(hashset!{0});
    let mut nfa_operational_times = vec![];
    for _ in 0..num_tries_for_median {
        let (nfa,duration) = get_nfa_from_interaction_exploration(&gen_ctx,
                                                                  &i,
                                                                  alphabet.clone());
        let in_micros = duration.as_micros();
        println!("via exploration translated interaction into nfa of {:} states in {:}μs", nfa.transitions.len(), in_micros);
        nfa_operational = nfa;
        nfa_operational_times.push(in_micros);
    }

    let opmetrics = NfaMetrics::new(
        median(nfa_operational_times),
        nfa_operational.transitions.len() as u32,
        nfa_operational.transitions.iter()
            .fold(0,|x,t|
                x + t.iter().fold(0,|y,(_,c)| y + (c.len() as u32))
            )
    );

    if let Some(opnfa_limit) = stop_if_opnfa_more_than {
        if opmetrics.num_states > opnfa_limit {
            println!("explo NFA has {:} states, more than {:} states, cancel costly computation of compo nfa",
                     opmetrics.num_states,
                     opnfa_limit);
            return None;
        }
    }

    let mut nfa_compositional = AutNFA::new_void_object(hashset!{0});
    let mut nfa_compositional_times = vec![];
    for _ in 0..num_tries_for_median {
        let (nfa,duration) = get_nfa_from_interaction_via_composition(&gen_ctx,
                                                                      &i,
                                                                      alphabet.clone());
        let in_micros = duration.as_micros();
        println!("via composition translated interaction into nfa of {:} states in {:}μs", nfa.transitions.len(), in_micros);
        nfa_compositional = nfa;
        nfa_compositional_times.push(in_micros);
    }

    let cmpmetrics = NfaMetrics::new(
        median(nfa_compositional_times),
        nfa_compositional.transitions.len() as u32,
        nfa_compositional.transitions.iter()
            .fold(0,|x,t|
                x + t.iter().fold(0,|y,(_,c)| y + (c.len() as u32))
            )
    );

    return Some((opmetrics,cmpmetrics));
}




pub struct ActParLimitation {}

impl ActParLimitation {

    pub fn is_limit_respected(p : u32, a : u32) -> bool {
        match p {
            8 => {
                a <= 45
            },
            9 => {
                //a <= 28
                false
            },
            10 => {
                //a <= 10
                false
            },
            x => {
                if x < 8 {
                    a <= 60
                } else {
                    false
                }
            }
        }
    }
}


pub fn run_nfa_generation_experiment2(number_of_interactions : u32,
                                     gen_ctx : &GeneralContext,
                                     num_tries_for_median : u32,
                                      gen_depth : u32,
                                      max_symbols : u32,
                                      max_par : u32) -> String {


    let mut csv_results = String::new();
    NfaGenerationExperiment2ResultMetrics::add_csv_title_line(&mut csv_results);
    csv_results.push_str("\n");

    // ***
    // parstrict interactions
    {
        let max_actions = 49;
        let mut context = GeneralContext::new();
        context.add_lf("l".to_string());
        for x in 0..max_actions {
            context.add_msg(format!("m{:}", x));
        }
        let alphabet = get_alphabet_from_gen_ctx(&context);
        for p in 0..=9 {
            for a in 1..max_actions {
                if ActParLimitation::is_limit_respected(p,a) {
                    println!("par{:} act{:} parstrict generation", p, a);
                    let mut nas = NextActionSpec::new(0,0);
                    let i = generate_par_strict_interaction(
                        &context,
                        &mut nas,
                        p,
                        a
                    );
                    let imetrics = InteractionMetrics::extract_from_interaction(&i);
                    let (opmetrics,cmpmetrics) = get_nfa_metrics(
                        gen_ctx,
                        &i,
                        &alphabet,
                        num_tries_for_median,
                        None
                    ).unwrap();
                    let metrics = NfaGenerationExperiment2ResultMetrics::new(
                        format!("par{:}act{:}",p,a),
                        GeneratedInteractionKind::ParStrict(p),
                        imetrics,
                        opmetrics,
                        cmpmetrics
                    );
                    metrics.add_csv_line(&mut csv_results);
                    csv_results.push_str("\n");
                }
            }
        }
    }


    // ***
    // random interactions
    {
        let alphabet = get_alphabet_from_gen_ctx(&gen_ctx);
        let mut memoized_ints = HashSet::new();

        let mut rng = rand::thread_rng();
        let mut x = 0;
        'myloop : while x < number_of_interactions {
            let i = generate_interaction(gen_ctx,&mut rng, gen_depth);
            if memoized_ints.contains(&i) {
                println!("already encounteterd interaction, retrying...");
                continue 'myloop;
            } else {
                memoized_ints.insert(i.clone());
            }
            if let Some(metrics) = get_metrics_from_random_interaction(
                    gen_ctx,
                    &alphabet,
                    &i,
                    max_symbols,
                    max_par,
                    num_tries_for_median,
                    x) {

                metrics.add_csv_line(&mut csv_results);
                csv_results.push_str("\n");
                x += 1;
            }
        }
    }

    // ***
    // doors interactions
    {
        let mut doors_gen_ctx = GeneralContext::new();
        doors_gen_ctx.add_msg("A".to_string());
        doors_gen_ctx.add_msg("B".to_string());
        doors_gen_ctx.add_msg("C".to_string());
        doors_gen_ctx.add_lf("l".to_string());
        let alphabet = get_alphabet_from_gen_ctx(&doors_gen_ctx);
        for num_doors in 1..=5 {
            for num_possible_letters in 2..=3 {
                for length_code in 1..=3 {
                    for length_after_code in 1..=3 {
                        println!(
                            "considering {:} doors with {:} possible letters, {:} code length and {:} after code length",
                            num_doors,
                            num_possible_letters,
                            length_code,
                            length_after_code
                        );
                        let ints = generate_doors_interactions(
                            num_doors,
                            num_possible_letters,
                            length_code,
                            length_after_code);
                        let num_ints = ints.len();
                        for (x,i) in ints.into_iter().enumerate() {
                            println!("door interaction {:} out of {:}", x+1, num_ints);
                            let imetrics = InteractionMetrics::extract_from_interaction(&i);
                            let opnfa_limit : u32 = if num_doors <= 4 {
                                5000
                            } else {
                                1000
                            };
                            if *imetrics.symbols.get(&SymbolKind::Action).unwrap() < 50 {
                                if let Some((opmetrics,cmpmetrics)) = get_nfa_metrics(
                                    &doors_gen_ctx,
                                    &i,
                                    &alphabet,
                                    num_tries_for_median,
                                    Some(5000)) {
                                    let metrics = NfaGenerationExperiment2ResultMetrics::new(
                                        format!("doors_{:}_{:}_{:}_{:}_{:}",num_doors,num_possible_letters,length_code,length_after_code,x),
                                        GeneratedInteractionKind::Doors,
                                        imetrics,
                                        opmetrics,
                                        cmpmetrics
                                    );
                                    metrics.add_csv_line(&mut csv_results);
                                    csv_results.push_str("\n");
                                }
                            } else {
                                println!("doors interaction has more than 50 actions, cancelling..");
                            }
                        }
                    }
                }
            }
        }
    }

    csv_results
}


fn random_respects_specifications(imetrics : &InteractionMetrics,
                                  max_symbols : u32,
                                  max_par : u32) -> bool {
    if *imetrics.symbols.get(&SymbolKind::LoopOther).unwrap() > 0 {
        panic!("has a non strict loop");
    }
    let s = imetrics.get_num_symbols();
    if s > max_symbols {
        println!("exceeded max num symbols");
        return false;
    }
    if s < 5 {
        println!("at least 5 symbols required");
        return false;
    }
    let p = *imetrics.symbols.get(&SymbolKind::Par).unwrap();
    let a = *imetrics.symbols.get(&SymbolKind::Action).unwrap();
    if p > max_par {
        println!("exceeded max par symbols");
        return false;
    }

    /*if !ActParLimitation::is_limit_respected(p,a) {
        println!("exceeded act/par limit");
        return false;
    }*/
    return true;
}


fn generate_interaction(gen_ctx : &GeneralContext,
                        rng : &mut ThreadRng,
                        gen_depth : u32) -> Interaction {

    let i = generate_random_interaction(&InteractionSymbolsProbabilities::default_high_level(),
                                        0,
                                        gen_depth,
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
    ican
}

fn get_metrics_from_random_interaction(gen_ctx : &GeneralContext,
                      alphabet : &Vec<BTreeSet<TraceAction>>,
                      i : &Interaction,
                      max_symbols : u32,
                      max_par : u32,
                      num_tries_for_median : u32,
                      x : u32) -> Option<NfaGenerationExperiment2ResultMetrics> {

    let imetrics = InteractionMetrics::extract_from_interaction(&i);
    if !random_respects_specifications(&imetrics,max_symbols,max_par) {
        return None;
    }

    draw_interaction(&gen_ctx,
                     &i,
                     &InteractionGraphicalRepresentation::AsSequenceDiagram,
                     &"temp".to_string(),
                     &"gen".to_string(),
                     &format!("nfagenexp_i{:}",x));


    match get_nfa_metrics(gen_ctx,&i,&alphabet,num_tries_for_median,Some(10000)) {
        None => {
            return None;
        },
        Some( (opmetrics,cmpmetrics) ) => {
            let rsults = NfaGenerationExperiment2ResultMetrics::new(
                format!("i{:}",x),
                GeneratedInteractionKind::Random,
                imetrics,
                opmetrics,
                cmpmetrics
            );
            return Some(rsults);
        }
    }


}