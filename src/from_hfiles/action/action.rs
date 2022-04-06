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


use pest::iterators::{Pair,Pairs};

use crate::core::syntax::action::*;
use crate::core::syntax::interaction::*;
use crate::core::general_context::GeneralContext;

use crate::from_hfiles::parser::*;
use crate::from_hfiles::error::HibouParsingError;

use crate::from_hfiles::action::act_content::*;
use crate::from_hfiles::action::act_targets::*;



pub fn parse_communication_action(gen_ctx : &GeneralContext, contents : &mut Pairs<Rule>) -> Result<Interaction,HibouParsingError> {
    let comm_act_content_pair : Pair<Rule>;
    let comm_act_target_pair : Pair<Rule>;
    let mut origin_info : Option<ParsedReference> = None;
    // ***
    let first_pair = contents.next().unwrap();
    match first_pair.as_rule() {
        Rule::SD_COMMUNICATION_ORIGIN => {
            match parse_comm_act_origin(gen_ctx,first_pair) {
                Err(e) => {
                    return Err(e);
                },
                Ok( parsed_ref ) => {
                    origin_info = Some(parsed_ref);
                    comm_act_content_pair = contents.next().unwrap();
                    comm_act_target_pair = contents.next().unwrap();
                }
            }
        },
        Rule::SD_COMMUNICATION_CONTENT => {
            comm_act_content_pair = first_pair;
            comm_act_target_pair = contents.next().unwrap();
        },
        _ => {
            panic!("what rule then ? : {:?}", first_pair.as_rule() );
        }
    }
    // ***
    match parse_comm_content(gen_ctx,comm_act_content_pair) {
        Err(e) => {
            return Err(e);
        },
        Ok( (comm_synchro, ms_id) ) => {
            match origin_info {
                None => {
                    match parse_comm_act_targets_as_lifelines(gen_ctx,comm_act_target_pair) {
                        Err(e) => {
                            return Err(e);
                        },
                        Ok( tar_lf_ids) => {
                            let rc_act = ReceptionAction::new(None,ms_id,comm_synchro,tar_lf_ids);
                            return Ok( Interaction::Reception(rc_act) );
                        }
                    }
                },
                Some( parsed_ref ) => {
                    match parsed_ref {
                        ParsedReference::GateRef( gt_id ) => {
                            match parse_comm_act_targets_as_lifelines(gen_ctx,comm_act_target_pair) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( tar_lf_ids) => {
                                    let rc_act = ReceptionAction::new(Some(gt_id),ms_id,comm_synchro,tar_lf_ids);
                                    return Ok( Interaction::Reception(rc_act) );
                                }
                            }
                        },
                        ParsedReference::LifelineRef( lf_id ) => {
                            match parse_comm_act_targets_as_generic_targets(gen_ctx,comm_act_target_pair) {
                                Err(e) => {
                                    return Err(e);
                                },
                                Ok( tar_refs) => {
                                    let em_act = EmissionAction::new(lf_id,ms_id,comm_synchro,tar_refs);
                                    return Ok( Interaction::Emission(em_act) );
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}