
# The execution and animation of interaction models

The main features of HIBOU revolve around the "execution" or "animation" of interaction models.
By execution we mean something which consists in executing an atomic action within an initial interaction model, thereby observing its occurrence.
The execution of such an action yields a new "follow-up" interaction model,
which specifies all the continuations of the behavior of the original interaction, which start by the occurence of the executed action.

This approach is detailed in
[this paper](https://link.springer.com/chapter/10.1007%2F978-3-030-45234-6_24) and [this one](https://dl.acm.org/doi/abs/10.1145/3412841.3442054).

In short, this consists in:
- identifying which atomic actions are immediately executable (frontier actions)
- when one such action is executed, computing, through the rewriting of the interaction term, the "follow-up" interaction

This is illustrated in the example below, where, for a given initial interaction, we represent all the atomic actions that can be
immediately executed within it, and, for each of those, the corresponding "follow-up" interactions.
This process can of course be repeated in a next step, making possible the computation of the expected behaviors (semantics)
of the initial interaction.

<img src="./images/2/interaction_execution_principle.svg" alt="interaction execution principle" width="1000">

In the following, we will give some insights on the different elements of our language via the execution of interaction models.


## Basic building blocks

Execution of an atomic emission:

<img src="./images/2/basic/emission.svg" alt="execution of an emission" width="150">

Execution of an atomic reception:

<img src="./images/2/basic/reception.svg" alt="execution of a reception" width="150">

Execution of an asynchronous message passing:

<img src="./images/2/basic/asynch_passing.svg" alt="execution of an asynchronous message passing" width="150">

Execution of a synchronous message passing:

<img src="./images/2/basic/synch_passing.svg" alt="execution of a synchronous message passing" width="150">

Execution of an asynchronous broadcast:

<img src="./images/2/basic/asynch_broadcast.svg" alt="execution of an asynchronous broadcast" width="275">

Execution of a synchronous broadcast:

<img src="./images/2/basic/synch_broadcast.svg" alt="execution of a synchronous broadcast" width="150">



## Alt

<img src="./images/2/alt.svg" alt="execution of actions within an alternative" width="275">


## Strict, Seq and Par

### Actions on the same lifeline(s)

With strict sequencing:

<img src="./images/2/sched/strict_same.svg" alt="" width="150">

With weak sequencing:

<img src="./images/2/sched/seq_same.svg" alt="" width="150">

With interleaving:

<img src="./images/2/sched/par_same.svg" alt="" width="275">

### Actions on different lifelines

With strict sequencing:

<img src="./images/2/sched/strict_diff.svg" alt="" width="150">

With weak sequencing:

<img src="./images/2/sched/seq_diff.svg" alt="" width="275">

With interleaving:

<img src="./images/2/sched/par_diff.svg" alt="" width="275">


## Coreg

As explained previously, a coretion behaves as par on certain lifelines and as seq on the others.
Here, we can see that:
- l1 must emit m1 before it can emit m2 because the coregion behaves as seq on l1 
- and l2 can receive m1 and m2 in any order because the coregion behaves as par on l2 

<img src="./images/2/coreg.svg" alt="" width="275">

## Pruning

The interplay between alternatives and sequencing makes so that the execution of certain actions
implicitly imply making certain choices on alternatives. 
This must be taken into account when computing the corresponding follow-up interaction.

<img src="./images/2/pruning/pruning_strict.svg" alt="" width="275">

Another example with a coregion:

<img src="./images/2/pruning/pruning_coreg.svg" alt="" width="350">




## Loops

Loops specify repetitions of behavior.
Please refer to "[A structural operational semantics for interactions with a look at loops](https://arxiv.org/abs/2105.00208)" for further details.

Below we demonstrate the difference between loopH and loopW on an example:

Here with loopH:

<img src="./images/2/loop/loop_withH.svg" alt="" width="550">

Here with loopW:

<img src="./images/2/loop/loop_withW.svg" alt="" width="650">



