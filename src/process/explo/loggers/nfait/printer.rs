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
use autour_core::traits::repr::AbstractLanguagePrinter;
use graph_process_manager_loggers::nfait::builder::NFAITProcessBuilder;
use graph_process_manager_loggers::nfait::logger::NFAITBuilderPrinter;


use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::io::output::to_hfiles::trace::trace_action::trace_actions_as_htf_encoding;

use crate::process::explo::conf::ExplorationConfig;
use crate::process::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::process::explo::node::ExplorationNodeKind;
use crate::process::explo::step::ExplorationStepKind;


pub struct ActionNFAITPrinter {
    pub index_to_action_map : Vec<BTreeSet<TraceAction>>,
    pub gen_ctx : GeneralContext
}

impl ActionNFAITPrinter {
    pub fn new(index_to_action_map: Vec<BTreeSet<TraceAction>>, gen_ctx: GeneralContext) -> Self {
        ActionNFAITPrinter { index_to_action_map, gen_ctx }
    }
}


const SYNTAX_EMPTY_CLEAR : &str = "∅";
const SYNTAX_EMPTY_HTML : &str = "&#8709;";

const SYNTAX_EPSILON_CLEAR : &str = "𝜀";
const SYNTAX_EPSILON_HTML : &str = "&#x3B5;";

const SYNTAX_WILDCARD_DOT : &str = ".";
const SYNTAX_WILDCARD_HASHTAG : &str = "#";

const SYNTAX_CONCATENATION_EMPTY : &str = "";
const SYNTAX_CONCATENATION_DOT : &str = ".";
const SYNTAX_ALTERNATION : &str = "|";

const SYNTAX_INTERSECTION_CLEAR : &str = "∩";
const SYNTAX_INTERSECTION_HTML : &str = "&cap;";

const SYNTAX_NEGATION_CLEAR : &str = "¬";
const SYNTAX_NEGATION_HTML : &str = "&not;";


impl AbstractLanguagePrinter<usize> for ActionNFAITPrinter {

    fn is_letter_string_repr_atomic(&self, letter: &usize) -> bool {
        false
    }

    fn get_letter_string_repr(&self, letter: &usize) -> String {
        trace_actions_as_htf_encoding(&self.gen_ctx, self.index_to_action_map.get(*letter).unwrap())
    }

    fn get_concatenation_separator(&self, use_html: bool) -> &'static str {
        SYNTAX_CONCATENATION_DOT
    }

    fn get_alternation_separator(&self, use_html: bool) -> &'static str {
        SYNTAX_ALTERNATION
    }

    fn get_intersection_separator(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_INTERSECTION_HTML
        } else {
            SYNTAX_INTERSECTION_CLEAR
        }
    }

    fn get_wildcard_symbol(&self, _use_html: bool) -> &'static str {
        SYNTAX_WILDCARD_DOT
    }

    fn get_negate_symbol(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_NEGATION_HTML
        } else {
            SYNTAX_NEGATION_CLEAR
        }
    }

    fn get_empty_symbol(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_EMPTY_HTML
        } else {
            SYNTAX_EMPTY_CLEAR
        }
    }

    fn get_epsilon_symbol(&self, use_html: bool) -> &'static str {
        if use_html {
            SYNTAX_EPSILON_HTML
        } else {
            SYNTAX_EPSILON_CLEAR
        }
    }
}

impl NFAITProcessBuilder<ExplorationConfig,usize> for ActionNFAITPrinter {

    fn step_into_letter(&mut self,
                        context: &ExplorationContext,
                        param: &ExplorationParameterization,
                        step: &ExplorationStepKind) -> Option<usize> {
        match step {
            ExplorationStepKind::Execute(frt_elt) => {
                if let Some(idx) = self.index_to_action_map.iter().position(|r| r == &frt_elt.target_actions) {
                    Some(idx)
                } else {
                    self.index_to_action_map.push(frt_elt.target_actions.clone());
                    Some( self.index_to_action_map.len() - 1 )
                }
            }
        }
    }

    fn is_node_final(&self,
                     context: &ExplorationContext,
                     param: &ExplorationParameterization,
                     node: &ExplorationNodeKind) -> bool {
        node.interaction.express_empty()
    }

}


impl NFAITBuilderPrinter<ExplorationConfig,usize> for ActionNFAITPrinter {}



