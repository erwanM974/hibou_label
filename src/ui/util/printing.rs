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





fn get_ascii_border() -> &'static str {
    return r#"===================="#;
}

fn get_ascii_left() -> Vec<&'static str> {
    let mut my_vec = Vec::new();
    my_vec.push(r#" ___   Holistic   "#);
    my_vec.push(r#"(o,o)  Interaction"#);
    my_vec.push(r#"{`"'}  Behavioral "#);
    my_vec.push(r#"-"-"-  Oracle     "#);
    my_vec.push(r#" \_/   Utility    "#);
    my_vec.push(r#"                  "#);
    my_vec.push(r#"  V-label-0.8.5   "#);
    return my_vec;
}

pub fn print_on_hibou_cli(ret_print : Vec<String>) {
    let ascii_left = get_ascii_left();
    // ***
    println!("{}", get_ascii_border());
    if ret_print.len() >= ascii_left.len() {
        for i in 0..ascii_left.len() {
            println!("{}  |  {}", ascii_left[i], ret_print[i]);
        }
        for i in ascii_left.len()..ret_print.len() {
            println!("{} |  {}", " ".repeat(19),  ret_print[i]);
        }
    } else {
        for i in 0..ret_print.len() {
            println!("{}  |  {}", ascii_left[i], ret_print[i]);
        }
        for i in ret_print.len()..ascii_left.len() {
            println!("{}  |", ascii_left[i]);
        }
    }
    println!("{}", get_ascii_border());
}

