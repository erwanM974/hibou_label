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


use std::path::Path;
use graph_process_manager_loggers::stepstrace::printer::StepsTraceProcessPrinter;


use crate::io::output::to_hfiles::trace::to_htf::write_multi_trace_into_file;
use crate::loggers::tracegen::object::TraceGenLoggerObject;
use crate::loggers::tracegen::printer::MultiTraceProcessPrinter;
use crate::process::ana::conf::AnalysisConfig;
use crate::process::ana::context::AnalysisContext;
use crate::process::ana::node::node::AnalysisNodeKind;
use crate::process::ana::param::param::AnalysisParameterization;
use crate::process::ana::step::AnalysisStepKind;


impl StepsTraceProcessPrinter<AnalysisConfig,TraceGenLoggerObject> for MultiTraceProcessPrinter {

    fn get_initial_object(&self,
                          context: &AnalysisContext,
                          param: &AnalysisParameterization,
                          node: &AnalysisNodeKind) -> TraceGenLoggerObject {
        self.get_initial_multi_trace()
    }

    fn add_step_to_object(&self,
                          context: &AnalysisContext,
                          param: &AnalysisParameterization,
                          object: &TraceGenLoggerObject,
                          step: &AnalysisStepKind) -> TraceGenLoggerObject {
        match step {
            AnalysisStepKind::EliminateNoLongerObserved(_) => {
                object.clone()
            },
            AnalysisStepKind::Execute(frt_elt,_,_) => {
                self.add_actions_to_multi_trace(object,&frt_elt.target_actions)
            }
        }
    }

    fn should_print_on_node_reached(&self,
                                    context: &AnalysisContext,
                                    param: &AnalysisParameterization,
                                    node: &AnalysisNodeKind,
                                    node_depth: u32) -> bool {
        self.should_generate_multi_trace_on_interaction_reached(&node.interaction,node_depth)
    }

    fn print_object(&self,
                    context: &AnalysisContext,
                    param: &AnalysisParameterization,
                    object: &TraceGenLoggerObject,
                    path: &Path) {
        write_multi_trace_into_file(path,
                                    &context.gen_ctx,
                                    &self.partition,
                                    &object.mu);
    }
}


