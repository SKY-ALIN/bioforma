from bioforma.alignment.distance import (
    hamming,
    simd_hamming,
    levenshtein,
    simd_levenshtein,
)


def test_hamming():
    assert hamming(b'GTCTGCATGCG', b'TTTAGCTAGCG') == 5
    try:
        hamming(b"GACTATATCGA", b"TTTAGCTC")
    except ValueError:
        assert True
    else:
        assert False


def test_simd_hamming():
    assert simd_hamming(b'GTCTGCATGCG', b'TTTAGCTAGCG') == 5
    try:
        simd_hamming(b"GACTATATCGA", b"TTTAGCTC")
    except ValueError:
        assert True
    else:
        assert False


def test_levenshtein():
    assert levenshtein(b"ACCGTGGAT", b"AAAAACCGTTGAT") == 5
    assert levenshtein(b"ACCGTGGAT", b"AAAAACCGTTGAT") == levenshtein(b"AAAAACCGTTGAT", b"ACCGTGGAT")
    assert levenshtein(b"AAA", b"TTTT") == 4


def test_simd_levenshtein():
    assert simd_levenshtein(b"ACCGTGGAT", b"AAAAACCGTTGAT") == 5
    assert simd_levenshtein(b"ACCGTGGAT", b"AAAAACCGTTGAT") == simd_levenshtein(b"AAAAACCGTTGAT", b"ACCGTGGAT")
    assert simd_levenshtein(b"AAA", b"TTTT") == 4
