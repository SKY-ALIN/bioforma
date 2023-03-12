from bioforma.scores import blosum62


def test_blosum62():
    assert blosum62(b'A', b'A') == 4
    assert blosum62(b'O', b'*') == -4
    assert blosum62(b'A', b'*') == -4
    assert blosum62(b'*', b'*') == 1
    assert blosum62(b'X', b'X') == -1
    assert blosum62(b'X', b'Z') == -1
