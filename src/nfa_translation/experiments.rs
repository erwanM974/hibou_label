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


use std::time::Instant;
use autour_core::dfa::dfa::AutDFA;
use autour_core::nfa::nfa::AutNFA;
use autour_core::traits::transform::AutTransformable;
use autour_core::traits::translate::AutTranslatable;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::nfa_translation::alphabet::get_alphabet_from_gen_ctx;
use crate::nfa_translation::compositional::get_nfa_from_interaction_via_composition;
use crate::nfa_translation::get_nfa_from_logger::get_nfa_from_interaction_exploration;

pub struct NfaGenerationExperimentResults {
    pub int : Interaction,

    pub nfa_operational : AutNFA<usize>,
    pub nfa_operational_median_time : u128,

    pub nfa_minimized_kw_from_opera : AutNFA<usize>,
    pub nfa_kw_med_time_from_opera : u128,

    pub dfa_minimized_from_opera : AutDFA<usize>,
    pub mindfa_med_time_from_opera : u128,

    pub nfa_compositional : AutNFA<usize>,
    pub nfa_compositional_median_time : u128,

    pub nfa_minimized_kw_from_compo : AutNFA<usize>,
    pub nfa_kw_med_time_from_compo : u128,

    pub dfa_minimized_from_compo : AutDFA<usize>,
    pub mindfa_med_time_from_compo : u128,
}

impl NfaGenerationExperimentResults {
    pub fn new(int: Interaction, nfa_operational: AutNFA<usize>, nfa_operational_median_time: u128, nfa_minimized_kw_from_opera: AutNFA<usize>, nfa_kw_med_time_from_opera: u128, dfa_minimized_from_opera: AutDFA<usize>, mindfa_med_time_from_opera: u128, nfa_compositional: AutNFA<usize>, nfa_compositional_median_time: u128, nfa_minimized_kw_from_compo: AutNFA<usize>, nfa_kw_med_time_from_compo: u128, dfa_minimized_from_compo: AutDFA<usize>, mindfa_med_time_from_compo: u128) -> Self {
        Self { int, nfa_operational, nfa_operational_median_time, nfa_minimized_kw_from_opera, nfa_kw_med_time_from_opera, dfa_minimized_from_opera, mindfa_med_time_from_opera, nfa_compositional, nfa_compositional_median_time, nfa_minimized_kw_from_compo, nfa_kw_med_time_from_compo, dfa_minimized_from_compo, mindfa_med_time_from_compo }
    }
}

fn median(numbers: Vec<u128>) -> u128 {
    let mut numbers = numbers;
    numbers.sort();
    let mid = numbers.len() / 2;
    numbers[mid]
}

pub fn run_nfa_generation_experiment(int : Interaction,
                                     gen_ctx : GeneralContext,
                                     num_tries_for_median : u32,
                                     state_lim : usize) -> NfaGenerationExperimentResults {

    let alphabet = get_alphabet_from_gen_ctx(&gen_ctx);

    let mut nfa_operational = AutNFA::new_void_object(hashset!{0});
    let mut nfa_operational_times = vec![];
    for _ in 0..num_tries_for_median {
        let (nfa,duration) = get_nfa_from_interaction_exploration(&gen_ctx,
                                                                  &int,
                                                                  alphabet.clone());
        let in_micros = duration.as_micros();
        println!("via exploration translated interaction into nfa of {:} states in {:}μs", nfa.transitions.len(), in_micros);
        nfa_operational = nfa;
        nfa_operational_times.push(in_micros);
    }

    let mut nfa_kw_opera = AutNFA::new_void_object(hashset!{0});
    let mut nfa_kw_opera_times = vec![];
    if nfa_operational.transitions.len() < state_lim {
        for _ in 0..num_tries_for_median {
            let now = Instant::now();
            nfa_kw_opera = nfa_operational.clone().minimize();
            let elapsed = now.elapsed();
            let in_micros = elapsed.as_micros();
            println!("minimized it into nfa of {:} states in {:}μs", nfa_kw_opera.transitions.len(), in_micros);
            nfa_kw_opera_times.push(in_micros);
        }
    } else {
        println!("too big to be minimized in reasonable time using kameda weiner");
        nfa_kw_opera_times.push(0);
    }

    let mut dfa_opera = AutDFA::new_void_object(hashset!{0});
    let mut mindfa_opera_times = vec![];
    for _ in 0..num_tries_for_median {
        let now = Instant::now();
        dfa_opera = nfa_operational.to_dfa().minimize();
        let elapsed = now.elapsed();
        let in_micros = elapsed.as_micros();
        println!("minimized it into dfa of {:} states in {:}μs", dfa_opera.transitions.len(), in_micros);
        mindfa_opera_times.push(in_micros);
    }


    let mut nfa_compositional = AutNFA::new_void_object(hashset!{0});
    let mut nfa_compositional_times = vec![];
    for _ in 0..num_tries_for_median {
        let (nfa,duration) = get_nfa_from_interaction_via_composition(&gen_ctx,
                                                                  &int,
                                                                  alphabet.clone());
        let in_micros = duration.as_micros();
        println!("via composition translated interaction into nfa of {:} states in {:}μs", nfa.transitions.len(), in_micros);
        nfa_compositional = nfa;
        nfa_compositional_times.push(in_micros);
    }

    let mut nfa_kw_compo = AutNFA::new_void_object(hashset!{0});
    let mut nfa_kw_compo_times = vec![];
    if nfa_compositional.transitions.len() < state_lim {
        for _ in 0..num_tries_for_median {
            let now = Instant::now();
            nfa_kw_compo = nfa_compositional.clone().minimize();
            let elapsed = now.elapsed();
            let in_micros = elapsed.as_micros();
            println!("minimized it into nfa of {:} states in {:}μs", nfa_kw_compo.transitions.len(), in_micros);
            nfa_kw_compo_times.push(in_micros);
        }
    } else {
        println!("too big to be minimized in reasonable time using kameda weiner");
        nfa_kw_compo_times.push(0);
    }

    let mut dfa_compo = AutDFA::new_void_object(hashset!{0});
    let mut mindfa_compo_times = vec![];
    for _ in 0..num_tries_for_median {
        let now = Instant::now();
        dfa_compo = nfa_compositional.to_dfa().minimize();
        let elapsed = now.elapsed();
        let in_micros = elapsed.as_micros();
        println!("minimized it into dfa of {:} states in {:}μs", dfa_compo.transitions.len(), in_micros);
        mindfa_compo_times.push(in_micros);
    }

    NfaGenerationExperimentResults::new(int,
                                        nfa_operational,
                                        median(nfa_operational_times),
                                        nfa_kw_opera,
                                        median(nfa_kw_opera_times),
                                        dfa_opera,
                                        median(mindfa_opera_times),
                                        nfa_compositional,
                                        median(nfa_compositional_times),
                                        nfa_kw_compo,
                                        median(nfa_kw_compo_times),
                                        dfa_compo,
                                        median(mindfa_compo_times))
}
