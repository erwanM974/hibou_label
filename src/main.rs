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

extern crate strum;

#[macro_use]
extern crate strum_macros;

extern crate rusttype;

extern crate image;

extern crate imageproc;

extern crate pest;

#[macro_use]
extern crate pest_derive;

#[macro_use]
extern crate clap;

#[macro_use]
extern crate maplit;

extern crate rand;

// **********

pub mod core;
pub mod io;
pub mod ui;
pub mod plantuml;
pub mod process;
pub mod loggers;
pub mod util;
pub mod trace_manip;

// **********

use crate::ui::hibou_cli::hibou_cli;

fn main() {
    hibou_cli();
}
