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


use std::fmt;


use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::position::Position;

use crate::core::trace::TraceAction;
use crate::core::syntax::action::ObservableAction;



#[derive(Debug)]
pub enum HibouCoreError {
    UnknownLifeline(usize),
    UnknownMessage(usize)
}

impl fmt::Display for HibouCoreError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            HibouCoreError::UnknownLifeline( lf_id ) => {
                return write!(f, "{}", format!("context error ; unknown lifeline : {:}", lf_id));
            },
            HibouCoreError::UnknownMessage( ms_id ) => {
                return write!(f, "{}", format!("context error ; unknown message : {:}", ms_id));
            }
        }
    }
}
