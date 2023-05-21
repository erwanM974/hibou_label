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

use image::Rgb;
use image_colored_text::ttp::TextToPrint;
use itertools::Itertools;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};

use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::TraceAction;
use crate::io::output::draw_commons::hibou_color_palette::{HC_Grammar_Symbol, HC_Lifeline, HCP_Black, HCP_LightGray};
use crate::io::output::draw_traces::implem::trace_action::diagram_repr_trace_actions;
use crate::process::ana::node::flags::{MultiTraceAnalysisFlags, TraceAnalysisFlags};

fn extract_texts_on_canal_hidden(gen_ctx : &GeneralContext,
                                 lifelines : &HashSet<usize>,
                                 canal_flag : &TraceAnalysisFlags) -> Vec<Vec<TextToPrint>> {
    let mut canal_l1_text : Vec<TextToPrint> = Vec::new();
    add_lifelines_text_to_canal(gen_ctx,lifelines,&mut canal_l1_text,&mut 0, HCP_LightGray,HCP_LightGray);
    canal_l1_text.push( TextToPrint::new(" ".to_string(), Rgb(HCP_Black)) );
    // ***
    if canal_flag.simulated_before > 0 {
        canal_l1_text.push( TextToPrint::new(
            format!("♧{:}", canal_flag.simulated_before),
            Rgb(HCP_LightGray)) );
    }
    canal_l1_text.push( TextToPrint::new(
        format!("⚐{:}", canal_flag.consumed),
        Rgb(HCP_LightGray)) );
    canal_l1_text.push( TextToPrint::new(
        "⚑".to_string(),
        Rgb(HCP_LightGray)) );
    canal_l1_text.push( TextToPrint::new(
        " non-obs".to_string(),
        Rgb(HCP_LightGray)) );
    canal_l1_text.push( TextToPrint::new(
        " ".to_string(),
        Rgb(HCP_Black)) );
    return vec![ canal_l1_text, vec![] ];
}

fn extract_texts_on_canal_visible(gen_ctx : &GeneralContext,
                                 lifelines : &HashSet<usize>,
                                 canal_trace : &Trace,
                                  canal_flags : &TraceAnalysisFlags) -> Vec<Vec<TextToPrint>> {
    let mut canal_l1_text : Vec<TextToPrint> = Vec::new();
    let mut char_width_canal : usize = 1;
    {
        add_lifelines_text_to_canal(gen_ctx,lifelines,&mut canal_l1_text,&mut char_width_canal, HC_Lifeline,HC_Grammar_Symbol);
        // ***
        canal_l1_text.push( TextToPrint::new(
            " ←".to_string(),
            Rgb(HC_Grammar_Symbol)) );
        let rem_len = canal_trace.len() - canal_flags.consumed;
        if rem_len > 0 {
            let mut rem = (&canal_trace[canal_flags.consumed..canal_trace.len()]).iter();
            add_trace_text_to_canal(gen_ctx,&mut canal_l1_text,rem_len, &mut rem);
        } else {
            canal_l1_text.push( TextToPrint::new("ε".to_string(), Rgb(HCP_LightGray)) );
        }
        // ***
        canal_l1_text.push( TextToPrint::new(" ".to_string(), Rgb(HCP_Black)) );
    }
    // ***
    let mut canal_l2_text : Vec<TextToPrint> = Vec::new();
    {
        let blank_space : String = (0..char_width_canal).map(|_| " ").collect::<String>();
        canal_l2_text.push( TextToPrint::new(blank_space, Rgb(HCP_Black)) );
        // ***
        if canal_flags.simulated_before > 0 {
            canal_l2_text.push( TextToPrint::new(
                format!("♧{:}", canal_flags.simulated_before),
                Rgb(HC_Grammar_Symbol)) );
        }
        if (canal_flags.consumed > 0) || (canal_flags.simulated_after > 0) {
            canal_l2_text.push( TextToPrint::new(
                format!("⚐{:}", canal_flags.consumed),
                Rgb(HC_Grammar_Symbol)) );
            if canal_trace.len() == canal_flags.consumed {
                canal_l2_text.push( TextToPrint::new("⚑".to_string(), Rgb(HC_Grammar_Symbol)) );
                if canal_flags.simulated_after > 0 {
                    canal_l2_text.push( TextToPrint::new(
                        format!("{:}♣", canal_flags.simulated_after),
                        Rgb(HC_Grammar_Symbol)) );
                }
            }
        }
        canal_l2_text.push( TextToPrint::new(" ".to_string(), Rgb(HCP_Black)) );
    }
    // ***
    return vec![ canal_l1_text, canal_l2_text ];
}

pub fn extract_texts_on_multi_trace(gen_ctx : &GeneralContext,
                                    co_localizations : &CoLocalizations,
                                    multi_trace : &MultiTrace,
                                    flags : &MultiTraceAnalysisFlags,
                                    is_simulation : bool,
                                    sim_crit_loop : bool,
                                    sim_crit_act : bool) -> Vec<Vec<TextToPrint>> {
    let mut all_texts : Vec<Vec<TextToPrint>> = Vec::new();
    for (canal_id,lf_ids) in co_localizations.locs_lf_ids.iter().enumerate() {
        let canal_trace = multi_trace.get(canal_id).unwrap();
        let canal_flags : &TraceAnalysisFlags = flags.canals.get(canal_id).unwrap();
        let lifelines = co_localizations.get_lf_ids_from_coloc_ids(&hashset!{canal_id});
        // ***
        if canal_flags.no_longer_observed {
            all_texts.extend( extract_texts_on_canal_hidden(gen_ctx,&lifelines,canal_flags) );
        } else {
            all_texts.extend( extract_texts_on_canal_visible(gen_ctx,&lifelines,canal_trace, canal_flags) )
        }
    }
    if is_simulation {
        let mut simu_vec: Vec<TextToPrint> = vec![];
        // ***
        simu_vec.push( TextToPrint::new(" ⌕ ".to_string(), Rgb(HC_Grammar_Symbol)) );
        if (!sim_crit_loop) && (!sim_crit_act) {
            simu_vec.push( TextToPrint::new("*".to_string(), Rgb(HC_Grammar_Symbol)) );
        } else {
            if sim_crit_loop {
                simu_vec.push( TextToPrint::new(format!("L{:} ",flags.rem_loop_in_sim),
                    Rgb(HC_Grammar_Symbol)) );
            }
            if sim_crit_act {
                simu_vec.push( TextToPrint::new(format!("A{:}",flags.rem_act_in_sim),
                    Rgb(HC_Grammar_Symbol)) );
            }
        }
        // ***
        all_texts.push(simu_vec);
    }
    //
    return all_texts;
}

fn add_lifelines_text_to_canal(gen_ctx : &GeneralContext,
                                      lifelines : &HashSet<usize>,
                                      canal_text : &mut Vec<TextToPrint>,
                                      char_width_canal : &mut usize,
                               lf_color : [u8;3],
                               gram_color : [u8;3]) {
    canal_text.push( TextToPrint::new("[".to_string(), Rgb(gram_color)) );
    let mut remaining_len = lifelines.len();
    for lf_id in lifelines.iter().sorted() {
        let lf_name = gen_ctx.get_lf_name(*lf_id).unwrap();
        *char_width_canal = *char_width_canal + lf_name.len();
        canal_text.push( TextToPrint::new(lf_name, Rgb(lf_color)) );
        remaining_len = remaining_len -1;
        if remaining_len > 0 {
            canal_text.push( TextToPrint::new(",".to_string(), Rgb(gram_color)) );
            *char_width_canal = *char_width_canal + 1;
        }
    }
    canal_text.push( TextToPrint::new("]".to_string(), Rgb(gram_color)) );
    *char_width_canal = *char_width_canal + 2;
}


fn add_trace_text_to_canal<'a>(gen_ctx: &GeneralContext,
                               canal_text : &mut Vec<TextToPrint>,
                               init_len : usize,
                               rem_actions : &mut impl Iterator<Item = &'a BTreeSet<TraceAction>> ) {
    let mut rem_len = init_len;
    while let Some(actions) = rem_actions.next() {
        canal_text.append(&mut diagram_repr_trace_actions(actions,gen_ctx,false));
        rem_len -= 1;
        if rem_len > 0 {
            canal_text.push( TextToPrint::new(".".to_string(), Rgb(HC_Grammar_Symbol)) );
        }
    }
}

