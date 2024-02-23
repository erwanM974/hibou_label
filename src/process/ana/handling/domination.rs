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
use itertools::Itertools;
use crate::core::execution::semantics::execute::execute_interaction;
use crate::core::execution::semantics::frontier::{FrontierElement, global_frontier};
use crate::core::execution::trace::trace::TraceAction;
use crate::core::language::eliminate_lf::eliminable::LifelineEliminable;
use crate::core::language::syntax::interaction::Interaction;
use crate::process::ana::context::AnalysisContext;

pub fn get_head_actions_ids_maps(algo_uses_lifeline_removal_steps : bool,
                                 context : &AnalysisContext,
                                 interaction : &Interaction,
                                 head_actions : &Vec<(usize,&BTreeSet<TraceAction>,bool)>)
    -> (HashMap<usize,HashSet<FrontierElement>>,HashMap<usize,HashSet<Interaction>>)

{
    let mut head_action_id_to_frt_elts : HashMap<usize,HashSet<FrontierElement>> = hashmap!{};
    let mut head_action_id_to_follow_ups : HashMap<usize,HashSet<Interaction>> = hashmap!{};

    // iter immediately executable multi-actions
    for frt_elt in global_frontier(interaction,true) {
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
    (head_action_id_to_frt_elts,head_action_id_to_follow_ups)
}




pub fn get_domination_domain(
    algo_uses_lifeline_removal_steps : bool,
    context : &AnalysisContext,
    head_actions : &Vec<(usize,&BTreeSet<TraceAction>,bool)>,
    head_action_id_to_follow_ups : &HashMap<usize,HashSet<Interaction>>,
    head_id_for_which_to_compute_domain : usize)
    -> HashSet<usize>
{

    let mut dominates : HashSet<usize> = hashset!{};
    let left = head_id_for_which_to_compute_domain;
    for right in head_action_id_to_follow_ups.keys().copied().filter(|right| *right != left) {
        let (_,left_actions,left_is_last) = head_actions.iter().filter(|(c,_,_)| *c == left).next().unwrap();
        let (_,right_actions,right_is_last) = head_actions.iter().filter(|(c,_,_)| *c == right).next().unwrap();
        let mut left_then_right = hashset!{};
        for left_follow_up in head_action_id_to_follow_ups.get(&left).unwrap() {
            for after_left in global_frontier(left_follow_up,true) {
                if after_left.target_actions == **right_actions {
                    let exe_result = execute_interaction(left_follow_up,
                                                         &after_left.position,
                                                         &after_left.target_lf_ids,
                                                         false);
                    let follow_up = if algo_uses_lifeline_removal_steps && *right_is_last {
                        let lfs_to_remove = context.co_localizations.get_coloc_lfs_ids(right);
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
            for after_right in global_frontier(right_follow_up,true) {
                if after_right.target_actions == **left_actions {
                    let exe_result = execute_interaction(right_follow_up,
                                                         &after_right.position,
                                                         &after_right.target_lf_ids,
                                                         false);
                    let follow_up = if algo_uses_lifeline_removal_steps && *left_is_last {
                        let lfs_to_remove = context.co_localizations.get_coloc_lfs_ids(left);
                        exe_result.interaction.eliminate_lifelines(lfs_to_remove)
                    } else {
                        exe_result.interaction
                    };
                    right_then_left.insert(follow_up);
                }
            }
        }
        //
        if right_then_left.is_subset(&left_then_right) {
            dominates.insert(right);
        }
    }
    dominates
}





pub fn get_domination_maps(
    algo_uses_lifeline_removal_steps : bool,
    context : &AnalysisContext,
    head_actions : &Vec<(usize,&BTreeSet<TraceAction>,bool)>,
    head_action_id_to_follow_ups : &HashMap<usize,HashSet<Interaction>>)
    -> (HashMap<usize,HashSet<usize>>,HashMap<usize,HashSet<usize>>)
{
// for every combination/pair A1,A2 of head action that have matches
    // determines whether or not A1 dominates A2 i.e.,
    // all follow ups that can be obtained by executing first A2 and then A1
    // can also be reached via executing first A1 and then A2
    let mut dominated_by : HashMap<usize,HashSet<usize>> = hashmap!{};
    let mut dominates : HashMap<usize,HashSet<usize>> = hashmap!{};
    // we use ".sorted()" so that the iter is deterministic
    for pair in head_action_id_to_follow_ups.keys().sorted().combinations(2) {
        let left = **pair.get(0).unwrap();
        let (left_coloc_id,left_actions,left_is_last) = head_actions.get(left).unwrap();
        let right = **pair.get(1).unwrap();
        let (right_coloc_id,right_actions,right_is_last) = head_actions.get(right).unwrap();
        let mut left_then_right = hashset!{};
        for left_follow_up in head_action_id_to_follow_ups.get(&left).unwrap() {
            for after_left in global_frontier(left_follow_up,true) {
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
            for after_right in global_frontier(right_follow_up,true) {
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
            dominates.entry(right)
                .and_modify(|over| { over.insert(left); })
                .or_insert(hashset![left]);
        }
        if right_then_left.is_subset(&left_then_right) {
            dominated_by.entry(right)
                .and_modify(|dom_by| { dom_by.insert(left); })
                .or_insert(hashset![left]);
            dominates.entry(left)
                .and_modify(|over| { over.insert(right); })
                .or_insert(hashset![right]);
        }
    }
    (dominated_by,dominates)
}



