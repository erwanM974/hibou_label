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



use graph_process_manager_loggers::stepstrace::object::ObjectToBuildWhenTracingSteps;
use crate::core::execution::trace::multitrace::MultiTrace;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct TraceGenLoggerObject {
    pub mu : MultiTrace
}

impl TraceGenLoggerObject {
    pub fn new(mu: MultiTrace) -> Self {
        TraceGenLoggerObject { mu }
    }
}

impl ObjectToBuildWhenTracingSteps for TraceGenLoggerObject {}


