from typing import Callable

from bioforma.scores import (
    blosum62,
    pam120,
    pam200,
    pam250,
    pam40,
)


def _raise_test(func: Callable[[bytes, bytes], int]):
    try:
        func(b'AA', b'T')
    except ValueError:
        assert True
    else:
        assert False

    try:
        func(b'T', b'AA')
    except ValueError:
        assert True
    else:
        assert False

    try:
        func(b'', b'')
    except ValueError:
        assert True
    else:
        assert False


def test_blosum62():
    assert blosum62(b'A', b'A') == 4
    assert blosum62(b'O', b'*') == -4
    assert blosum62(b'A', b'*') == -4
    assert blosum62(b'*', b'*') == 1
    assert blosum62(b'X', b'X') == -1
    assert blosum62(b'X', b'Z') == -1
    _raise_test(blosum62)


def test_pam120():
    assert pam120(b'A', b'A') == 3
    assert pam120(b'*', b'*') == 1
    assert pam120(b'A', b'*') == -8
    assert pam120(b'*', b'*') == 1
    assert pam120(b'X', b'X') == -2
    assert pam120(b'X', b'Z') == -1
    _raise_test(pam120)


def test_pam200():
    assert pam200(b'A', b'A') == 3
    assert pam200(b'*', b'*') == 1
    assert pam200(b'A', b'*') == -9
    assert pam200(b'Y', b'Z') == -5
    assert pam200(b'X', b'X') == -1
    assert pam200(b'X', b'Z') == -1
    _raise_test(pam200)


def test_pam250():
    assert pam250(b'A', b'A') == 2
    assert pam250(b'*', b'*') == 1
    assert pam250(b'A', b'*') == -8
    assert pam250(b'*', b'*') == 1
    assert pam250(b'X', b'X') == -1
    assert pam250(b'X', b'Z') == -1
    _raise_test(pam250)


def test_pam40():
    assert pam40(b'A', b'A') == 6
    assert pam40(b'*', b'*') == 1
    assert pam40(b'A', b'*') == -15
    assert pam40(b'*', b'*') == 1
    assert pam40(b'X', b'X') == -4
    assert pam40(b'X', b'Z') == -4
    _raise_test(pam40)
