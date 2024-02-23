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


use std::collections::{BTreeSet, HashMap, HashSet};
use std::iter::FromIterator;
use itertools::Itertools;

use crate::core::execution::semantics::execute::execute_interaction;

use crate::core::execution::semantics::frontier::{FrontierElement, global_frontier};
use crate::core::execution::trace::multitrace::Trace;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::eliminate_lf::eliminable::LifelineEliminable;
use crate::core::language::syntax::interaction::Interaction;
use crate::process::ana::context::AnalysisContext;
use crate::process::ana::node::flags::{MultiTraceAnalysisFlags, TraceAnalysisFlags};
use crate::process::ana::param::anakind::{AnalysisKind, SimulationActionCriterion, SimulationLoopCriterion};
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::step::{AnalysisStepKind, SimulationStepKind};


use crate::util::powerset::powerset;

impl AnalysisParameterization {

    pub fn is_ok_to_simulate(&self,
                             frt_elt : &FrontierElement,
                             interaction : &Interaction,
                             flags : &MultiTraceAnalysisFlags) -> bool {
        match &self.ana_kind {
            AnalysisKind::Simulate(ref config) => {
                let mut ok_to_simulate = true;
                match config.act_crit {
                    SimulationActionCriterion::None => {
                        // nothing
                    },
                    _ => {
                        if flags.rem_act_in_sim <= 0 {
                            ok_to_simulate = false;
                        }
                    }
                }
                match config.loop_crit {
                    SimulationLoopCriterion::None => {
                        // nothing
                    },
                    _ => {
                        let loop_depth = frt_elt.max_loop_depth;
                        if loop_depth > flags.rem_loop_in_sim {
                            ok_to_simulate = false;
                        }
                    }
                }
                return ok_to_simulate;
            },
            _ => {
                panic!();
            }
        }
    }

    pub fn get_simulation_matches_in_analysis(&self,
                                              context : &AnalysisContext,
                                              interaction : &Interaction,
                                              flags : &MultiTraceAnalysisFlags) -> Vec<AnalysisStepKind> {
        let mut next_steps = vec![];
        // ***
        for frt_elt in global_frontier(&interaction,true) {
            let canal_ids_of_targets = context.co_localizations.get_coloc_ids_from_lf_ids(&frt_elt.target_lf_ids);
            // ***
            let mut match_on_canal : Vec<usize> = vec!{}; // ids of the canals on which there is a match
            let mut ok_canals : HashSet<usize> = hashset!{}; // canals in which we already do something match or simu
            let mut act_left_to_match : HashSet<&TraceAction> = frt_elt.target_actions.iter().collect();
            for (canal_id, canal_flag) in flags.canals.iter().enumerate() {
                let canal_trace = context.multi_trace.get(canal_id).unwrap();
                match canal_trace.get(canal_flag.consumed) {
                    None => {},
                    Some( got_multiact ) => {
                        let mut intersect_with_front_elt = false;
                        let mut entirely_included_in_front_elt = true;
                        for got_act in got_multiact {
                            if act_left_to_match.contains(got_act) {
                                intersect_with_front_elt = true;
                            } else {
                                entirely_included_in_front_elt = false;
                            }
                        }
                        // ***
                        if intersect_with_front_elt && entirely_included_in_front_elt {
                            match_on_canal.push(canal_id );
                            ok_canals.insert(canal_id);
                            for got_act in got_multiact {
                                act_left_to_match.remove(got_act);
                            }
                        }
                    }
                }
            }
            // ***
            let mut to_simulate : HashMap<usize,SimulationStepKind> = hashmap!{}; // id of the canal on which the simulation step is done, which kind of simulation step
            let mut ok_to_simulate = true;
            if act_left_to_match.len() > 0 {
                ok_to_simulate = self.is_ok_to_simulate(&frt_elt,interaction,flags);
            }
            // ***
            for tract in act_left_to_match {
                if !ok_to_simulate {
                    break;
                }
                let tract_coloc_id = context.co_localizations.get_lf_coloc_id(tract.lf_id).unwrap();
                if ok_canals.contains(&tract_coloc_id) {
                    panic!("an action left to match on a coloc on which we already do some match-execution");
                } else {
                    let mut gotit = false;
                    let canal_flag : &TraceAnalysisFlags = flags.canals.get(tract_coloc_id).unwrap();
                    let canal_trace = context.multi_trace.get(tract_coloc_id).unwrap();
                    // *
                    if canal_flag.consumed == canal_trace.len() {
                        to_simulate.insert( tract_coloc_id, SimulationStepKind::AfterEnd);
                        gotit = true;
                        break;
                    } else {
                        if self.ana_kind.sim_before() && (canal_flag.consumed == 0) {
                            to_simulate.insert(tract_coloc_id,SimulationStepKind::BeforeStart);
                            gotit = true;
                            break;
                        }
                    }
                    // *
                    if !gotit {
                        ok_to_simulate = false;
                    }
                }
            }
            // ***
            if ok_to_simulate {
                {
                    let consu_set : HashSet<usize>;
                    {
                        let simu_set : HashSet<usize> = HashSet::from_iter(to_simulate.keys().cloned());
                        consu_set = HashSet::from_iter( canal_ids_of_targets.difference( &simu_set ).cloned() );
                    }
                    next_steps.push( AnalysisStepKind::Execute(frt_elt.clone(),
                                                               consu_set,
                                                               to_simulate.clone()) );
                }
                if match_on_canal.len() > 0 && self.is_ok_to_simulate(&frt_elt,interaction,flags) {
                    for combinations in powerset(&match_on_canal) {
                        if combinations.len() > 0 {
                            let mut ok_to_simulate = true;
                            let mut to_simulate_more = to_simulate.clone();
                            for canal_id in combinations {
                                if !ok_to_simulate{
                                    break;
                                }
                                // *
                                let canal_flag : &TraceAnalysisFlags = flags.canals.get(canal_id).unwrap();
                                let canal_trace = context.multi_trace.get(canal_id).unwrap();
                                // *
                                if canal_trace.len() == canal_flag.consumed {
                                    to_simulate_more.insert( canal_id, SimulationStepKind::AfterEnd);
                                } else {
                                    if self.ana_kind.sim_before() && (canal_flag.consumed == 0) {
                                        to_simulate_more.insert(canal_id,SimulationStepKind::BeforeStart);
                                    } else {
                                        ok_to_simulate = false;
                                    }
                                }
                                // *
                            }
                            if ok_to_simulate {
                                {
                                    let consu_set : HashSet<usize>;
                                    {
                                        let simu_set : HashSet<usize> = HashSet::from_iter(to_simulate_more.keys().cloned());
                                        consu_set = HashSet::from_iter( canal_ids_of_targets.difference( &simu_set ).cloned() );
                                    }
                                    next_steps.push( AnalysisStepKind::Execute(frt_elt.clone(),
                                                                               consu_set,
                                                                               to_simulate_more.clone()) );
                                }
                            }
                        }
                    }
                }
            }
        }
        next_steps
    }

    pub fn is_action_univocal_in_analysis(context : &AnalysisContext,
                                          interaction : &Interaction,
                                          coloc_id : usize,
                                          head_multi_action : &BTreeSet<TraceAction>) -> bool {
        let mut lfs_to_remove = context.gen_ctx.get_all_lfs_ids();
        for lf_id in context.co_localizations.get_coloc_lfs_ids(coloc_id) {
            lfs_to_remove.remove(lf_id);
        }
        let mono_coloc_interaction = interaction.eliminate_lifelines(&lfs_to_remove);
        let mut match_count = 0;
        for frt_elt in global_frontier(&mono_coloc_interaction, false) {
            if frt_elt.target_actions == *head_multi_action {
                match_count += 1;
                if match_count >= 2 {
                    return false;
                }
            }
        }
        true
    }

    pub fn get_action_matches_in_analysis(&self,
                                          use_partial_order_reduction : bool,
                                          algo_uses_lifeline_removal_steps : bool,
                                          context : &AnalysisContext,
                                          interaction : &Interaction,
                                          flags : &MultiTraceAnalysisFlags) -> Vec<AnalysisStepKind> {
        /**
         TODO: this new way of computing matches has allowed partial order readuction but it has broken
         TODO: matching for synchronous communications (e.g., the <synch> keyword)
         TODO: indeed, we match each individual head action to a frontier element
         TODO: but with synchronous comm several distinct head actions can be matched together with a single frontier element
         TODO: a quick fix is easy to do but still needs to be done
         **/
        // ***
        let mut coloc_ids_to_frontier_elts : HashMap<usize,Vec<FrontierElement>> = hashmap!{};
        let matching_heads : Vec<(usize,&BTreeSet<TraceAction>)> = {
            // collects multi-actions at the head of each local components
            let mut head_actions : Vec<(usize,&BTreeSet<TraceAction>)> = vec![];
            for (canal_id,canal_flags) in flags.canals.iter().enumerate() {
                let trace = context.multi_trace.get(canal_id).unwrap();
                let trace_len = trace.len();
                if trace_len > canal_flags.consumed {
                    let trace_head = trace.get(canal_flags.consumed).unwrap();
                    head_actions.push((canal_id,trace_head));
                }
            }

            // iter head actions
            for frt_elt in global_frontier(interaction,true) {
                // iter head actions to look for a match
                'iter_head : for (coloc_id,head) in head_actions.iter() {
                    // if there is a match keeps track of frt_elt
                    if frt_elt.target_actions == **head {
                        coloc_ids_to_frontier_elts.entry(*coloc_id)
                            .or_insert_with(Vec::new)
                            .push(frt_elt);
                        break 'iter_head;
                    }
                }
            }

            // only keeps head actions that have at least one match:
            head_actions.into_iter()
                .filter(|(coloc_id,_)| coloc_ids_to_frontier_elts.contains_key(coloc_id))
                .collect()
        };

        // ***
        if use_partial_order_reduction {
            // iter matching head actions
            for (coloc_id,head) in matching_heads.iter() {
                // if there is a univocal head action
                // chooses only this single head action
                // as the unique successor
                // implements linearization/partial order reduction
                if Self::is_action_univocal_in_analysis(context,
                                                        interaction,
                                                        *coloc_id,
                                                        head) {
                    // a univocal dominant head action has been found
                    let mut frt_elts = coloc_ids_to_frontier_elts.remove(coloc_id).unwrap();
                    assert_eq!(frt_elts.len(),1);
                    let unique_frt_elt = frt_elts.pop().unwrap();
                    //
                    let canal_ids_of_targets = context.co_localizations.get_coloc_ids_from_lf_ids(
                        &unique_frt_elt.target_lf_ids
                    );
                    let kind = AnalysisStepKind::Execute(unique_frt_elt,
                                                         canal_ids_of_targets,
                                                         hashmap!{});
                    return vec![kind];
                }
            }
        }

        // DEFAULT BEHAVIOR TO REVERT TO
        let mut next_steps = vec![];
        // iter immediately executable multi-actions
        for (_,frt) in coloc_ids_to_frontier_elts
                .into_iter()
                .sorted_by(|(x,_),(y,_)| x.cmp(y)) {
            for frt_elt in frt {
                let canal_ids_of_targets = context.co_localizations
                    .get_coloc_ids_from_lf_ids(&frt_elt.target_lf_ids);
                let kind = AnalysisStepKind::Execute(frt_elt,
                                                     canal_ids_of_targets,
                                                     hashmap!{});
                // ***
                next_steps.push( kind );
            }
        }
        return next_steps;

    }

}





