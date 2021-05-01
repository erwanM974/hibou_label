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

use std::collections::{HashSet,HashMap};
use std::collections::btree_map::BTreeMap;

use crate::core::syntax::interaction::Interaction;
use crate::core::general_context::GeneralContext;
use crate::core::trace::*;
use crate::process::anakind::AnalysisKind;
use crate::process::verdicts::GlobalVerdict;



pub static CSV_SEP : &'static str = ";";

pub struct AnalysisBenchResult {
    pub verdict : GlobalVerdict,
    pub num_nodes_goal_wp : u32
}

impl AnalysisBenchResult {
    pub fn new(verdict : GlobalVerdict,
               num_nodes_goal_wp : u32) -> AnalysisBenchResult {
        return AnalysisBenchResult{verdict,num_nodes_goal_wp};
    }

    pub fn to_csv_line(&self) -> String {
        let mut csv_line = String::new();
        csv_line.push_str( &self.verdict.to_string() );
        csv_line.push_str( CSV_SEP );;
        csv_line.push_str( &self.num_nodes_goal_wp.to_string() );
        return csv_line;
    }
}

pub struct HVSBenchResult {
    /*pub hsf_file : String,
    pub htf_file : String,
    pub initial_interaction : Interaction,
    pub initial_multi_trace : AnalysableMultiTrace,*/
    pub prefix_information : BTreeMap<usize,usize>,
    pub res_hide : AnalysisBenchResult,
    pub res_simu : AnalysisBenchResult
}

impl HVSBenchResult {
    pub fn new(/*hsf_file : String,
               htf_file : String,
               initial_interaction : Interaction,
               initial_multi_trace : AnalysableMultiTrace,*/
               prefix_information : BTreeMap<usize,usize>,
               res_hide : AnalysisBenchResult,
               res_simu : AnalysisBenchResult) -> HVSBenchResult {
        return HVSBenchResult{/*hsf_file,htf_file,initial_interaction,initial_multi_trace,*/prefix_information,res_hide,res_simu};
    }



}

pub fn get_lfs_names(gen_ctx : &GeneralContext, lfs_ids : &HashSet<usize>) -> String {
    let mut my_string = "[".to_string();
    let mut remaining : usize = lfs_ids.len();
    for lf_id in lfs_ids {
        remaining = remaining -1;
        my_string.push_str( &gen_ctx.get_lf_name(*lf_id).unwrap() );
        if remaining > 0 {
            my_string.push_str( "," );
        }
    }
    my_string.push_str( "]" );
    return my_string;
}

impl HVSBenchResult  {

    pub fn get_csv_header(gen_ctx:&GeneralContext, initial_multi_trace : &AnalysableMultiTrace) -> String {
        let mut csv_line = String::new();
        /*
        csv_line.push_str( "hsf_file(options_discarded)" );
        csv_line.push_str( CSV_SEP );
        csv_line.push_str( "htf_file" );
        csv_line.push_str( CSV_SEP );*/
        // ***
        csv_line.push_str( "total_trace_len" );
        csv_line.push_str( CSV_SEP );
        // ***
        for canal in &initial_multi_trace.canals {
            csv_line.push_str( &format!("intial_length_in_{}",&get_lfs_names(gen_ctx,&canal.lifelines)) );
            csv_line.push_str( CSV_SEP );
            csv_line.push_str( &format!("removed_in_{}",&get_lfs_names(gen_ctx,&canal.lifelines)) );
            csv_line.push_str( CSV_SEP );
            csv_line.push_str( &format!("new_length_in_{}",&get_lfs_names(gen_ctx,&canal.lifelines)) );
            csv_line.push_str( CSV_SEP );
        }
        // ***
        csv_line.push_str( "verdict_hide" );
        csv_line.push_str( CSV_SEP );
        csv_line.push_str( "dfs_analyze_size_hide" );
        // ***
        csv_line.push_str( CSV_SEP );
        csv_line.push_str( "verdict_simu" );
        csv_line.push_str( CSV_SEP );
        csv_line.push_str( "dfs_analyze_size_simu" );
        return csv_line;
    }

    pub fn to_csv_line(&self, gen_ctx:&GeneralContext, initial_multi_trace : &AnalysableMultiTrace) -> String {
        let mut csv_line = String::new();
        /*csv_line.push_str( &self.hsf_file );
        csv_line.push_str( CSV_SEP );
        csv_line.push_str( &self.htf_file );
        csv_line.push_str( CSV_SEP );*/
        // ***
        csv_line.push_str( &(initial_multi_trace.length().to_string()) );
        csv_line.push_str( CSV_SEP );
        // ***
        for canal_id in 0..(initial_multi_trace.canals.len()) {
            let init_canal : &MultiTraceCanal = initial_multi_trace.canals.get(canal_id).unwrap();
            let initial_component_length = init_canal.trace.len();
            // ***
            csv_line.push_str( &initial_component_length.to_string() );
            csv_line.push_str( CSV_SEP );
            // ***
            let num_removed_acts  = *self.prefix_information.get(&canal_id).unwrap();
            csv_line.push_str( &num_removed_acts.to_string() );
            csv_line.push_str( CSV_SEP );
            // ***
            let remaining_length = (initial_component_length - num_removed_acts);
            csv_line.push_str( &remaining_length.to_string() );
            csv_line.push_str( CSV_SEP );
        }
        // ***
        {
            csv_line.push_str( &self.res_hide.to_csv_line() );
            csv_line.push_str( CSV_SEP );
            csv_line.push_str( &self.res_simu.to_csv_line() );
        }
        return csv_line;
    }
}