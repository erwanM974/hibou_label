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


pub enum TracegenProcessLoggerGeneration {
    exact,      // generate a trace file only for exactly accepted traces
                //   i.e. at each new node verify express empty and if it is the case generate
    prefixes,   // generate a trace file for each global prefixes
                //   i.e. at each new node generate
    /*
    multipref,  // generate a trace file for all multiprefixes
                //   works as "exact" but then, reload traces and generate prefixes
    multislices // generate a trace file for all multislices
                //   works as "exact" but then, reload traces and generate slices
     */
}



