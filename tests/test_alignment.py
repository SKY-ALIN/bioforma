from bioforma.alignment import (
    Alignment,
    AlignmentOperation,
    Match,
    Subst,
    Del,
    Ins,
    Xclip,
    Yclip,
)
from bioforma.alignment.distance import (
    hamming,
    simd_hamming,
    levenshtein,
    simd_levenshtein,
    simd_bounded_levenshtein,
)


def test_hamming():
    assert hamming(b'GTCTGCATGCG', b'TTTAGCTAGCG') == 5
    try:
        hamming(b'GACTATATCGA', b'TTTAGCTC')
    except ValueError:
        assert True
    else:
        assert False


def test_simd_hamming():
    assert simd_hamming(b'GTCTGCATGCG', b'TTTAGCTAGCG') == 5
    try:
        simd_hamming(b'GACTATATCGA', b'TTTAGCTC')
    except ValueError:
        assert True
    else:
        assert False


def test_levenshtein():
    x = b'ACCGTGGAT'
    y = b'AAAAACCGTTGAT'
    assert levenshtein(x, y) == 5
    assert levenshtein(x, y) == levenshtein(y, x)
    assert levenshtein(b'AAA', b'TTTT') == 4


def test_simd_levenshtein():
    x = b'ACCGTGGAT'
    y = b'AAAAACCGTTGAT'
    assert simd_levenshtein(x, y) == 5
    assert simd_levenshtein(x, y) == simd_levenshtein(y, x)
    assert simd_levenshtein(b'AAA', b'TTTT') == 4


def test_simd_bounded_levenshtein():
    x = b'ACCGTGGAT'
    y = b'AAAAACCGTTGAT'
    max_value = 4_294_967_295
    assert simd_bounded_levenshtein(x, y, max_value) == 5
    assert simd_bounded_levenshtein(x, y, max_value) == simd_bounded_levenshtein(y, x, max_value)
    assert simd_bounded_levenshtein(b'AAA', b'TTTT', max_value) == 4
    assert simd_bounded_levenshtein(x, y, 5) == 5
    assert simd_bounded_levenshtein(x, y, 4) is None


def test_alignment_operations():
    assert issubclass(Match, AlignmentOperation)
    assert issubclass(Subst, AlignmentOperation)
    assert issubclass(Del, AlignmentOperation)
    assert issubclass(Ins, AlignmentOperation)
    assert issubclass(Xclip, AlignmentOperation)
    assert issubclass(Yclip, AlignmentOperation)


def test_alignment():
    alignment = Alignment(
        score=5,
        x_start=3,
        y_start=0,
        x_end=9,
        y_end=10,
        y_len=10,
        x_len=10,
        operations=[Match(), Match(), Match(), Subst(), Ins(), Ins(), Del(), Del()],
        mode='semiglobal',
    )
    assert alignment is not None
