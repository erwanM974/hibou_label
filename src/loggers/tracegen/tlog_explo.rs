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
use crate::core::general_context::GeneralContext;
use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::position::Position;
use crate::core::trace::TraceAction;
use crate::loggers::tracegen::conf::TracegenProcessLoggerGeneration;
use crate::loggers::tracegen::tracegen_logger::TraceGenProcessLogger;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::explo_proc::interface::logger::ExplorationLogger;

impl ExplorationLogger for TraceGenProcessLogger {

    fn log_init(&mut self, interaction: &Interaction, gen_ctx: &GeneralContext) {
        self.initiate();
    }

    fn log_term(&mut self, options_as_str: &Vec<String>) {
        // ***
    }

    fn log_explore(&mut self,
                   gen_ctx: &GeneralContext,
                   parent_state_id: u32,
                   new_state_id: u32,
                   action_position: &Position,
                   executed_actions: &HashSet<TraceAction>,
                   new_interaction: &Interaction) {
        // ***
        self.add_actions(parent_state_id,new_state_id,executed_actions);
        //
        let generate_trace_file : bool;
        match self.generation {
            TracegenProcessLoggerGeneration::exact => {
                if new_interaction.express_empty() {
                    generate_trace_file = true;
                } else {
                    generate_trace_file = false;
                }
            },
            TracegenProcessLoggerGeneration::prefixes => {
                generate_trace_file = true;
            },
            TracegenProcessLoggerGeneration::terminal => {
                generate_trace_file = false;
            }
        }
        // ***
        if generate_trace_file {
            self.generate_trace_file(gen_ctx,new_state_id);
        }
    }

    fn log_filtered(&mut self, parent_state_id: u32, new_state_id: u32, elim_kind: &FilterEliminationKind) {
        // ***
    }

    fn log_notified_lastchild_explored(&mut self,
                                       gen_ctx: &GeneralContext,
                                       parent_id: u32) {
        self.trace_map.remove(&parent_id);
    }

    fn log_notified_terminal_node_explored(&mut self,
                                       gen_ctx: &GeneralContext,
                                       parent_id: u32) {
        match self.generation {
            TracegenProcessLoggerGeneration::terminal => {
                self.generate_trace_file(gen_ctx,parent_id);
            },
            _ => {
                // nothing
            }
        }
    }
}