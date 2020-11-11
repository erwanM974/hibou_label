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


use std::collections::{HashMap,HashSet};
use std::collections::btree_map::BTreeMap;

use std::fs::write;
use std::path::Path;

use std::fs;
use std::fs::File;
use std::io::{Read,BufReader,BufRead,BufWriter,Write};


use crate::core::general_context::GeneralContext;

use crate::core::trace::*;
use crate::core::syntax::interaction::Interaction;



use crate::rendering::custom_draw::seqdiag::interaction::draw_interaction;


use crate::rendering::process::graphic_logger::GraphicProcessLogger;

use crate::process::log::*;
use crate::process::hibou_process::*;
use crate::process::priorities::ProcessPriorities;
use crate::process::semkind::SemanticKind;
use crate::process::verdicts::GlobalVerdict;

use crate::process::analysis::analyze;
use crate::process::exploration::explore;
use crate::from_text::hsf_file::{ProcessKind,parse_hsf_file};
use crate::from_text::htf_file::parse_htf_file;

use crate::benchmark::hide_vs_simu_1::result::*;

pub fn hvs1_bench_analyze(hsf_file : &str, htf_file : &str, report_file : &str, compute_search_space : bool) {
    // ***
    let mut bench_report_file = File::create(report_file ).unwrap();
    // ***
    match parse_hsf_file(&hsf_file,&ProcessKind::Analyze) {
        Err(e) => {
            println!("{:?}",e);
            panic!();
        },
        Ok( (gen_ctx,my_int,hoptions) ) => {
            // ***
            match parse_htf_file(&htf_file,&gen_ctx) {
                Err(e) => {
                    println!("{:?}",e);
                    panic!();
                },
                Ok( multi_trace ) => {
                    if !multi_trace.are_colocalizations_singletons() {
                        panic!("comparing hiding and simulation only makes sense on multi-traces where co-localizations are singletons")
                    }
                    // ***
                    bench_report_file.write( HVSBenchResult::get_csv_header(&gen_ctx,&multi_trace).as_bytes() );
                    bench_report_file.write( "\n".as_bytes() );
                    // ***
                    let mut canals_lens : Vec<(usize,usize)> = Vec::new();
                    for canal_id in 0..(multi_trace.canals.len()) {
                        let canal = multi_trace.canals.get(canal_id).unwrap();
                        canals_lens.push( (canal_id, canal.trace.len()) );
                    }
                    for to_remove in get_removal_distr(&mut canals_lens) {
                        bench_prefix(&gen_ctx,&my_int,&multi_trace,&to_remove,compute_search_space,&mut bench_report_file);
                    }
                }
            }
        }
    }

}

fn get_removal_distr( local_comp_sizes : &mut Vec<(usize,usize)> ) -> HashSet< BTreeMap<usize,usize> > {
    let mut distrs : HashSet< BTreeMap<usize,usize> > = HashSet::new();
    match local_comp_sizes.pop() {
        None => {
            distrs.insert( BTreeMap::new() );
        },
        Some( (canal_key,canal_len) ) => {
            for sub_distr in get_removal_distr(local_comp_sizes) {
                for x in 0..=canal_len {
                    let mut new_distr = sub_distr.clone();
                    new_distr.insert(canal_key.clone(), x);
                    distrs.insert( new_distr );
                }
            }
        }
    }
    return distrs;
}

fn bench_prefix(gen_ctx : &GeneralContext,
                interaction : &Interaction,
                multi_trace : &AnalysableMultiTrace,
                to_remove : &BTreeMap<usize,usize>,
                compute_search_space : bool,
                bench_report_file : &mut File) {
    // ***
    let mut new_canals : Vec<MultiTraceCanal> = Vec::new();
    for canal_id  in 0..(multi_trace.canals.len()) {
        let canal = multi_trace.canals.get(canal_id).unwrap();
        let to_remove_num = to_remove.get(&canal_id).unwrap();
        let new_trace = canal.trace[0..(canal.trace.len() - *to_remove_num)].to_vec();
        new_canals.push( MultiTraceCanal::new(canal.lifelines.clone(), new_trace,false,0,0,0));
    }
    let new_multi_trace = AnalysableMultiTrace::new(new_canals,interaction.max_nested_loop_depth());
    // ***
    let res_hide = bench_prefix_with_sem(gen_ctx,interaction,&new_multi_trace,compute_search_space,&SemanticKind::Hide);
    let res_simu = bench_prefix_with_sem(gen_ctx,interaction,&new_multi_trace,compute_search_space,&SemanticKind::Simulate(false));

    // ***
    let hvs_result = HVSBenchResult::new(to_remove.clone(),
                        res_hide,
                        res_simu);
    bench_report_file.write( hvs_result.to_csv_line(&gen_ctx,&multi_trace).as_bytes() );
    bench_report_file.write( "\n".as_bytes() );
}

fn bench_prefix_with_sem(gen_ctx : &GeneralContext,
                         interaction : &Interaction,
                         new_multi_trace : &AnalysableMultiTrace,
                         compute_search_space : bool,
                         sem_kind : &SemanticKind) -> AnalysisBenchResult {

    if compute_search_space {
        let (verdict_none_goal,node_count_none_goal) = analyze(interaction.clone(),
                                                               new_multi_trace.clone(),
                                                               gen_ctx.clone(),
                                                               Vec::new(),
                                                               HibouSearchStrategy::DFS,
                                                               ProcessPriorities::new(0,0,-1,None,-2,-2),
                                                               Vec::new(),
                                                               sem_kind.clone(),
                                                               None);
        // ***
        let (verdict_wp_goal,node_count_wp_goal) = analyze(interaction.clone(),
                                                           new_multi_trace.clone(),
                                                           gen_ctx.clone(),
                                                           Vec::new(),
                                                           HibouSearchStrategy::DFS,
                                                           ProcessPriorities::new(0,0,-1,None,-2,-2),
                                                           Vec::new(),
                                                           sem_kind.clone(),
                                                           Some(GlobalVerdict::WeakPass));
        // ***
        assert!( verdict_none_goal == verdict_wp_goal );
        assert!( node_count_none_goal >= node_count_wp_goal );
        // ***
        return AnalysisBenchResult::new(
                                           verdict_wp_goal,
                                           Some(node_count_none_goal),
                                           node_count_wp_goal);
    } else {
        let (verdict_wp_goal,node_count_wp_goal) = analyze(interaction.clone(),
                                                           new_multi_trace.clone(),
                                                           gen_ctx.clone(),
                                                           Vec::new(),
                                                           HibouSearchStrategy::DFS,
                                                           ProcessPriorities::new(0,0,-1,None,-2,-2),
                                                           Vec::new(),
                                                           sem_kind.clone(),
                                                           Some(GlobalVerdict::WeakPass));
        // ***
        return AnalysisBenchResult::new(
            verdict_wp_goal,
            None,
            node_count_wp_goal);
    }
}