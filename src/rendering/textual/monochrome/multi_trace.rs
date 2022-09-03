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
use itertools::Itertools;
use crate::core::general_context::GeneralContext;
use crate::core::trace::TraceAction;
use crate::process::ana_proc::multitrace::AnalysableMultiTrace;
use crate::rendering::textual::monochrome::short_action::trace_actions_as_text;



pub fn multi_trace_as_text(gen_ctx : &GeneralContext,
                           co_localizations : Option<&Vec<HashSet<usize>>>,
                           multitrace : &Vec<Vec<HashSet<TraceAction>>>) -> String {
    let mut canals_strings : Vec<String> = vec![];
    for (canal_id,canal_trace) in multitrace.iter().enumerate() {
        let mut canal_string = "".to_string();
        // ***
        match co_localizations {
            Some( colocs ) => {
                let mut lifelines = vec![];
                for lf_id in colocs.get(canal_id).unwrap().iter().sorted() {
                    lifelines.push( gen_ctx.get_lf_name(*lf_id).unwrap() );
                }
                canal_string.push_str( &format!( "[{:}]", lifelines.join(",")) );
            },
            None => {
                canal_string.push_str( "[#any]" );
            }
        }
        // ***
        let mut trace_text_elements = vec![];
        for actions in canal_trace {
            trace_text_elements.push( trace_actions_as_text(gen_ctx, actions) );
        }
        canal_string.push_str(&format!( " {:};\n", trace_text_elements.join(".")) );
        // ***
        canals_strings.push(canal_string);
    }
    // ***
    return format!( "{{\n{:}\n}}", canals_strings.join(";\n"));
}

