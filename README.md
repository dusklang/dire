# DIRE HAS BEEN MOVED
This repository is kept around for posterity. Future development on Dire will take place in [dusklang/dusk](https://github.com/dusklang/dusk). The final commit in Dusk that uses this separate repository is [5c74fb2](https://github.com/dusklang/dusk/commit/5c74fb2bacdf166d8e7fa7c02ce703a815f62ea1).

# Dire: Dusk Intermediate REpresentation
This is (or will be) an intermediate representation library inspired by [MLIR](https://mlir.llvm.org/), primarily for use in the Dusk programming language.

Goals:
- Fast: to generate, and to execute.
- Extensible: Dusk programmers should be able to define new dialects (in MLIR parlance) to open up new capabilities and/or optimizations.
- Flexible: should be able to represent everything from super high-level code, all the way down to assembly
