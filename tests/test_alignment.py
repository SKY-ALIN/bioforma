from bioforma.alignment.distance import (
    hamming,
    simd_hamming,
    levenshtein,
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
    assert hamming(b'GTCTGCATGCG', b'TTTAGCTAGCG') == 5
    try:
        hamming(b"GACTATATCGA", b"TTTAGCTC")
    except ValueError:
        assert True
    else:
        assert False


def test_levenshtein():
    assert levenshtein(b"ACCGTGGAT", b"AAAAACCGTTGAT") == 5
    assert levenshtein(b"ACCGTGGAT", b"AAAAACCGTTGAT") == levenshtein(b"AAAAACCGTTGAT", b"ACCGTGGAT")
    assert levenshtein(b"AAA", b"TTTT") == 4
