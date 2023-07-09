from itertools import repeat

from bioforma.alignment import (
    Alignment,
    AlignmentOperation,
    Match,
    Subst,
    Del,
    Ins,
    Xclip,
    Yclip,
    Scoring,
    PairwiseAligner,
    DEFAULT_ALIGNER_CAPACITY,
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
    assert issubclass(AlignmentOperation, object)
    assert issubclass(Match, AlignmentOperation)
    assert issubclass(Subst, AlignmentOperation)
    assert issubclass(Del, AlignmentOperation)
    assert issubclass(Ins, AlignmentOperation)
    assert issubclass(Xclip, AlignmentOperation)
    assert issubclass(Yclip, AlignmentOperation)

    def check_magic_method(cls, *args):
        assert hash(cls(*args)) == hash(cls(*args))
        assert cls(*args) == cls(*args)
        assert not (cls(*args) != cls(*args))

    check_magic_method(Match)
    check_magic_method(Subst)
    check_magic_method(Del)
    check_magic_method(Ins)
    check_magic_method(Xclip, 2)
    check_magic_method(Yclip, 3)

    assert Xclip(2) != Xclip(0)
    assert Yclip(3) != Yclip(0)


def test_alignment_cigar():
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
    assert alignment.cigar(False) == "3S3=1X2I2D1S"

    alignment = Alignment(
        score=5,
        x_start=0,
        y_start=5,
        x_end=4,
        y_end=10,
        x_len=5,
        y_len=10 ,
        operations=[Yclip(5), Match(), Subst(), Subst(), Ins(), Del(), Del(), Xclip(1)],
        mode='custom',
    )
    assert alignment.cigar(False) == "1=2X1I2D1S"
    assert alignment.cigar(True) == "1=2X1I2D1H"

    alignment = Alignment(
        score=5,
        x_start=0,
        y_start=5,
        x_end=3,
        y_end=8,
        x_len=3,
        y_len=10,
        operations=[Yclip(5), Subst(), Match(), Subst(), Yclip(2)],
        mode='custom',
    )
    assert alignment.cigar(False) == "1X1=1X"

    alignment = Alignment(
        score=5,
        x_start=0,
        y_start=5,
        x_end=3,
        y_end=8,
        x_len=3,
        y_len=10,
        operations=[Subst(), Match(), Subst()],
        mode='custom',
    )
    assert alignment.cigar(False) == "1X1=1X"


def test_alignment_pretty():
    alignment = Alignment(
        score=1,
        x_start=0,
        y_start=2,
        x_end=3,
        y_end=5,
        x_len=2,
        y_len=7,
        operations=[Subst(), Match(), Match()],
        mode='semiglobal',
    )
    assert alignment.pretty(b"GAT", b"CTAATCC", 100) == (
        "  GAT  \n"
        "  \\||  \n"
        "CTAATCC\n"
        "\n\n"
    )

    alignment = Alignment(
        score=5,
        x_start=0,
        y_start=5,
        x_end=4,
        y_end=10,
        x_len=5,
        y_len=10,
        operations=[Yclip(5), Match(), Subst(), Subst(), Ins(), Del(), Del(), Xclip(1)],
        mode='custom',
    )
    assert alignment.pretty(b"AAAAA", b"TTTTTTTTTT", 100) == (
        "     AAAA--A\n"
        "     |\\\\+xx \n"
        "TTTTTTTT-TT \n"
        "\n\n"
    )


def test_alignment_path():
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
    assert alignment.path() == [
        (4, 5, Match()),
        (5, 6, Match()),
        (6, 7, Match()),
        (7, 8, Subst()),
        (8, 8, Ins()),
        (9, 8, Ins()),
        (9, 9, Del()),
        (9, 10, Del()),
    ]


def test_alignment_properties():
    operations = [Match(), Match(), Match(), Subst(), Ins(), Ins(), Del(), Del()]
    alignment = Alignment(
        score=5,
        x_start=3,
        y_start=0,
        x_end=9,
        y_end=10,
        x_len=10,
        y_len=10,
        operations=operations,
        mode='semiglobal',
    )
    assert alignment.score == 5
    assert alignment.x_start == 3
    assert alignment.y_start == 0
    assert alignment.x_end == 9
    assert alignment.y_end == 10
    assert alignment.x_len == 10
    assert alignment.y_len == 10
    assert alignment.operations == operations
    assert alignment.mode == 'semiglobal'


def test_constants():
    assert DEFAULT_ALIGNER_CAPACITY == 200


def test_semiglobal_pairwise_aligner():
    x = b"ACCGTGGAT"
    y = b"AAAAACCGTTGAT"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_semiglobal(x, y)
    assert alignment.y_start == 4
    assert alignment.x_start == 0
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Subst(),
        Match(),
        Match(),
        Match(),
    ]


def test_semiglobal_gap_open_lt_mismatch_pairwise_aligner():
    x = b"ACCGTGGAT"
    y = b"AAAAACCGTTGAT"
    scoring = Scoring.from_scores(-1, -1, match_score=1, mismatch_score=-5)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_semiglobal(x, y)
    assert alignment.y_start == 4
    assert alignment.x_start == 0
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Match(),
        Del(),
        Match(),
        Ins(),
        Match(),
        Match(),
        Match(),
    ]


def test_global_affine_ins_pairwise_aligner():
    x = b"ACGAGAACA"
    y = b"ACGACA"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-3)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Ins(),
        Ins(),
        Ins(),
        Match(),
        Match(),
        Match(),
    ]


def test_global_affine_ins2_pairwise_aligner():
    x = b"AGATAGATAGATAGGGAGTTGTGTAGATGATCCACAGT"
    y = b"AGATAGATAGATGTAGATGATCCACAGT"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    operations: list[AlignmentOperation] = (
        list(repeat(Match(), 11))
        + list(repeat(Ins(), 10))
        + list(repeat(Match(), 17))
    )
    assert alignment.operations == operations


def test_local_affine_ins2_pairwise_aligner():
    x = b"ACGTATCATAGATAGATAGGGTTGTGTAGATGATCCACAG"
    y = b"CGTATCATAGATAGATGTAGATGATCCACAGT"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_local(x, y)
    assert alignment.x_start == 1
    assert alignment.y_start == 0


def test_local_pairwise_aligner():
    x = b"ACCGTGGAT"
    y = b"AAAAACCGTTGAT"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_local(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 4
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Subst(),
        Match(),
        Match(),
        Match(),
    ]


def test_global_pairwise_aligner():
    x = b"ACCGTGGAT"
    y = b"AAAAACCGTTGAT"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.operations == [
        Del(),
        Del(),
        Del(),
        Del(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Subst(),
        Match(),
        Match(),
        Match(),
    ]


def test_blosum62_pairwise_aligner():
    x = b"AAAA"
    y = b"AAAA"
    scoring = Scoring(-5, -1, 'blosum62')
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.score == 16
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Match(),
    ]


def test_blosum62_local_pairwise_aligner():
    x = b"LSPADKTNVKAA"
    y = b"PEEKSAV"
    scoring = Scoring(-10, -1, 'blosum62')
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_local(x, y)
    assert alignment.x_start == 2
    assert alignment.y_start == 0
    assert alignment.x_end == 9
    assert alignment.y_end == 7
    assert alignment.score == 16
    assert alignment.operations == [
        Match(),
        Subst(),
        Subst(),
        Match(),
        Subst(),
        Subst(),
        Match(),
    ]


def test_issue11_pairwise_aligner():
    y = b"TACC"
    x = b"AAAAACC"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.operations == [
        Ins(),
        Ins(),
        Ins(),
        Subst(),
        Match(),
        Match(),
        Match(),
    ]


def test_issue12_1_pairwise_aligner():
    x = b"CCGGCA"
    y = b"ACCGTTGACGC"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_semiglobal(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 1
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Subst(),
        Subst(),
        Subst(),
    ]


def test_issue12_2_pairwise_aligner():
    x = b"ACCGTTGACGC"
    y = b"CCGGCA"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_semiglobal(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.operations == [
        Subst(),
        Match(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Subst(),
        Match(),
        Match(),
    ]


def test_issue12_3_pairwise_aligner():
    x = b"AAAAACCGTTGACGCAA"
    y = b"CCGTCCGGCAA"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)

    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_semiglobal(x, y)
    assert alignment.x_start == 0
    assert alignment.operations == [
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Match(),
        Subst(),
        Subst(),
        Match(),
        Subst(),
        Subst(),
        Subst(),
        Match(),
        Match(),
        Match(),
        Match(),
    ]

    aligner = PairwiseAligner(scoring, m=len(y), n=len(x))
    alignment = aligner.calculate_semiglobal(y, x)
    assert alignment.x_start == 0
    assert alignment.operations == [
        Match(),
        Subst(),
        Subst(),
        Match(),
        Subst(),
        Subst(),
        Subst(),
        Match(),
        Match(),
        Match(),
        Match(),
    ]


def test_left_aligned_del_pairwise_aligner():
    x = b"GTGCATCATGTG"
    y = b"GTGCATCATCATGTG"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Del(),
        Del(),
        Del(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
    ]


def test_global_right_del_pairwise_aligner():
    x = b"AACCACGTACGTGGGGGGA"
    y = b"CCACGTACGT"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.score == -9
    assert alignment.operations == [
        Ins(),
        Ins(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
        Ins(),
    ]


def test_left_aligned_ins_pairwise_aligner():
    x = b"GTGCATCATCATGTG"
    y = b"GTGCATCATGTG"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring, m=len(x), n=len(y))
    alignment = aligner.calculate_global(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.operations == [
        Match(),
        Match(),
        Match(),
        Ins(),
        Ins(),
        Ins(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
    ]


def test_aligner_new_pairwise_aligner():  # test default capacity
    x = b"ACCGTGGAT"
    y = b"AAAAACCGTTGAT"
    scoring = Scoring.from_scores(-5, -1, match_score=1, mismatch_score=-1)
    aligner = PairwiseAligner(scoring)

    alignment = aligner.calculate_semiglobal(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 4
    operations = [
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Subst(),
        Match(),
        Match(),
        Match(),
    ]
    assert alignment.operations == operations

    alignment = aligner.calculate_local(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 4
    assert alignment.operations == operations

    alignment = aligner.calculate_global(x, y)
    assert alignment.x_start == 0
    assert alignment.y_start == 0
    assert alignment.operations == [
        Del(),
        Del(),
        Del(),
        Del(),
        Match(),
        Match(),
        Match(),
        Match(),
        Match(),
        Subst(),
        Match(),
        Match(),
        Match(),
    ]
