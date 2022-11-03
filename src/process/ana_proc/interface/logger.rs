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
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::MultiTrace;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::position::position::Position;
use crate::core::execution::trace::trace::TraceAction;
use crate::process::abstract_proc::common::FilterEliminationKind;
use crate::process::ana_proc::interface::step::SimulationStepKind;
use crate::process::ana_proc::logic::flags::MultiTraceAnalysisFlags;
use crate::process::ana_proc::logic::verdicts::CoverageVerdict;

pub trait AnalysisLogger {

    fn log_init(&mut self,
                gen_ctx : &GeneralContext,
                co_localizations : &CoLocalizations,
                multi_trace : &MultiTrace,
                interaction : &Interaction,
                flags : &MultiTraceAnalysisFlags,
                is_simulation : bool,
                sim_crit_loop : bool,
                sim_crit_act : bool);

    fn log_term(&mut self,
                options_as_str : &Vec<String>);

    fn log_execution(&mut self,
                     gen_ctx : &GeneralContext,
                     co_localizations : &CoLocalizations,
                     multi_trace : &MultiTrace,
                     parent_state_id : u32,
                     new_state_id : u32,
                     action_position : &Position,
                     executed_actions : &HashSet<TraceAction>,
                     consu_set : &HashSet<usize>,
                     sim_map : &HashMap<usize,SimulationStepKind>,
                     new_interaction : &Interaction,
                     new_flags : &MultiTraceAnalysisFlags,
                     is_simulation : bool,
                     sim_crit_loop : bool,
                     sim_crit_act : bool);

    fn log_hide(&mut self,
                gen_ctx : &GeneralContext,
                co_localizations : &CoLocalizations,
                multi_trace : &MultiTrace,
                parent_state_id : u32,
                new_state_id : u32,
                lfs_to_hide : &HashSet<usize>,
                hidden_interaction : &Interaction,
                new_flags : &MultiTraceAnalysisFlags);

    fn log_filtered(&mut self,
                    parent_state_id : u32,
                    new_state_id : u32,
                    elim_kind : &FilterEliminationKind);

    fn log_verdict(&mut self,
                   parent_state_id : u32,
                   verdict : &CoverageVerdict);

}
