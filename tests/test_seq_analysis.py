from bioforma.seq_analysis.gc import gc_content, gc3_content


def test_gc():
    assert gc_content(b"GATATACA") == 2. / 8.
    assert gc_content(b"ATAT") == .0
    assert gc_content(b"ATGC") == .5
    assert gc_content(b"GCGC") == 1.

    assert round(gc3_content(b"GATATACA"), 6) == round(2. / 3., 6)
