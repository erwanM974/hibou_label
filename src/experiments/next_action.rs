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



use crate::core::general_context::GeneralContext;
use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction};
use crate::core::language::syntax::interaction::Interaction;

pub struct NextActionSpec {
    pub next_lf : usize,
    pub next_ms : usize
}

impl NextActionSpec {
    pub fn new(next_lf: usize, next_ms: usize) -> Self {
        Self { next_lf, next_ms }
    }

    pub fn get_next_action(&mut self,
                           gen_ctx : &GeneralContext) -> Interaction {
        let act = EmissionAction::new(
            self.next_lf,
            self.next_ms,
            CommunicationSynchronicity::Asynchronous,
            vec![]
        );
        let int = Interaction::Emission(act);
        if self.next_ms + 1 < gen_ctx.get_ms_num() {
            self.next_ms += 1;
        } else {
            if self.next_lf + 1 < gen_ctx.get_lf_num() {
                self.next_ms = 0;
                self.next_lf += 1;
            } else {
                self.next_ms = 0;
                self.next_lf = 0;
            }
        }
        int
    }
}

