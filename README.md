
<img src="./readme/images/hibou_banner_v2.svg" alt="hibou banner" width="750">

# HIBOU

HIBOU (for Holistic Interaction Behavioral Oracle Utility) provides utilities for
designing, drawing & manipulating interaction models, explore their semantics
and analyse outputs of distributed systems (sets of distributed logs)
with regard to formal specifications written as interaction models.

This present version "hibou_label" treats labelled interaction models.
A fork "[hibou_efm](https://github.com/erwanM974/hibou_efm)" that treats interaction
models enriched with data and time is also available.

This piece of software has been developed as part of my PhD thesis in 2018-2021 at the 
[CentraleSupelec](https://www.centralesupelec.fr/)
engineering school
(part of [Université Paris-Saclay](https://www.universite-paris-saclay.fr/))
in collaboration with the 
[CEA](http://www.cea.fr/) (Commissariat à l'énergie atomique et aux énergies alternatives).

## Publications 

Associated publications (in chronological order): 
- "[Revisiting Semantics of Interactions for Trace Validity Analysis](https://link.springer.com/chapter/10.1007%2F978-3-030-45234-6_24)"
- "[A small-step approach to multi-trace checking against interactions](https://dl.acm.org/doi/abs/10.1145/3412841.3442054)"
- "[Equivalence of Denotational and Operational Semantics for Interaction Languages](https://link.springer.com/chapter/10.1007/978-3-031-10363-6_8)"
- "[Interaction-based Offline Runtime Verification of Distributed Systems](http://fsen.ir/2023/files/FSEN-Preproceedings.pdf)" (best paper award)


## Coq proofs

The theoretical background of this present tool has been checked with some automated proofs written in Coq:
- [equivalence of three semantics for the base language](https://erwanm974.github.io/coq_hibou_label_semantics_equivalence/)
- [syntactic equivalence relation between semantically equivalent interactions terms](https://erwanm974.github.io/coq_hibou_label_equivalent_terms/)
- [proof of correctness for the multi-trace analysis algorithm](https://erwanm974.github.io/coq_hibou_label_multi_trace_analysis/)
- [equivalence of two semantics for a language extended with concurrent regions](https://erwanm974.github.io/coq_interaction_semantics_equivalence_with_coregions)


## Experiments

Experiments on various features of the tools are publicly available:
- [solving 3SAT problems wth HIBOU](https://github.com/erwanM974/hibou_3sat_benchmark_experiment)
- [using a lifeline-removal operation to analyze partially observed multi-traces](https://github.com/erwanM974/hibou_hiding_usecases)
- [using simulation steps to analyze partially observed multi-traces](https://github.com/erwanM974/hibou_simulation_usecases_for_slice_recognition)
- [generating NFA monitors from interactions](https://github.com/erwanM974/hibou_nfa_transformation_usecases_for_monitoring)




# Documentation

A README (not up-to-date) can be accessed 
[here](https://github.com/erwanM974/hibou_label/blob/master/readme/0_main.md).

