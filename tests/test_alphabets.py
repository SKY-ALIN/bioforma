from bioforma.alphabets import Alphabet, RankTransform
from bioforma.alphabets.dna import (
    make_dna_alphabet,
    make_dna_n_alphabet,
    make_dna_iupac_alphabet,
    get_dna_symbol_complement,
    get_dna_complement,
)
from bioforma.alphabets.protein import (
    make_protein_alphabet,
    make_protein_iupac_alphabet,
)
from bioforma.alphabets.rna import (
    make_rna_alphabet,
    make_rna_n_alphabet,
    make_rna_iupac_alphabet,
    get_rna_symbol_complement,
    get_rna_complement,
)


def test_alphabet():
    a = Alphabet(b'ACGT')
    assert a.symbols == b'ACGT'
    assert bytes(a) == b'ACGT'
    assert len(a) == 4
    assert repr(a) == '<Alphabet: ACGT>'
    assert a.is_word(b'GATTACA')
    assert not a.is_word(b'42')

    b = Alphabet(b'ATX')
    assert bytes(a & b) == b'AT'
    assert bytes(a | b) == b'ACGTX'


def test_rank_transform():
    a = Alphabet(b'ACGTacgt')
    rt = RankTransform(a)
    assert rt.get(b'A') == 0
    assert rt.get(b't') == 7
    assert rt.transform(b'aAcCgGtT') == [4, 0, 5, 1, 6, 2, 7, 3]
    assert rt.ranks =={'G': 2, 't': 7, 'c': 5, 'C': 1, 'a': 4, 'A': 0, 'T': 3, 'g': 6}
    assert repr(rt) == '<RankTransform: A-0, C-1, G-2, T-3, a-4, c-5, g-6, t-7>'

    try:
        rt.transform(b'acxben')
    except ValueError:
        assert True
    else:
        assert False

    assert rt.q_grams(2, b'ACGT') == [1, 10, 19]

    assert RankTransform(Alphabet(b'ACGT')).get_width() == 2
    assert RankTransform(Alphabet(b'ACGTN')).get_width() == 3


def test_dna():
    assert make_dna_alphabet().symbols == b'ACGTacgt'
    assert set(make_dna_n_alphabet().symbols) == set(b'ACGTNacgtn')
    assert set(make_dna_iupac_alphabet().symbols) == set(b'ACGTRYSWKMBDHVNZacgtryswkmbdhvnz')

    assert get_dna_symbol_complement(b'A') == b'T'
    assert get_dna_symbol_complement(b'c') == b'g'
    assert get_dna_symbol_complement(b'N') == b'N'
    assert get_dna_symbol_complement(b'Y') == b'R'
    assert get_dna_symbol_complement(b's') == b's'

    # Length check
    try:
        get_dna_symbol_complement(b'AA')
    except ValueError:
        assert True
    else:
        assert False

    assert get_dna_complement(b'ACGTN') == b'NACGT'
    assert get_dna_complement(b'GaTtaCA') == b'TGtaAtC'
    assert get_dna_complement(b'AGCTYRWSKMDVHBN') == b'NVDBHKMSWYRAGCT'


def test_protein():
    assert set(make_protein_alphabet().symbols) == set(b'ARNDCEQGHILKMFPSTWYVarndceqghilkmfpstwyv')
    assert set(make_protein_iupac_alphabet().symbols) == set(b'ABCDEFGHIKLMNPQRSTVWXYZabcdefghiklmnpqrstvwxyz')


def test_rna():
    assert make_rna_alphabet().symbols == b'ACGUacgu'
    assert set(make_rna_n_alphabet().symbols) == set(b'ACGUNacgun')
    assert set(make_rna_iupac_alphabet().symbols) == set(b'ACGURYSWKMBDHVNZacguryswkmbdhvnz')

    assert get_rna_symbol_complement(b'A') == b'U'
    assert get_rna_symbol_complement(b'g') == b'c'
    assert get_rna_symbol_complement(b'Y') == b'R'
    assert get_rna_symbol_complement(b's') == b's'
    assert get_rna_symbol_complement(b'N') == b'N'

    # Length check
    try:
        get_rna_symbol_complement(b'AA')
    except ValueError:
        assert True
    else:
        assert False

    assert get_rna_complement(b'ACGUN') == b'NACGU'
    assert get_rna_complement(b'GaUuaCA') == b'UGuaAuC'
    assert get_rna_complement(b'AGCUYRWSKMDVHBNZ') == b'ZNVDBHKMSWYRAGCU'
