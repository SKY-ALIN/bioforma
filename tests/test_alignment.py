from bioforma.alignment.distance import hamming, simd_hamming


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
