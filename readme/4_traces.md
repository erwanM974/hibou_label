
# The notions of trace and multi-trace

When a distributed system is executed, we can collect logs on the external interfaces of its sub-system so as to have a
trace of the events that were observed during the execution. We consider two kinds of events that may occur on the interface
of sub-systems: emissions and receptions of messages.

As the lifelines of an interaction model represent the sub-systems of the distributed system modelled by the interaction,
we can understand those collected logs - called traces - as sequences of words
"l!m" or "l?m" - called actions - where "l" is a lifeline and "m" a message.
An action "l!m" corresponds to the emission of message "m" from lifeline "l"
(disregarding the target of said emission).
An action "l?m" corresponds to the reception of message "m" on lifeline "l"
(disregarding the source of the message).

## Centralized traces

A centralized trace is a sequence of any such action regardless of the lifelines on which they occur.
Below is given, in the syntax accepted by HIBOU, such a centralized trace.
This trace must be specified in a ".htf" file, which stands for Hibou Trace File.
Here, the "[#all]" signifies that the trace is defined over all lifelines.

```
[#all] a!m1.b?m1.b!m2.c?m2
```

In [this paper](https://link.springer.com/chapter/10.1007%2F978-3-030-45234-6_24), we describe our approach for the analysis of centralized traces.

## Multi-traces

However, it is not always possible to collect such centralized traces.
More often that not, we have a local log/trace for each sub-system of the distributed system.
As such, we use the notion of multi-trace.

Multi-traces are sets of traces called its components.
Each component is defined over a subset of lifelines called a co-localization that is disjoint to that of any other component.

In [this other paper](https://dl.acm.org/doi/abs/10.1145/3412841.3442054), we describe our approach for the analysis of multi-traces.
In it, we restricted co-localizations to singletons in order to simplify the presentation.
However, our approach still works when considering the more general case of co-localizations.

On the example below is given an example of ".htf" file which defines a multi-trace composed of 2 components:
- on the co-localization of the 2 lifelines "a" and "b", the local trace "b!m.a!m" has been logged
- on the localization of the "c" lifeline, the local trace "c?m.c?m" has been logged

```
{
    [a,b] b!m.a!m;
    [c]   c?m.c?m
}
```

Let us note that we can also analyze global traces simply by defining a multi-trace with a single component as is done below.
Here we used the "#all" keyword to state that this component is defined over all the lifelines defined in "@lifeline".

```
{
    [#all] a!m1.b?m1.b!m2.c?m2
}
```

We can also use the "#any" keyword to state that a given multi-trace component is defined over all the lifelines that appear in the subsequent trace definition.
For example below is defined a multi-trace that is the same than the one in our previous multi-trace example.
The first component is defined over lifelines "a" and "b", and the second over lifeline "c".

```
{
    [#any] b!m.a!m;
    [#any] c?m.c?m
}
```

## Lifelines with no specified trace component

If, in a given ".htf" file, traces on some lifelines are not specified, the corresponding lifelines are automatically fitted with the empty trace.
As such, if the set of lifelines is "L={a,b}", we can specify the multi-trace "(a!m.a!m, ε)" in a ".htf" file either with:

```
{
    [a] a!m.a!m;
    [b]
}
```

Or simply with (for instance):

```
[#any] a!m.a!m
```

Here lifeline "b" has no specified canal. As such, it will be automatically fitted with a dedicated canal containing the empty trace "ε".