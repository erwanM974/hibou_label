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



use itertools::Itertools;
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};

use crate::core::general_context::GeneralContext;
use crate::output::commons::textual_representation::trace::trace_action::trace_actions_as_text;


pub fn multi_trace_as_text(gen_ctx : &GeneralContext,
                           co_localizations : &CoLocalizations,
                           multi_trace : &MultiTrace) -> String {
    let mut canals_strings : Vec<String> = vec![];
    for (canal_id,coloc_lfs) in co_localizations.locs_lf_ids.iter().enumerate() {
        let canal_trace : &Trace = multi_trace.get(canal_id).unwrap();
        // ***
        let mut canal_string = "".to_string();
        // ***
        let mut lf_names : Vec<String> = vec![];
        for lf_id in coloc_lfs.iter().sorted() {
            lf_names.push( gen_ctx.get_lf_name(*lf_id).unwrap() );
        }
        canal_string.push_str( &format!( "[{:}]", lf_names.join(",")) );
        // ***
        let mut trace_text_elements = vec![];
        for actions in canal_trace {
            trace_text_elements.push( trace_actions_as_text(gen_ctx, actions) );
        }
        canal_string.push_str(&trace_text_elements.join(".") );
        // ***
        canals_strings.push(canal_string);
    }
    // ***
    return format!( "{{\n{:}\n}}", canals_strings.join(";\n"));
}

