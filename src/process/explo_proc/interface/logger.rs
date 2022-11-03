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
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::position::position::Position;
use crate::process::abstract_proc::common::FilterEliminationKind;


pub trait ExplorationLogger {

    fn log_init(&mut self,
                interaction : &Interaction,
                gen_ctx : &GeneralContext);

    fn log_term(&mut self,
                options_as_str : &Vec<String>);

    fn log_explore(&mut self,
                     gen_ctx : &GeneralContext,
                     parent_state_id : u32,
                     new_state_id : u32,
                     action_position : &Position,
                     executed_actions : &HashSet<TraceAction>,
                     new_interaction : &Interaction);

    fn log_filtered(&mut self,
                    parent_state_id : u32,
                    new_state_id : u32,
                    elim_kind : &FilterEliminationKind);

    fn log_notified_lastchild_explored(&mut self,
                                       gen_ctx: &GeneralContext,
                                       parent_id : u32);

    fn log_notified_terminal_node_explored(&mut self,
                                       gen_ctx: &GeneralContext,
                                       parent_id : u32);

}


