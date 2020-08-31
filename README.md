# HIBOU ICTSS 2020

This piece of software has been developped as part of a PhD thesis in 2018-2020.
This particular build has been prepared to accompany the publication of a paper in the 2020 edition of the ICTSS
conference (32ND IFIP INTERNATIONAL CONFERENCE ON TESTING SOFTWARE AND SYSTEMS.

HIBOU (for Holistic Interaction Behavioral Oracle Utility) provides utilities for the analysis of traces and 
multi-traces collected from the execution of Distributed Systems against interaction models.

## Entry language

Interaction models are specified with .hsf files.
The figure below illustrates:
- on the left the model of the interaction as a binary tree (mathematical model)
- in the middle the encoding using the entry langage of HIBOU (PEG grammar)
- on the right the resulting sequence diagram as drawn by HIBOU  

![image info](./README_entry_schema.png)


## Exploration

HIBOU can generate execution trees which illustrate the semantics of the a given interaction model.
The exploration of such execution trees can be defined up to certain limits (depth, number of nodes, loop 
instanciations).

![image info](./README_explo1.png)

![image info](./README_explo2.png)

## Analysis

HIBOU can analyse traces or multitraces (defined with any groups of co-localized lifelines).

![image info](./README_ana1.png)