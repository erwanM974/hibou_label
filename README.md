
<img src="./readme/images/hibou_banner_v2.svg" alt="hibou banner" width="750">

# HIBOU

HIBOU (for Holistic Interaction Behavioral Oracle Utility) provides utilities for
designing, drawing & manipulating interaction models, explore their semantics
and analyse outputs of distributed systems (sets of distributed logs)
with regards to formal specifications written as interaction models.

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

We described our approach in the following papers: 
- "[Revisiting Semantics of Interactions for Trace Validity Analysis](https://link.springer.com/chapter/10.1007%2F978-3-030-45234-6_24)"
- "[A small-step approach to multi-trace checking against interactions](https://dl.acm.org/doi/abs/10.1145/3412841.3442054)"

## Coq proofs

The theoretical background of this present tool has been checked with some automated proofs written in Coq:
- [proof of correctness for the semantics](https://erwanm974.github.io/coq_hibou_label_semantics_equivalence/)
- [proof of correctness for the multi-trace analysis algorithm](https://erwanm974.github.io/coq_hibou_label_multi_trace_analysis/)

# Documentation

The README can be accessed 
[here](https://github.com/erwanM974/hibou_label/blob/master/readme/0_main.md).

