

# HIBOU Readme

## Documentation 

We propose a short documentation organized as follows:
- a section about [interaction models and their encoding](https://github.com/erwanM974/hibou_label/blob/master/readme/1_interaction.md).
- a section about [the execution and animation of interaction models](https://github.com/erwanM974/hibou_label/blob/master/readme/2_execution.md).
- a section about [the exploration of an interaction's semantics with HIBOU](https://github.com/erwanM974/hibou_label/blob/master/readme/3_tool_sem_explo.md).
- a section about [the notions of trace and multi-trace](https://github.com/erwanM974/hibou_label/blob/master/readme/4_traces.md).
- a section about [(multi-)trace analysis in full observation with HIBOU](https://github.com/erwanM974/hibou_label/blob/master/readme/5_mu_ana.md).
- a section about [(multi-)trace analysis in partial observation with HIBOU](https://github.com/erwanM974/hibou_label/blob/master/readme/6_partial.md).

## How to install/compile

You can build the Rust project with cargo using "cargo build --release".

This will generate an executable in "./target/release".

Or you could download the provided binaries for windows (compiled on Windows 10) and linux (compiled on Ubuntu 20.04).

## Requirements / Dependencies

So as to generate the images of the graphs, you will need to have graphviz installed on your system.
Graphviz is available at [https://www.graphviz.org/download/](https://www.graphviz.org/download/).

The "dot" command provided by Graphviz must be in your "PATH" environment variable.

## Examples

All the examples in this README are provided in the "examples" directory.


