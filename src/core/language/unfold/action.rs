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



use crate::core::language::syntax::action::{CommunicationSynchronicity, EmissionAction, EmissionTargetRef, ReceptionAction};
use crate::core::language::syntax::interaction::Interaction;
use crate::core::language::unfold::unfoldable::AtomicUnfoldableAsInteraction;




impl AtomicUnfoldableAsInteraction for EmissionAction {
    fn unfold_as_interaction(&self) -> Interaction {
        if self.targets.len() == 0 {
            return Interaction::Emission(self.clone());
        } else {
            let emission = EmissionAction::new(self.origin_lf_id,self.ms_id,CommunicationSynchronicity::Asynchronous,vec![]);
            let mut receptions = vec![];
            for target_ref in &self.targets {
                match target_ref {
                    EmissionTargetRef::Lifeline(tar_lf_id) => {
                        receptions.push( ReceptionAction::new(None,self.ms_id,CommunicationSynchronicity::Asynchronous,vec![*tar_lf_id]));
                    },
                    _ => {
                        // nothing
                    }
                }
            }
            return Interaction::Strict(Box::new(Interaction::Emission(emission)),
                                       Box::new(deploy_receptions(&mut receptions)));
        }
    }
}



impl AtomicUnfoldableAsInteraction for ReceptionAction {
    fn unfold_as_interaction(&self) -> Interaction {
        match self.recipients.len() {
            0 => {
                return Interaction::Empty;
            },
            1 => {
                return Interaction::Reception(self.clone());
            },
            _ => {
                let mut receptions = vec![];
                for rcp_lf_id in &self.recipients {
                    receptions.push( ReceptionAction::new(None,self.ms_id,CommunicationSynchronicity::Asynchronous,vec![*rcp_lf_id]));
                }
                return deploy_receptions(&mut receptions);
            }
        }
    }
}


fn deploy_receptions(rem_targets : &mut Vec<ReceptionAction>) -> Interaction {
    let rem_tlen = rem_targets.len();
    if rem_tlen == 0 {
        return Interaction::Empty;
    } else if rem_tlen == 1 {
        let rcp = rem_targets.remove(0);
        return Interaction::Reception(rcp);
    } else if rem_tlen == 2 {
        let rcp1 = rem_targets.remove(0);
        let rcp2 = rem_targets.remove(0);
        return Interaction::Seq( Box::new(Interaction::Reception(rcp1)), Box::new(Interaction::Reception(rcp2)) );
    } else {
        let rcp1 = rem_targets.remove(0);
        return Interaction::Seq(Box::new(Interaction::Reception(rcp1)), Box::new(deploy_receptions(rem_targets)));
    }
}


