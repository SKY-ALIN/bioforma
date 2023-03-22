from bioforma.seq_analysis.gc import gc_content, gc3_content
from bioforma.seq_analysis.orf import Finder


def test_gc():
    assert gc_content(b'GATATACA') == 2. / 8.
    assert gc_content(b'ATAT') == .0
    assert gc_content(b'ATGC') == .5
    assert gc_content(b'GCGC') == 1.

    assert round(gc3_content(b'GATATACA'), 6) == round(2. / 3., 6)


def test_finder_init():
    try:
        Finder([b'ATGG'], [b'TGA'], 50)
    except ValueError:
        assert True
    else:
        assert False

    try:
        Finder([], [], 50)
    except ValueError:
        assert True
    else:
        assert False


def test_orf():
    f = Finder([b'ATG'], [b'TGA', b'TAG', b'TAA'], 5)
    assert f.find_all(b'ACGGCTAGAAAAGGCTAGAAAA') == []

    res = f.find_all(b'GGGATGGGGTGAGGG')
    assert len(res) == 1
    assert res[0].start == 3
    assert res[0].end == 12
    assert res[0].offset == 0
    assert repr(res[0]) == '<Orf: start=3, end=12, offset=0>'

    res = f.find_all(b'AGGGATGGGGTGAGGG')
    assert len(res) == 1
    assert res[0].start == 4
    assert res[0].end == 13
    assert res[0].offset == 1

    res = f.find_all(b'ATGGGGTGAGGGGGATGGAAAAATAAG')
    assert len(res) == 2
    assert res[0].start == 0
    assert res[0].end == 9
    assert res[0].offset == 0
    assert res[1].start == 14
    assert res[1].end == 26
    assert res[1].offset == 2

    res = f.find_all(b'ATGGGGATGGGGGGATGGAAAAATAAGTAG')
    assert len(res) == 3
    assert res[0].start == 14
    assert res[0].end == 26
    assert res[0].offset == 2
    assert res[1].start == 0
    assert res[1].end == 30
    assert res[1].offset == 0
    assert res[2].start == 6
    assert res[2].end == 30
    assert res[2].offset == 0
