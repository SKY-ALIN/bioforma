from . import Alphabet


def make_dna_alphabet() -> Alphabet: ...
def make_dna_n_alphabet() -> Alphabet: ...
def make_dna_iupac_alphabet() -> Alphabet: ...
def get_dna_symbol_complement(chr: bytes) -> bytes: ...
def get_dna_complement(text: bytes) -> bytes: ...
