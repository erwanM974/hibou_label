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



use std::collections::{HashMap, HashSet};
use std::ops::DerefMut;
use crate::core::general_context::GeneralContext;
use crate::process::abstract_proc::common::{FilterEliminationKind, HibouSearchStrategy};


pub trait AbstractConfiguration : Sized {
    type NodeKind : AbstractNodeKind;
    type StepKind : AbstractStepKind<Self>;
    type Priorities : AbstractPriorities;
    type Filter : AbstractFilter<Self>;
    type FilterCriterion;
    type Logger;
}

pub trait AbstractPriorities : Sized {

    fn print_as_string(&self) -> String;

}


pub trait AbstractStepKind<Config : AbstractConfiguration> : Sized {

    fn get_priority(&self, process_priorities : &Config::Priorities) -> i32;

}

pub trait AbstractFilter<Config : AbstractConfiguration> : Sized + std::string::ToString {

    fn apply_filter(&self, depth : u32, node_counter : u32, criterion : &Config::FilterCriterion) -> Option<FilterEliminationKind>;

}

pub trait AbstractNodeKind : Sized {

}

pub struct GenericNode<Config : AbstractConfiguration> {
    pub kind : Config::NodeKind,
    pub remaining_ids_to_process : HashSet<u32>,
    pub depth : u32
}

pub struct GenericStep<Config : AbstractConfiguration> {
    pub parent_id : u32,
    pub id_as_child : u32,
    pub kind : Config::StepKind
}

impl<Config : AbstractConfiguration> GenericStep<Config> {
    pub fn new(parent_id : u32,
               id_as_child : u32,
               kind : Config::StepKind) -> GenericStep<Config> {
        return GenericStep{parent_id,id_as_child,kind};
    }
}
