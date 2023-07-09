# 🧬 bioforma

Rust-bio wrap for Python. Use the speed of Rust with the Python's high-level for bioinformatics challenges.

![CI](https://github.com/SKY-ALIN/bioforma/actions/workflows/ci.yml/badge.svg)
[![license](https://img.shields.io/github/license/SKY-ALIN/bioforma.svg)](https://github.com/SKY-ALIN/bioforma/blob/main/LICENSE)

## Installation

Install using `pip install bioforma`.

## Status

⚠️ In the active development phase. ⚠️

Progress tree (15/92):

- [ ] alignment
  - [x] **distance.rs**
  - [x] pairwise
    - [ ] banded.rs
  - [ ] poa.rs
  - [ ] sparse.rs
- [x] **alphabets**
  - [x] **dna.rs**
  - [x] **protein.rs**
  - [x] **rna.rs**
- [ ] data_structures
  - [ ] annot_map.rs
  - [ ] bit_tree.rs
  - [ ] bitenc.rs
  - [ ] bwt.rs
  - [ ] fmindex.rs
  - [ ] interpolation_table.rs
  - [ ] interval_tree
    - [ ] array_backed_interval_tree.rs
    - [ ] avl_interval_tree.rs
  - [ ] qgram_index.rs
  - [ ] rank_select.rs
  - [ ] smallints.rs
  - [ ] suffix_array.rs
  - [ ] wavelet_matrix.rs
- [ ] io
  - [ ] bed.rs
  - [ ] fasta.rs
  - [ ] fastq.rs
  - [ ] gff.rs
  - [ ] newick.pest
  - [ ] newick.rs
- [ ] pattern_matching
  - [ ] bndm.rs
  - [ ] bom.rs
  - [ ] horspool.rs
  - [ ] kmp.rs
  - [ ] myers
    - [ ] builder.rs
    - [ ] common_tests.rs
    - [ ] helpers.rs
    - [ ] long.rs
    - [ ] myers_impl.rs
    - [ ] simple.rs
    - [ ] traceback.rs
  - [ ] pssm
    - [ ] dnamotif.rs
    - [ ] errors.rs
    - [ ] protmotif.rs
  - [ ] shift_and.rs
  - [ ] ukkonen.rs
- [x] **scores**
  - [x] **blosum62.rs**
  - [x] **pam120.rs**
  - [x] **pam200.rs**
  - [x] **pam250.rs**
  - [x] **pam40.rs**
- [x] **seq_analysis**
  - [x] **gc.rs**
  - [x] **orf.rs**
- [ ] stats
  - [ ] bayesian
    - [ ] bayes_factors.rs
    - [ ] model.rs
  - [ ] combinatorics.rs
  - [ ] hmm
    - [ ] errors.rs
  - [ ] pairhmm
    - [ ] homopolypairhmm.rs
    - [ ] pairhmm.rs
  - [ ] probs
    - [ ] adaptive_integration.rs
    - [ ] cdf.rs
    - [ ] errors.rs
- [ ] utils
  - [ ] fastexp.rs
  - [ ] interval
    - [ ] errors.rs
  - [ ] text.rs
- [ ] types (rust-bio-types)
  - [x] **alignment.rs**
  - [ ] annot
    - [ ] contig.rs
    - [ ] loc.rs
    - [ ] pos.rs
    - [ ] refids.rs
    - [ ] spliced.rs
  - [ ] genome.rs
  - [ ] phylogeny.rs
  - [ ] sequence.rs
  - [ ] strand.rs
  - [ ] variant.rs

---

Under [MIT Licence](https://github.com/SKY-ALIN/bioforma/blob/main/LICENSE).
