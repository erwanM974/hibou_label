use std::fs;
use std::collections::HashMap;
use std::collections::btree_map::BTreeMap;
use std::path::Path;

use pest::iterators::Pair;

use crate::pest::Parser;

use crate::core::syntax::interaction::Interaction;
use crate::core::syntax::action::*;
use crate::core::general_context::GeneralContext;


use crate::from_hfiles::error::HibouParsingError;
use crate::process::log::ProcessLogger;


pub fn interaction_to_hsf(interaction : &Interaction, gen_ctx : &GeneralContext, file_name : &String) {
    //TODO
}