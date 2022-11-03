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



use std::collections::HashSet;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};
use crate::core::execution::trace::trace::TraceAction;




pub fn mutate_by_inserting_actions(multi_trace : &MultiTrace,
                                   max_num_inserts : usize,
                                   one_per_compo_max : bool,
                                   only_at_end : bool,
                                   actions_and_colocs : Vec<(HashSet<TraceAction>,usize)>) -> MultiTrace {
    // ***
    let mut rem_num_inserts : usize;
    if one_per_compo_max {
        rem_num_inserts = max_num_inserts.max(multi_trace.len());
    } else {
        rem_num_inserts = max_num_inserts;
    }
    // ***
    let mut rng = rand::thread_rng();
    let rng_coloc_indices = Uniform::from(0..multi_trace.len() );
    // ***
    let mut seen_colocs : HashSet<usize> = hashset!{};
    let mut mutated_mt = multi_trace.clone();
    while rem_num_inserts > 0 {
        let coloc_id : usize = rng_coloc_indices.sample(&mut rng);
        if one_per_compo_max && seen_colocs.contains(&coloc_id) {
            // nothing
        } else {
            let component : &mut Trace = mutated_mt.get_mut(coloc_id).unwrap();
            let mut candidate_actions : Vec<&HashSet<TraceAction>> = actions_and_colocs.iter()
                .filter(|(a,idx)| *idx == coloc_id)
                .map(|(a,idx)| a).collect();
            if candidate_actions.len() > 0 {
                candidate_actions.shuffle(&mut rng);
                let to_insert : HashSet<TraceAction> = (*candidate_actions.get(0).unwrap()).clone();
                if only_at_end {
                    component.push(to_insert);
                } else {
                    mutate_vector_by_insert(&mut rng,component,to_insert);
                }
                rem_num_inserts -= 1;
            }
        }
    }
    return mutated_mt;
}



fn mutate_vector_by_insert<T>(rng : &mut ThreadRng, vector : &mut Vec<T>, to_insert : T) {
    let vec_length = vector.len();
    if vec_length > 0 {
        let mut vec_indices : Vec<usize> = (0..vec_length).collect();
        vec_indices.shuffle(rng);
        vector.insert(*vec_indices.get(0).unwrap(), to_insert);
    } else {
        vector.push(to_insert);
    }
}




#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use super::{mutate_vector_by_insert};

    #[test]
    fn mutate_insert_test() {
        let orig_vec : Vec<u32> = vec![0,1,2];
        let mut myvec : Vec<u32> = orig_vec.clone();
        let mut rng = rand::thread_rng();
        mutate_vector_by_insert(&mut rng, &mut myvec, 4);
        println!("{:?}", myvec);
        assert_ne!(myvec,orig_vec);
        // ***
        let expected : HashSet<Vec<u32>> = hashset!{vec![4,0,1,2],vec![0,4,1,2],vec![0,1,4,2],vec![0,1,2,4]};
        assert!(expected.contains(&myvec));
    }
}
