
# Interaction models and their encoding

Interaction models are specified within ".hsf" (Hibou Specification File) files.
An example interaction diagram (as drawn by HIBOU) is given below.

<img src="./readme/images/1/ex_readme.png" alt="interaction diagram" width="750">

This diagram is encoded as follows.

```
seq(
	loopW(
		alt(
			l1 -- m1 -> l2,
			l2 -- m1 -> l3 
		)
	),
	loopP(
		seq(
			l5 -- m2 -> l4,
			alt(
				l4 -- m3 -> l5,
				l4 -- m4 -> l5 
			)
		)
	),
	l5 -- <synch>m5 -> l3,
    l3 -- m6 -> l2,
    coreg(l4)(
		l2 -- m7 -> (l4,l5),
    	loopH(
			l2 -- m8 -> l4 
		)
	),
	l1 -- <synch>m9 -> (l4,l5)
)
```




## Signature Declaration

The signature of the interaction model is declared in the "@message" and "@lifeline" sections of the ".hsf" file.
It suffices then to list the different message names and lifeline names that will be used in the model.

For instance,
in the example above
(which you can find in [examples/1/ex_readme.hsf](https://github.com/erwanM974/hibou_label/blob/master/examples/1/ex_readme.hsf)),
we have the following:

```
@message{m1;m2;m3;m4;m5;m6;m7;m8;m9}
@lifeline{l1;l2;l3;l4;l5}
```

## Interaction Term

Interactions are terms of a formal language, that can be specified using a simple and intuitive inductive language.
Those terms are build inductively from the composition of basic buildings blocks using specific operators.

### Basic building blocks

<img src="./readme/images/1/basic_blocks.png" alt="building blocks" width="600">

The most basic interactions can either be:
- the empty interaction, that specify an absence of observable behavior and which is encoded with "o" or "∅"
- the reception of a message "m" on a lifeline "a" from the environment, which is encoded with "m -> a"
- the emission of a message "m" from lifeline "a" to the environment, which is encoded using "a -- m ->|"
- the asynchronous passing of a message "m" from lifeline "a" to lifeline "b", which is encoded with "a -- m -> b" (here message "m" is passed asynchronously from lifeline "a" to "b")
- an asynchronous broadcast, which can be encoded with "a -- m -> (b,c,...)"
- asynchronous receptions of the same message "m" by several lifelines as "m -> (a,b,...)"
- synchronous variants of the three previous items can be encoded using a "<synch>" keyword. Those then correspond to synchronized multi-rendez-vous between lifelines.

The diagram above is hence encoded as follows:

```
@message{reception;emission;m}
@lifeline{lifeline1;lifeline2;lifeline3}
seq(
	reception -> lifeline2,
	lifeline2 -- emission ->|,
	lifeline1 -- m -> lifeline2,
	lifeline1 -- <synch>m -> lifeline2,
	lifeline1 -- m -> (lifeline2,lifeline3),
	lifeline1 -- <synch>m -> (lifeline2,lifeline3),
	m -> (lifeline1,lifeline2),
	<synch>m -> (lifeline1,lifeline2)
)
```



### Operators

Interaction terms can be composed inductively using some operators so as to build more complex interaction terms.
We define the following operators:
- "strict", "seq" and "par", which are the classical "scheduling operators" and "coreg", which is another special "scheduling operator"
- "alt" which is the "alternative operator"
- "loopS", "loopH", "loopW" and "loopP" which are "repetition operators"

#### Scheduling operators
Scheduling operators specify how the execution of different sub-interactions can be scheduled w.r.t. one another.
Given sub-interactions "i1", "i2", ..., and "in", and given a scheduling operator "f",
we encode using "f(i1,i2,...,in)" the interaction which results from the scheduling with "f" of those sub-interactions.
We allow n-ary expressions in the entry language but the interaction term is constructed as a binary tree in hibou.

- the "strict" operator specifies strict sequencing i.e. preceding sub-interactions must be executed entirely before any following interaction can be
- the "seq" operator specifies weak sequencing i.e. a strict scheduling is only enforced between actions occurring on the same lifeline
- the "par" operator allow any interleaving of the executions of sub-interactions
- "coreg(PL)" operators can be configured by a subset PL of lifelines which behaviors are to be parallelized.
  "coreg(PL)" then behaves as "seq" for lifelines not in PL, and as "par" for lifelines which belong to PL

#### Alternative operator

The "alt" operator specified exclusive alternative choice between the execution of sub-interactions. As for the scheduling operators, we allow n-ary expressions.

#### Repetition operators

Loop operators are unary operators that represent the repetition of a given sub-interaction,
each repeated behavior being sequenced w.r.t. the others using a scheduling operator.

For instance, given "i" an interaction:
- "loopS(i)" is equivalent to the infinite alternative "alt(∅,i,strict(i,i),...)"
- "loopW(i)" is equivalent to the infinite alternative "alt(∅,i,seq(i,i),...)"
- "loopP(i)" is equivalent to the infinite alternative "alt(∅,i,par(i,i),...)"

"loopH" is a specific restriction of "loopW" which we documented in
"[A structural operational semantics for interactions with a look at loops](https://arxiv.org/abs/2105.00208)".