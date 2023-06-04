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




use std::collections::{BTreeSet, HashSet};
use std::time::{Duration, Instant};
use autour_core::nfa::nfa::AutNFA;
use autour_core::traits::translate::AutTranslatable;
use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::manager::manager::GenericProcessManager;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graph_process_manager_loggers::nfait::logger::GenericNFAITLogger;
use crate::core::execution::trace::trace::TraceAction;
use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::interaction::Interaction;
use crate::process::explo::conf::ExplorationConfig;
use crate::process::explo::context::{ExplorationContext, ExplorationParameterization};
use crate::process::explo::filter::filter::ExplorationFilter;
use crate::process::explo::loggers::nfait::printer::ActionNFAITPrinter;
use crate::process::explo::node::ExplorationNodeKind;
use crate::process::explo::priorities::ExplorationPriorities;
use crate::process::explo::step::ExplorationStepKind;


pub fn get_nfa_from_interaction_exploration(gen_ctx : &GeneralContext,
                                            int : &Interaction,
                                            base_alphabet : Vec<BTreeSet<TraceAction>>)
            -> (AutNFA<usize>,Duration) {

    let usize_alphabet : HashSet<usize> = (0..base_alphabet.len()).collect();
    let max_loop_depth = int.total_loop_num()*2;
    let nfa_logger = GenericNFAITLogger::new(
        ActionNFAITPrinter::new(base_alphabet,
                                gen_ctx.clone()),
                                             "nfa".to_string(),
                                             None,
                                             ".".to_string());
    let explo_ctx = ExplorationContext::new(gen_ctx.clone());
    let delegate : GenericProcessDelegate<ExplorationStepKind,ExplorationNodeKind,ExplorationPriorities> =
        GenericProcessDelegate::new(QueueSearchStrategy::BFS,
                                    GenericProcessPriorities::new(ExplorationPriorities::default(),false));

    let mut exploration_manager : GenericProcessManager<ExplorationConfig> =
        GenericProcessManager::new(explo_ctx,
                                   ExplorationParameterization{},
                                   delegate,
                                   vec![Box::new(ExplorationFilter::MaxLoopInstanciation(max_loop_depth))],
                                   vec![Box::new(nfa_logger)],
                                   None,
                                   true);

    // ***
    // ***
    let init_node = ExplorationNodeKind::new(int.clone(),0);
    // ***
    let now = Instant::now();
    let (node_count,_) = exploration_manager.start_process(init_node);
    let elapsed_get_nfa = now.elapsed();
    // ***
    let raw_logger = exploration_manager.get_logger(0).unwrap();
    let nfa_logger : &GenericNFAITLogger<ExplorationConfig,usize,ActionNFAITPrinter> =
        raw_logger.as_any().downcast_ref::<GenericNFAITLogger<ExplorationConfig,usize,ActionNFAITPrinter>>().unwrap();
    // ***
    let mut nfa = nfa_logger.get_nfait().to_nfa();
    let printer = nfa_logger.builder_printer.clone();
    // ***
    nfa.alphabet = usize_alphabet;
    // ***
    return (nfa, elapsed_get_nfa);
}