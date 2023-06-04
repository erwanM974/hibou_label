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


use std::collections::BTreeSet;
use crate::core::general_context::GeneralContext;
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};

pub fn get_alphabet_from_gen_ctx(gen_ctx : &GeneralContext) -> Vec<BTreeSet<TraceAction>> {
    let mut alphabet = vec![];
    for lf in 0..gen_ctx.get_lf_num() {
        for ms in 0..gen_ctx.get_ms_num() {
            alphabet.push(btreeset!{TraceAction::new(lf,TraceActionKind::Emission,ms)});
            alphabet.push(btreeset!{TraceAction::new(lf,TraceActionKind::Reception,ms)});
        }
    }
    alphabet
}