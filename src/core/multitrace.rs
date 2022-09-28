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

// TODO: use this to refactor the current implementation of multitraces:
// TODO: decouple the multi-trace content's with the various flags and stuff for the analysis
// TODO: also in the graphical representation do not draw the multi-trace in the same drawing but rather draw two different and compose with Graphviz, it's cleaner I guess

use std::collections::HashSet;
use crate::core::trace::TraceAction;

pub struct MultiTraceCanal {
    pub co_localization : HashSet<usize>,
    pub trace : Vec<HashSet<TraceAction>>
}

pub type MultiTrace = Vec<MultiTraceCanal>;