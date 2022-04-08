

## Process options


The "@analyze_option" section specifies options to be used when the model is exploited
as a reference for the analysis of a multi-trace with the "analyze" command.

We will detail the available options when describing the "analyze" and "explore" commands.


# Entry language : traces & multi-traces (.htf files)









# Command Line Interface



## Help

The HIBOU executable provides a small documentation about its interface. This can be accessed by typing "hibou_label help" or "hibou_label -h".
It explaines that there are 4 sub-commands in HIBOU:
- "draw" (to be used as "hibou_label draw <.hsf file>"), which draws as a sequence diagram a given interaction
- "analyze" (to be used as "hibou_label analyze <.hsf file> <.htf file>"), which analyze a multi-trace w.r.t. an interaction
- "explore" (to be used as "hibou_label explore <.hsf file>"), which computes (partially or totally if possible) the semantics of a given interaction
- "help", which is the present help

Each sub-command also has its dedicated documentation:
- "hibou_label draw -h" provides a small documentation for the drawing utility
- "hibou_label analyze -h" provides a small documentation for the "analyze" sub-command
- "hibou_label explore -h" provides a small documentation for the "explore" sub-command

In the following, we provide more details about those sub-commands.

## Draw

Diagrams, such as the one on the previous images can be drawn using the "hibou draw" command as exemplified below.

<img src="./README_images/draw_command_building_blocks.png" alt="draw command" width="450">

<img src="./README_images/building_blocks.png" alt="building blocks" width="500">


## Analyze - Introduction

The "analyze" sub-command of HIBOU can analyse multi-traces w.r.t. interactions.
For any multi-trace and any interaction, it returns a verdict about the conformity of the multi-trace w.r.t.
a certain semantics of the interaction.

Indeed, one can consider several semantics (sets of multi-traces specified by a given interaction) when dealing with interaction languages.
In our approach, we distinguish between:
- "AccMult", the semantics which is that of exactly accepted multi-traces (projections of accepted global traces)
- "SemMult", the semantics which is that of multi-traces which are obtained from projecting prefixes of accepted global traces
- "MultiPref", that of prefixes (in the sense of multi-traces) of multi-traces from "AccMult" (i.e. one can remove actions at the end of each
local component of the multi-trace independently)
- "Slices", that of slices of multi-traces from "AccMult" (i.e. one can remove actions at both the end and the beginning
of each local component of the multi-trace independently)

You may have guessed that for any given interaction, its associated semantics (using those 4 semantics) form a set of
russian matryoshka dolls i.e. they are included in one another as follows:
AccMult is in SemMult which is in MultiPref, itself in Slices.

In hibou_label, we propose several distinct methods (abusively called "semantics") to identify if a given multi-trace is in
any one of those sets w.r.t. a given interaction.
The method used is specified using the "semantics" attribute ("prefix" is the default) in the "@analysis_option"
section of the input ".hsf" file.

The proposed methods are:
- ``semantics=accept``, which returns a verdict "Pass" if the multi-trace is in "AccMult" or "Fail" if not
- ``semantics=prefix``, which returns "Pass" if the multi-trace is in "AccMult", "WeakPass" if it is in "SemMult" but not in "AccMult",
and either "Inconc" or "Fail" if not, as explained in Sec.4.5 of [this paper](https://arxiv.org/abs/2009.01777)
- ``semantics=hide``, which returns "Pass" if the multi-trace is in "AccMult", "WeakPass" if it is in "SemMult" but not in "AccMult",
either "WeakPass" or "Inconc" if it is in "MultiPref" but not in "SemMult" (WeakPass if all local components of the
multi-trace are defined on a single lifeline i.e. co-localizations are singletons), and "Fail" if none of the above. This method uses
a specific "hide" function that we will detail in an upcoming paper.
- ``semantics=simulate[multi-prefix]``, which returns "Pass" if the multi-trace is in "AccMult", "WeakPass" if it is in "MultiPref"
but not in "AccMult", and "Fail" if none of the above. This method simulate the execution of the actions that are missing at the end
of the multi-trace's components so as to complete it (if possible) into a multi-trace from "SemMult".
- ``semantics=simulate[multi-slice]``, which returns "Pass" if the multi-trace is in "AccMult", "WeakPass" if it is in "Slices"
but not in "AccMult", and "Fail" if none of the above. This method simulate the execution of the actions that are missing
at the beginning and/or at end of the multi-trace's components so as to complete it (if possible) into a multi-trace from "SemMult".

We have proven in [this Coq proof](https://erwanm974.github.io/coq_hibou_label_multi_trace_analysis/) that the verdict "Pass"
is equivalent to the membership of the multi-trace to the "AccMult" semantics of the interaction.

### Analysing multi-traces

Our approach to analysis consists in consuming one-by-one the head elements of the multi-trace
i.e. the actions which are at the beginning of its trace components.

To do so, we compare them with the actions that are immediately executable within the initial interaction model.
If there is a match between such an action - called a frontier action - and a trace action,
we can compute another interaction model which describes what can happen in the remainder of the execution.
We then repeat the process with this new interaction model and the new multi-trace on which we have removed the consumed action.

Consequently, any given analysis opens-up paths which are successions of couples (interaction,multi-trace).
Each such path terminates either with:
- in ``accept`` mode, either:
  + a "Cov" local verdict, when the multi-trace has been entirely consumed and the interaction can express the empty execution (statically verified on the interaction term)
  + or an "UnCov" local verdict in the other cases (i.e. either when the consumption of the multi-trace is impossible, or when the multi-trace has been emptied but the interaction cannot express the empty execution)
- in ``prefix`` mode either:
  + a "Cov" local verdict, when the multi-trace has been entirely consumed and the interaction can express the empty execution
  + a "TooShort" local verdict, when the multi-trace has been entirely consumed but the interaction cannot express the empty execution
  + an "Out" local verdict, when no component of the multi-trace can be entirely consumed
  + a "LackObs" local verdict, when some but not all of the components of the multi-trace can be entirely consumed
- in ``hide`` mode either:
  + a "Cov" local verdict, when the multi-trace has been entirely consumed without having to take a hiding step
    and the interaction can express the empty execution
  + a "TooShort" local verdict, when the multi-trace has been entirely consumed, no hiding step has been taken,
    but the interaction cannot express the empty execution
  + a "MultiPref" or "Inconc" local verdict, when the multi-trace has been entirely consumed
    but in the path corresponding to this consumption, there was a hiding step
    (as explained, we have "MultiPref" if the multi-trace is defined on canals that are singletons and "Inconc" if not)
  + an "Out" local verdict, when the multi-trace cannot be entirely consumed
- in ``simulate`` mode either:
  + a "Cov" local verdict, when the multi-trace has been entirely consumed without any simulation step
    and the interaction can express the empty execution
  + a "TooShort" local verdict, when the multi-trace has been entirely consumed, no simulation step has been taken,
    but the interaction cannot express the empty execution
  + a "MultiPref" local verdict, when the multi-trace has been entirely consumed and in the path
    corresponding to this consumption, there were simulation steps but only corresponding
    to missing actions at the end of local components (ceased local observation too early)
  + a "Slice" local verdict, when the multi-trace has been entirely consumed and in the path
    corresponding to this consumption, there were simulation steps corresponding
    to missing actions at the beginning of local components (started local observation too late)
    Let us note that simulation step at the beginning of local components is only enabled if ``semantics=simulate[multi-slice]``
    and not if ``semantics=simulate[multi-prefix]``
  + an "Out" local verdict, when the multi-trace cannot be entirely consumed

From those local verdicts, the global verdict is inferred:
- in "accept" mode:
  + "Pass" is returned if there exists a path terminating in "Cov"
  + "Fail" is returned otherwise
- in "prefix" mode:
  + "Pass" is returned if there exists a path terminating in "Cov"
  + "WeakPass" is returned if there are no paths leading to "Cov" but there exist one terminating in "TooShort"
  + "Inconc" is returned if there are no paths leading to either "Cov" or "TooShort" but there is one leading to "LackObs"
  + "Fail" is returned otherwise
- in "hide" mode:
  + "Pass" is returned if there exists a path terminating in "Cov"
  + "WeakPass" is returned if there are no paths leading to "Cov" but there exist one terminating in "TooShort" or "MultiPref"
  + "Inconc" is returned if there are no paths leading to either "Cov", "TooShort" or "MultiPref" but there is one leading to "Inconc"
  + "Fail" is returned otherwise
- in "simulate" mode:
  + "Pass" is returned if there exists a path terminating in "Cov"
  + "WeakPass" is returned if there are no paths leading to "Cov" but there exist one terminating in "TooShort" or "MultiPref" or "Slice"
  + "Fail" is returned otherwise


## Analyze - "accept" & "prefix" modes


### Principle

As hinted at earlier, and as illustrated below, the basic principle of trace analysis with hibou_label is to consume the input trace
while executing the consumed actions in the interaction model (if possible).
This is done repeatedly until the input trace is emptied. Once this is the case, a verdict can be produced (as detailed earlier).

In the example below, we analyze a global trace and there is a single path that is explored during the analysis before reaching a "Cov" verdict,
signifying that the input trace is an accepted trace.

<img src="./README_images/analysis_accept_principle_1.svg" alt="principle of analysis with accept semantics 1" width="1000">

Of course, during any analysis, several paths can open-up. It is often the case when analysing multi-traces, which may allow many possible
interleavings between distributed events, but it can also appear when analysing global traces (as is the case below), when there are several
instances of a same action that match the head action of the trace in the interaction term.

As illustrated below, in certain cases we may not be able to consume the trace entirely. Here, the action "l2?m2" cannot be consumed in any of the 2 paths.
In this case, the algorithm detected a non-conformity and returns an "UnCov" verdict (with ``semantics = accept`` mode).

<img src="./README_images/analysis_accept_principle_2.svg" alt="principle of analysis with accept semantics 2" width="850">

### Example 1 (introducing analysis options & basic search strategies)

Below is given an example analysis, that you can reproduce by using the files from the "examples" folder.

<img src="./README_images/analysis_command_1.png" alt="analysis command ex1" width="600">

The analysis of the multi-trace specified in the "mutrace.htf" file against the interaction specified in
the "example_for_analysis.hsf" file yields the "Pass" global verdict.

For this analysis we used the following options, declared in the "@analyze_option" section of "example_for_analysis.hsf".

```
@analyze_option{
    loggers = [graphic[svg,vertical]];
    semantics = prefix;
    strategy = DepthFS;
    use_locfront = false;
    goal = Pass
}
```



As explained earlier, we will use the "prefix" semantics to analyze the multi-trace.
Here it is specified with ``semantics = prefix`` (default value is ``prefix``).



So as to reduce the search space there is an option to eliminate certain nodes of the execution tree
by making use of "local frontiers". We will discuss this in another example. Here we deactivated this
feature with ``use_locfront = false``. By default it is activated.

We can also specify the verdict which will be the goal of the analysis, meaning that the analysis will stop once a verdict that is greater or equal to the goal
is found. This is done using the "goal" option.
For instance, if we set the goal to "Pass" then, the analysis will stop either when a "Cov" is found, or when all paths have been exploited.
If we set the goal "WeakPass", it will then suffice to find either a "Cov" or a "TooShort" (or "MultiPref" or "Slice" in some semantics).
In this example we used ``goal = Pass``. By default the goal is "Pass".

The options specified here allowed us, in the case of this example, to quickly find a path that consumed the entire
multi-trace and we did not need to explore further executions of the initial interaction model.
For instance, you can see on the image below (generated by the "graphic" logger)
that we did not explore the branch starting with the execution and consumption of "c!m4".

<img src="./README_images/ana_mutrace_dfs_none.svg" alt="analysis ex1 with dfs" width="750">

Let us note that we would have had the following if we had used the Breadth First Search strategy:

<img src="./README_images/ana_mutrace_bfs_none.svg" alt="analysis ex1 with bfs" width="900">

### Example 2 (global trace and "WeakPass" as goal)

Below is another example, this time of the analysis of a global trace, and yielding the "WeakPass" verdict.
You can also reproduce it by using the files from the "examples" folder.

<img src="./README_images/analysis_command_2.png" alt="analysis command ex2" width="600">

<img src="./README_images/ana_glotrace.svg" alt="analysis ex2" width="800">

Let us note that, if we change the goal to "WeakPass" in the options, the branch on the right is not explored in the analysis because the analysis stops
once the goal verdict (or a verdict that is stronger) is reached.

<img src="./README_images/ana_glotrace_with_weakpass_as_goal.svg" alt="analysis ex2 with weakpass as goal" width="600">

### Example 5 (showcasing verification of local frontiers)

As we hinted earlier, we have developed a manner to reduce the size of the search space of analysis.
This consists in preventing useless explorations of some branches of the execution tree that are destined to yield an "Out"/"UnCov" verdict.
Let us first see this optional feature in action.
Below we have an analysis without the feature activated i.e. with ``use_locfront = false``.

<img src="./README_images/example_locfront_false.svg" alt="example showcasing locfront set at false" width="1600">

Then, we have below the same analysis (same interaction and same multi-trace to analyze) but with the feature activated
i.e. with ``use_locfront = true``.

<img src="./README_images/example_locfront_true.svg" alt="example showcasing locfront set at true" width="800">

We can see that far less nodes are explored in the analysis when the option ``use_locfront`` is set to ``true``.
In facts, in this example we have 2 nested loops; an outer loop and an inner loop.
The multi-trace that is analysed in both cases illustrated above is the following:

```
{
    [l1] l1!m1.l1!m1;
    [l2] l2?m1.l2?m1.l2?m2.l2?m2;
    [l3] l3!m2.l3!m2
}
```

We have 2 repetitions of the passing of message "m1" (specified in the outer loop)
followed by 2 repetitions of the passing of message "m2" (specified in the inner loop).

When the outer loop have been instanciated once (third node from the top in both cases), we have, in the interaction term
two occurrences of the inner loop. One that is above (outside the outer loop), and one that is below (inside the outer loop).
When the algorithm want to consume "l3!m2", it can interpret it as instanciating the inner loop that is above, or the one that is below (both have "l3!m2" in the frontier).
However if we instanciate the inner loop that is above by executing "l3!m2",
then the subsequent reception "l2?m2" must occur on lifeline "l2" immediately after (no other action can be interleaved)
this is not the case in the trace, as we have, on lifeline "l2", 1 other reception of "m1" before receiving "m2".
As a result, exploring this branch is fruitless but the bare algorithm (from [this paper](https://arxiv.org/abs/2009.01777))
does not have the tools to realize this. Therefore it keeps exploring while consuming the multi-trace until it cannot do so anymore.

However, in the case where ``use_locfront = true``, we can see that the exploration is stopped immediately in the left child of the third node from the top, with a "Dead" verdict.
This "Dead" verdict corresponds to a mismatch that have been detected when verifying the local frontiers (which we will detail in an upcoming paper).
Here, the local frontier of lifeline "l2" contains only "l2?m2" and hence we cannot match "l2?m1" which is the head action of the corresponding component trace of the multi-trace.

The gain of using ``use_locfront = true`` (in terms of number of nodes that are explored) can be considerable in certain cases (depending on the interaction models).
In this precise example, let us consider the numbers of repetitions of the inner and outer loops.
Our example from above had 2 repetitions of the outer loop followed by 2 repetitions of the inner loop (no interleavings).
We will note it "2o2i" for 2 outer 2 inner.
In the table below we also consider analysis for multi-traces corresponding to 3, 4 and 5 repetitions of the outer loops, followed by 2 repetitions of the inner loop
(denoted 3o2i, ..., 5o2i).
In the first row is the number of nodes required to reach the verdict without the ``use_locfront`` option, and in the second row with the option.

|                            | 2o2i | 3o2i | 4o2i | 5o2i |
|----------------------------|------|------|------|------|
| DepthFS without locfront   |   22 |   55 |  116 |  217 |
| DepthFS with locfront      |   11 |   15 |   19 |   23 |




## Analyze - "hide" mode

TODO: write README section

## Analyze - "simulate" mode

TODO: write README section



## Canonize

A process to obtain normal forms of interaction terms.
Three phases of rewriting are successively applied (only once / one time each).
The remaining interaction term is a simplified, compact form, which have the same semantics as the original.

### Remarks

This process works for interaction terms without co-regions.

Patterns of message emissions and broadcasts are flattened i.e.
```
a -- m -> (b,c)
```
becomes
```
strict(
    a -- m ->|,
    seq(
        m -> b,
        m -> c
    )
)
```

