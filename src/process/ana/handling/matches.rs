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
        for frt_elt in global_frontier(&interaction) {
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


    pub fn get_action_matches_in_analysis(&self,
                                          use_partial_order_reduction : bool,
                                          algo_uses_lifeline_removal_steps : bool,
                                          context : &AnalysisContext,
                                          interaction : &Interaction,
                                          flags : &MultiTraceAnalysisFlags) -> Vec<AnalysisStepKind> {
        // ***
        // collects multi-actions at the head of each local components
        // and keeps track if they are the last multi-action on that component via a boolean
        let mut head_actions : Vec<(usize,&BTreeSet<TraceAction>,bool)> = vec![];
        for (canal_id,canal_flags) in flags.canals.iter().enumerate() {
            let trace = context.multi_trace.get(canal_id).unwrap();
            let trace_len = trace.len();
            if trace_len > canal_flags.consumed {
                let trace_head = trace.get(canal_flags.consumed).unwrap();
                let is_last_on_canal = canal_flags.consumed == (trace_len - 1);
                head_actions.push((canal_id,trace_head,is_last_on_canal));
            }
        }
        // ***
        if use_partial_order_reduction {
            let mut head_action_id_to_frt_elts : HashMap<usize,HashSet<FrontierElement>> = hashmap!{};
            let mut head_action_id_to_follow_ups : HashMap<usize,HashSet<Interaction>> = hashmap!{};

            // iter immediately executable multi-actions
            for frt_elt in global_frontier(&interaction) {
                // iter head actions to look for a match
                'iter_head : for (x,(coloc_id,head,is_last)) in head_actions.iter().enumerate() {
                    // if there is a match keeps track of frt_elt and the follow_up interaction
                    if frt_elt.target_actions == **head {
                        let exe_result = execute_interaction(interaction,
                                                             &frt_elt.position,
                                                             &frt_elt.target_lf_ids,
                                                             false);
                        let follow_up = if algo_uses_lifeline_removal_steps && *is_last {
                            let lfs_to_remove = context.co_localizations.get_coloc_lfs_ids(*coloc_id);
                            exe_result.interaction.eliminate_lifelines(lfs_to_remove)
                        } else {
                            exe_result.interaction
                        };
                        head_action_id_to_follow_ups.entry(x)
                            .or_insert_with(HashSet::new)
                            .insert(follow_up);
                        head_action_id_to_frt_elts.entry(x)
                            .or_insert_with(HashSet::new)
                            .insert(frt_elt);
                        break 'iter_head;
                    }
                }
            }

            // for every combination/pair A1,A2 of head action that have matches
            // determines whether or not A1 dominates A2 i.e.,
            // all follow ups that can be obtained by executing first A2 and then A1
            // can also be reached via executing first A1 and then A2
            let mut dominated_by : HashMap<usize,HashSet<usize>> = hashmap!{};
            // we use ".sorted()" so that the iter is deterministic
            for pair in head_action_id_to_follow_ups.keys().sorted().combinations(2) {
                let left = **pair.get(0).unwrap();
                let (left_coloc_id,left_actions,left_is_last) = head_actions.get(left).unwrap();
                let right = **pair.get(1).unwrap();
                let (right_coloc_id,right_actions,right_is_last) = head_actions.get(right).unwrap();
                let mut left_then_right = hashset!{};
                for left_follow_up in head_action_id_to_follow_ups.get(&left).unwrap() {
                    for after_left in global_frontier(left_follow_up) {
                        if after_left.target_actions == **right_actions {
                            let exe_result = execute_interaction(left_follow_up,
                                                                 &after_left.position,
                                                                 &after_left.target_lf_ids,
                                                                 false);
                            let follow_up = if algo_uses_lifeline_removal_steps && *right_is_last {
                                let lfs_to_remove = context.co_localizations.get_coloc_lfs_ids(*right_coloc_id);
                                exe_result.interaction.eliminate_lifelines(lfs_to_remove)
                            } else {
                                exe_result.interaction
                            };
                            left_then_right.insert(follow_up);
                        }
                    }
                }
                let mut right_then_left = hashset!{};
                for right_follow_up in head_action_id_to_follow_ups.get(&right).unwrap() {
                    for after_right in global_frontier(right_follow_up) {
                        if after_right.target_actions == **left_actions {
                            let exe_result = execute_interaction(right_follow_up,
                                                                 &after_right.position,
                                                                 &after_right.target_lf_ids,
                                                                 false);
                            let follow_up = if algo_uses_lifeline_removal_steps && *left_is_last {
                                let lfs_to_remove = context.co_localizations.get_coloc_lfs_ids(*left_coloc_id);
                                exe_result.interaction.eliminate_lifelines(lfs_to_remove)
                            } else {
                                exe_result.interaction
                            };
                            right_then_left.insert(follow_up);
                        }
                    }
                }
                //
                if left_then_right.is_subset(&right_then_left) {
                    dominated_by.entry(left)
                        .and_modify(|dom_by| { dom_by.insert(right); })
                        .or_insert(hashset![right]);
                }
                if right_then_left.is_subset(&left_then_right) {
                    dominated_by.entry(right)
                        .and_modify(|dom_by| { dom_by.insert(left); })
                        .or_insert(hashset![left]);
                }
            }


            /** BELOW WAS AN ATTEMPT WITH ITERATIVE ELIMINATIONS BUT IT MAKES FALSE NEGATIVES IN THE ANALYSIS **/
            /*
            // we use a VEC so that iteration is deterministic
            let mut remaining_heads : Vec<usize> = head_action_id_to_follow_ups.keys().copied().sorted().collect();
            //println!("there are {:} heads, starting removing..", remaining_heads.len());
            // remove Condorcet losers from the remaining heads as long as there are Condorcet losers
            'outer_loop : loop {
                if remaining_heads.len() == 1 {
                    // of course, we should not remove the last remaining head,
                    // even if it is dominated
                    break 'outer_loop;
                }
                let mut loser = None;
                'iter_heads : for (x,head_id) in remaining_heads.iter().enumerate() {
                    if let Some(head_id_is_dominated_by) = dominated_by.get(head_id) {
                        //println!("head {:} is dominated by: {:?}",head_id, head_id_is_dominated_by);
                        let the_remaining_others : HashSet<usize> = remaining_heads
                            .iter()
                            .copied()
                            .filter(|x| *x != *head_id)
                            .collect();
                        //println!("the other remaining heads are : {:?}",the_remaining_others);
                        if the_remaining_others.is_subset(head_id_is_dominated_by) {
                            //println!("this includes all the {:} other remaining heads",the_remaining_others.len());
                            loser = Some(x);
                            break 'iter_heads;
                        }
                    }
                }
                if let Some(loser_index) = loser {
                    // a Condorcet loser has been found and must be removed
                    remaining_heads.remove(loser_index);
                } else {
                    // no Condorcet loser remains
                    break 'outer_loop;
                }
            }

             */

            let all_heads : Vec<usize> = head_action_id_to_follow_ups.keys().copied().sorted().collect();
            let mut non_loser_heads : Vec<usize> = vec![];
            for head_id in all_heads.iter() {
                if let Some(head_id_is_dominated_by) = dominated_by.get(head_id) {
                    let the_others : HashSet<usize> = all_heads
                        .iter()
                        .copied()
                        .filter(|x| *x != *head_id)
                        .collect();
                    let is_loser = the_others.is_subset(head_id_is_dominated_by);
                    if !is_loser {
                        non_loser_heads.push(*head_id);
                    }
                }
            }

            // gathers final next_steps
            let mut next_steps = vec![];
            for head_id in non_loser_heads {
                let frt_elts = head_action_id_to_frt_elts.remove(&head_id).unwrap();
                for frt_elt in frt_elts {
                    let canal_ids_of_targets = context.co_localizations.get_coloc_ids_from_lf_ids(&frt_elt.target_lf_ids);
                    let kind = AnalysisStepKind::Execute(frt_elt,
                                                         canal_ids_of_targets,
                                                         hashmap!{});
                    // ***
                    next_steps.push( kind );
                }
            }
            return next_steps;
        } else {
            let mut next_steps = vec![];
            // iter immediately executable multi-actions
            for frt_elt in global_frontier(&interaction) {
                // iter head actions to look for a match
                'iter_head : for (_,head,_) in head_actions.iter() {
                    if frt_elt.target_actions == **head {
                        let canal_ids_of_targets = context.co_localizations
                            .get_coloc_ids_from_lf_ids(&frt_elt.target_lf_ids);
                        let kind = AnalysisStepKind::Execute(frt_elt,
                                                             canal_ids_of_targets,
                                                             hashmap!{});
                        // ***
                        next_steps.push( kind );
                        break 'iter_head;
                    }
                }
            }
            return next_steps;
        }

    }

}





