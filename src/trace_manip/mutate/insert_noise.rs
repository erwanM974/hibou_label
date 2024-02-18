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


use std::collections::{BTreeSet, HashSet};
use std::fs;
use std::path::PathBuf;
use rand::Rng;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::distributions::{Distribution, Uniform};
use crate::core::colocalizations::CoLocalizations;
use crate::core::execution::trace::multitrace::{MultiTrace, Trace};
use crate::core::execution::trace::trace::{TraceAction, TraceActionKind};
use crate::core::general_context::GeneralContext;
use crate::io::file_extensions::HIBOU_TRACE_FILE_EXTENSION;
use crate::io::output::to_hfiles::trace::to_htf::write_multi_trace_into_file;


pub fn generate_insert_noise_mutant(gen_ctx : &GeneralContext,
                                    co_localizations : &CoLocalizations,
                                    multi_trace : &MultiTrace,
                                    parent_folder : Option<&str>,
                                    mutant_name : &str,
                                    max_num_inserts : u32,
                                    one_per_compo_max : bool,
                                    only_at_end : bool) -> String{
    let file_name = format!("{:}.{:}", mutant_name, HIBOU_TRACE_FILE_EXTENSION);
    let path : PathBuf;
    match parent_folder {
        None => {
            path = [&file_name].iter().collect();
        },
        Some( parent ) => {
            // creates directory
            fs::create_dir_all(parent).unwrap();
            path = [parent, &file_name].iter().collect();
        }
    }
    // ***
    let mutant_mt = mutate_by_inserting_noise(gen_ctx,co_localizations,multi_trace,max_num_inserts,one_per_compo_max,only_at_end);
    write_multi_trace_into_file(path.as_path(),
                                gen_ctx,
                                co_localizations,
                                &mutant_mt);
    return path.into_os_string().to_str().unwrap().to_string();
}


fn generate_random_action_on_coloc(rng : &mut ThreadRng,
                                   gen_ctx : &GeneralContext,
                                   coloc_lfs : &BTreeSet<usize>) -> TraceAction {
    let lf_id : usize;
    {
        let coloc_lfs_as_vec : Vec<usize> = coloc_lfs.iter().cloned().collect();
        let rng_lf_id_idx = Uniform::from(0..coloc_lfs_as_vec.len() );
        let got_id : usize = rng_lf_id_idx.sample(rng);
        lf_id = *coloc_lfs_as_vec.get(got_id).unwrap();
    }
    let ms_id : usize;
    {
        let msgs_as_vec : Vec<usize> = (0..gen_ctx.get_ms_num()).collect();
        let rng_ms_id_idx = Uniform::from(0..msgs_as_vec.len() );
        let got_id : usize = rng_ms_id_idx.sample(rng);
        ms_id = *msgs_as_vec.get(got_id).unwrap();
    }
    if rng.gen::<bool>() {
        return TraceAction::new(lf_id,TraceActionKind::Emission,ms_id);
    } else {
        return TraceAction::new(lf_id,TraceActionKind::Reception,ms_id);
    }
}


fn mutate_by_inserting_noise(gen_ctx : &GeneralContext,
                               co_localizations : &CoLocalizations,
                               multi_trace : &MultiTrace,
                                   max_num_inserts : u32,
                                   one_per_compo_max : bool,
                                   only_at_end : bool) -> MultiTrace {
    // ***
    let mut rem_num_inserts : u32;
    if one_per_compo_max {
        rem_num_inserts = max_num_inserts.max(multi_trace.len() as u32);
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
            let to_insert = generate_random_action_on_coloc(&mut rng,gen_ctx,co_localizations.get_coloc_lfs_ids(coloc_id));
            if only_at_end {
                component.push(btreeset!{to_insert});
            } else {
                mutate_vector_by_insert(&mut rng,component,btreeset!{to_insert});
            }
            rem_num_inserts -= 1;
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
