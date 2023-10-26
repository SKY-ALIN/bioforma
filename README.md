# 🧬 bioforma

Rust implementations of bioinformatics data structures and algorithms for Python.
Pyo3-based wrap of rust-bio and rust-bio-types packages.

![CI](https://github.com/SKY-ALIN/bioforma/actions/workflows/ci.yml/badge.svg)
[![license](https://img.shields.io/github/license/SKY-ALIN/bioforma.svg)](https://github.com/SKY-ALIN/bioforma/blob/main/LICENSE)

## Installation

Install using `pip install bioforma==0.1.0a0`.

Install from source using `make build & make install`.

## Examples

### One-way Open Reading Frame (ORF) Finder

```python
from bioforma.seq_analysis.orf import Finder

f = Finder(start=[b'ATG'], stop=[b'TGA', b'TAG', b'TAA'], min_len=5)
for orf in f.find_all(b'ATGGGGATGGGGGGATGGAAAAATAAGTAG'):
    print(repr(orf))

# Output:
# <Orf: start=14, end=26, offset=2>
# <Orf: start=0, end=30, offset=0>
# <Orf: start=6, end=30, offset=0>
```

### Pairwise Alignment

Calculate alignments with a generalized variant of the Smith Waterman algorithm.

```python
from bioforma.alignment import Alignment, Scoring, PairwiseAligner

x = b"ACCGTTGACGC"
y = b"CCGGCA"
scoring = Scoring.from_scores(gap_open=-5, gap_extend=-1, match_score=1, mismatch_score=-1)
aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
alignment: Alignment = aligner.calculate_semiglobal(x, y)

print(alignment.path())
# Output:
# [(1, 1, <Subst>), (2, 2, <Match>), (3, 2, <Ins>), (4, 2, <Ins>),
# (5, 2, <Ins>), (6, 2, <Ins>), (7, 2, <Ins>), (8, 2, <Ins>),
# (9, 3, <Subst>), (10, 4, <Match>), (11, 5, <Match>)]

print(alignment.pretty(x, y, ncol=100))
# Output:
# ACCGTTGACGC 
# \|++++++\|| 
# CC------GGCA

print(alignment.cigar(hard_clip=False))
# Output: 1X1=6I1X2=
```

### Rank Transform

Tools based on transforming the alphabet symbols to their lexicographical ranks.

```python
from bioforma.alphabets import Alphabet, RankTransform
from bioforma.alphabets.dna import make_dna_alphabet

a: Alphabet = make_dna_alphabet()
rt = RankTransform(a)

print(repr(rt))
# Output: <RankTransform: A-0, C-1, G-2, T-3, a-4, c-5, g-6, t-7>

print(rt.transform(b'aAcCgGtT'))
# Output: [4, 0, 5, 1, 6, 2, 7, 3]

print(rt.q_grams(2, b'ACGT'))
# Output: [1, 10, 19]
```

## Status

⚠️ In the active development phase. ⚠️

See [ROADMAP](https://github.com/SKY-ALIN/bioforma/blob/main/ROADMAP.md) for more.

---

Under [MIT Licence](https://github.com/SKY-ALIN/bioforma/blob/main/LICENSE).
