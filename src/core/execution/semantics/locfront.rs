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
use crate::core::language::syntax::position::*;
use crate::core::language::syntax::action::ObservableAction;
use crate::core::language::syntax::interaction::*;
use crate::core::execution::semantics::frontier::global_frontier;


pub fn local_frontier(gen_ctx : &GeneralContext,
                      interaction : &Interaction,
                      co_localization : &HashSet<usize>) -> Vec<ObservableAction> {
    let mut lfs_to_remove : HashSet<usize> = HashSet::new();
    for lf_id in 0..gen_ctx.get_lf_num() {
        if !co_localization.contains(&lf_id) {
            lfs_to_remove.insert(lf_id);
        }
    }
    let hidden_interaction = interaction.hide(&lfs_to_remove);
    let mut local_frontier : Vec<ObservableAction> = Vec::new();
    for front_pos in global_frontier( &hidden_interaction ) {
        let front_act = hidden_interaction.get_sub_interaction(&front_pos).as_leaf();
        local_frontier.push(front_act.clone() );
    }
    return local_frontier;
}



