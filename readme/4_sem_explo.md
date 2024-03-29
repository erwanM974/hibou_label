

# The exploration of an interaction's semantics with HIBOU

HIBOU proposes a number of commands via a Command Line Interface (CLI). This includes an "explore" command which we detail in this section.
The "explore" command (to be used as "hibou_label explore <.hsf file>") computes (partially or totally if possible) the semantics of a given interaction.

The "explore" command of HIBOU can generate execution trees (drawings) which illustrate the semantics of a given interaction model.
Below is given an example exploration drawing that you can obtain by typing
"hibou_label.exe explore ex_readme.hsf" with the files from "examples" folder.

<img src="./images/4/ex_readme_explo.svg" alt="exploration" width="950">

In the header of a ".hsf" file, we can configure a number of options that will then be used if the ".hsf" file is exploited
with the "explore command". Those options are set within an "@explore_option" section. Here we used the following options for the exploration:

```
@explore_option{
  loggers = [graphic[svg,vertical]];
  strategy = DepthFS;
  ilters = [ max_depth = 3,
             max_loop_depth = 1,
             max_node_number = 7 ];
  priorities = [ loop=0, 
                 emission=0, 
                 reception = 0]
}
```

## Loggers 

We can specify that we want the exploration of this ".hsf" file to be logged in a certain manner with the "loggers" attribute.

### Graphic logger

We can configure a "graphic" logger to create an image file (with the same name as the ".hsf" file)
describing the exploration.
The generation of this image requires the graphviz tool to be installed ([https://www.graphviz.org/download/](https://www.graphviz.org/download/)),
and the "dot" command to be in the "PATH" environment variable.

The output of the graphic logger can be configured by certain options as such ``graphic[options]``.
Here we have ``graphic[svg,vertical]``, which means the output will be a .svg file (requires cairo to be installed)
and the graph will have a vertical layout.
With ``graphic[png]``, the output would be a .png file.
And with ``graphic[horizontal]``, the layout of the graph would be horizontal.

### Trace generation logger

We can configure a "trace generation" logger that will create ".htf" files for traces corresponding
to paths of the part of the execution tree that is uncovered during its exploration.

The trace generation logger can be declared and configured via ``tracegen[options]``:
- an option specifies under which circumstances a trace file can be created: 
  - ``tracegen[exact]`` only allows the creation of a trace when reaching an interaction
  which can express the empty trace (hence all generated traces are exactly accepted by the initial interaction).
  - ``tracegen[prefix]`` creates trace files for all interactions that are reached and hence will
  create trace files for all prefixes of accepted traces
  - ``tracegen[terminal]`` only creates trace file on terminal nodes of the explored tree.
  Nodes can be terminal either because there are no more actions to be executed or because of
  an set limitation on the exploration. In any case, this allows generating fewer trace files,
  with no trace being a prefix of another.


## Search strategy 

A search strategy: Breadth First Search (BreadthFS) or Depth First Search (DepthFS) can be specified using the "strategy" option.
Indeed, from any given interaction, several frontier actions may be evaluated, leading to several child follow-up interactions in the tree.
We can then explore those child nodes and their children using any search heuristic.

## Filters 

We can specify a number of filters that will limit the exploration
of graphs in algorithmic treatments in
different ways.
- "max_depth" limits the depth of the explored graph
- "max_loop_depth" limits the cumulative number of loop instances that can be unfolded in a given execution
- "max_node_number" limits the number of nodes in the explored graph

## Priorities

Whenever sibling frontier actions are evaluated, the order in which they are is, by default, the lexicographic order of their positions.
As a result, actions at the top of the diagram will be evaluated first during any exploration.

By setting priorities, we can modify the order in which frontier actions are evaluated. 
A priority takes the form of a signed integer value. A positive priority makes the action more prioritized and a negative one makes it less so.
In exploration, those priorities concern:
- actions within loops
- emissions
- receptions





