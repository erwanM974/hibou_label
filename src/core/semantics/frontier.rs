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

use crate::core::syntax::position::*;
use crate::core::syntax::interaction::*;



pub fn global_frontier(interaction : &Interaction) -> Vec<Position> {
    match interaction {
        &Interaction::Empty => {
            return Vec::new();
        },
        &Interaction::Action(_) => {
            return vec![Position::Epsilon];
        },
        &Interaction::Strict(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, global_frontier(i1));
            if i1.express_empty() {
                front.append( &mut push_frontier(&PositionKind::Right, global_frontier(i2)) )
            }
            return front;
        },
        &Interaction::Seq(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, global_frontier(i1));
            for pos2 in push_frontier(&PositionKind::Right, global_frontier(i2)) {
                let act = interaction.get_sub_interaction(&pos2 ).as_leaf();
                if i1.avoids(act.lf_id) {
                    front.push(pos2);
                }
            }
            return front;
        },
        &Interaction::Alt(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, global_frontier(i1));
            front.append( &mut push_frontier(&PositionKind::Right, global_frontier(i2)) );
            return front;
        },
        &Interaction::Par(ref i1, ref i2) => {
            let mut front = push_frontier(&PositionKind::Left, global_frontier(i1));
            front.append( &mut push_frontier(&PositionKind::Right, global_frontier(i2)) );
            return front;
        },
        &Interaction::Loop(_, ref i1) => {
            return push_frontier(&PositionKind::Left, global_frontier(i1));
        }
    }
}



enum PositionKind {
    Left,
    Right
}

fn push_frontier(pkind : &PositionKind, frontier : Vec<Position>) -> Vec<Position> {
    let mut new_frontier : Vec<Position> = Vec::new();
    // ***
    for my_pos in frontier {
        let new_pos : Position;
        match pkind {
            PositionKind::Left => {
                new_pos = Position::Left( Box::new(my_pos ) );
            },
            PositionKind::Right => {
                new_pos = Position::Right( Box::new(my_pos ) );
            }
        }
        new_frontier.push( new_pos );
    }
    // ***
    return new_frontier;
}


