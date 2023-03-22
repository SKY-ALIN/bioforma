from typing import Sequence, Literal


class AlignmentOperation:
    def __repr__(self) -> str: ...


class Match(AlignmentOperation):
    def __new__(cls) -> Match: ...


class Subst(AlignmentOperation):
    def __new__(cls) -> Subst: ...


class Del(AlignmentOperation):
    def __new__(cls) -> Del: ...


class Ins(AlignmentOperation):
    def __new__(cls) -> Ins: ...


class Xclip(AlignmentOperation):
    def __new__(cls, x: int) -> Xclip: ...


class Yclip(AlignmentOperation):
    def __new__(cls, y: int) -> Yclip: ...


class Alignment:
    def __new__(
            cls,
            score: int,
            x_start: int,
            y_start: int,
            x_end: int,
            y_end: int,
            x_len: int,
            y_len: int,
            operations: Sequence[AlignmentOperation],
            mode: Literal['local', 'semiglobal', 'global', 'custom'] = 'global',
    ) -> AlignmentOperation: ...
    def __repr__(self) -> str: ...
