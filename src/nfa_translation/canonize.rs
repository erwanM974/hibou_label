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


use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::io::input::hcf::canon::options::HibouCanonizeOptions;
use crate::process::canon::conf::CanonizationConfig;
use crate::process::canon::context::CanonizationContext;
use crate::process::canon::node::CanonizationNodeKind;
use crate::process::canon::param::default::DefaultCanonizationProcess;
use crate::process::canon::param::phase::CanonizationParameterization;
use crate::process::canon::priorities::CanonizationPriorities;
use crate::process::canon::step::CanonizationStepKind;

pub fn canonize_interaction(gen_ctx : &GeneralContext,
                            int : &Interaction,
                            def : DefaultCanonizationProcess) -> Interaction {
    let canon_opts = HibouCanonizeOptions::default();

    let canon_ctx = CanonizationContext::new(gen_ctx.clone());
    let delegate : GenericProcessDelegate<CanonizationStepKind,CanonizationNodeKind,CanonizationPriorities> =
        GenericProcessDelegate::new(
            canon_opts.strategy,
            canon_opts.priorities
        );

    let canon_param = CanonizationParameterization::from_default(def,canon_opts.search_all);

    let mut canon_manager : GenericProcessManager<CanonizationConfig> = GenericProcessManager::new(
        canon_ctx,
        canon_param,
        delegate,
        canon_opts.filters,
        canon_opts.loggers,
        None,
        true
    );
    // ***
    let init_node = CanonizationNodeKind::new(int.clone(),0);
    // ***
    let (_,mut verdict) = canon_manager.start_process(init_node);
    if verdict.canonized_ints.len() != 1 {
        println!("warning got {:} differents canonized interactions", verdict.canonized_ints.len());
    }
    verdict.canonized_ints.remove(0)
}
