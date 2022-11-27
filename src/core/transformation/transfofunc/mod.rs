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

pub mod action;
pub mod dedupl;
pub mod defactorize;
pub mod factorize_par;
pub mod factorize_prefix;
pub mod factorize_suffix;
pub mod flush;
pub mod invert;
pub mod loop_alt_simpl;
pub mod loop_simpl;
pub mod merge_action;
pub mod merge_skip;
pub mod merge_skip_invert;
pub mod merge_shift_left;
pub mod merge_shift_right;
pub mod simpl;
