from typing import Iterable as _Iterable

from nekoton import Address as _Address, Tokens as _Tokens, TransactionType as _TransactionType, MessageType as _MessageType, \
    AccountStatus as _AccountStatus, GqlExprPart


class OrderBy:
    def __init__(self, path: str):
        self._field_path = path

    def asc(self) -> GqlExprPart:
        return GqlExprPart('{{path:"{}",direction:ASC}}'.format(self._field_path))

    def desc(self) -> GqlExprPart:
        return GqlExprPart('{{path:"{}",direction:DESC}}'.format(self._field_path))


class BoolFilter:
    def __init__(self, field: str):
        self._field = field

    def _op(self, value: bool) -> GqlExprPart:
        if value:
            return GqlExprPart('{}:{{eq:true}}'.format(self._field))
        else:
            return GqlExprPart('{}:{{eq:false}}'.format(self._field))

    def __invert__(self) -> GqlExprPart:
        return self._op(False)

    def __eq__(self, other: bool) -> GqlExprPart:
        return self._op(other)

    def __ne__(self, other: bool) -> GqlExprPart:
        return self._op(not other)


class IntFilter:
    def __init__(self, field: str):
        self._field = field

    def any_of(self, values: _Iterable[int]) -> GqlExprPart:
        return self._multi_op('in', values)

    def not_any_of(self, values: _Iterable[int]) -> GqlExprPart:
        return self._multi_op('notIn', values)

    def _op(self, op: str, value: int) -> GqlExprPart:
        return GqlExprPart('{}:{{{}:{}}}'.format(self._field, op, value))

    def _multi_op(self, op: str, values: _Iterable[int]) -> GqlExprPart:
        return GqlExprPart('{}:{{{}:[{}]}}'.format(self._field, op, ','.join(map(str, values))))

    def __eq__(self, other: int) -> GqlExprPart:
        return self._op("eq", other)

    def __ne__(self, other: int) -> GqlExprPart:
        return self._op("ne", other)

    def __gt__(self, other: int) -> GqlExprPart:
        return self._op("gt", other)

    def __lt__(self, other: int) -> GqlExprPart:
        return self._op("lt", other)

    def __le__(self, other: int) -> GqlExprPart:
        return self._op("le", other)


class StringFilter:
    def __init__(self, field: str):
        self._field = field

    def any_of(self, values: _Iterable[str]) -> GqlExprPart:
        return self._multi_op('in', values)

    def not_any_of(self, values: _Iterable[str]) -> GqlExprPart:
        return self._multi_op('notIn', values)

    def _op(self, op: str, value: str) -> GqlExprPart:
        return GqlExprPart('{}:{{{}:"{}"}}'.format(self._field, op, value))

    def _multi_op(self, op: str, values: _Iterable[str]) -> GqlExprPart:
        values = '","'.join(values)
        if not values:
            return GqlExprPart('{}:{{{}:[]}}'.format(self._field, op))
        else:
            return GqlExprPart('{}:{{{}:["{}"]}}'.format(self._field, op, values))

    def __eq__(self, other: str) -> GqlExprPart:
        return self._op("eq", other)

    def __ne__(self, other: str) -> GqlExprPart:
        return self._op("ne", other)

    def __gt__(self, other: str) -> GqlExprPart:
        return self._op("gt", other)

    def __lt__(self, other: str) -> GqlExprPart:
        return self._op("lt", other)

    def __le__(self, other: str) -> GqlExprPart:
        return self._op("le", other)


class IntAsStringFilter(StringFilter):
    def any_of(self, values: _Iterable[int | str]) -> GqlExprPart:
        return self._multi_op('in', map(str, values))

    def not_any_of(self, values: _Iterable[int | str]) -> GqlExprPart:
        return self._multi_op('notIn', map(str, values))

    def __eq__(self, other: int | str) -> GqlExprPart:
        return self._op("eq", str(other))

    def __ne__(self, other: int | str) -> GqlExprPart:
        return self._op("ne", str(other))

    def __gt__(self, other: int | str) -> GqlExprPart:
        return self._op("gt", str(other))

    def __lt__(self, other: int | str) -> GqlExprPart:
        return self._op("lt", str(other))

    def __le__(self, other: int | str) -> GqlExprPart:
        return self._op("le", str(other))


class TokensFilter(StringFilter):
    @staticmethod
    def __convert(value: _Tokens | int | str) -> str:
        if isinstance(value, _Tokens):
            return str(value.to_nano())
        else:
            return str(value)

    def any_of(self, values: _Iterable[_Tokens | int | str]) -> GqlExprPart:
        return self._multi_op('in', map(TokensFilter.__convert, values))

    def not_any_of(self, values: _Iterable[_Tokens | int | str]) -> GqlExprPart:
        return self._multi_op('notIn', map(TokensFilter.__convert, values))

    def __eq__(self, other: _Tokens | int | str) -> GqlExprPart:
        return self._op("eq", TokensFilter.__convert(other))

    def __ne__(self, other: _Tokens | int | str) -> GqlExprPart:
        return self._op("ne", TokensFilter.__convert(other))

    def __gt__(self, other: _Tokens | int | str) -> GqlExprPart:
        return self._op("gt", TokensFilter.__convert(other))

    def __lt__(self, other: _Tokens | int | str) -> GqlExprPart:
        return self._op("lt", TokensFilter.__convert(other))

    def __le__(self, other: _Tokens | int | str) -> GqlExprPart:
        return self._op("le", TokensFilter.__convert(other))


class HashFilter(StringFilter):
    @staticmethod
    def __convert(value: bytes | str) -> str:
        if isinstance(value, (bytes, bytearray)):
            return value.hex()
        else:
            return value

    def any_of(self, values: _Iterable[bytes | str]) -> GqlExprPart:
        return self._multi_op('in', map(HashFilter.__convert, values))

    def not_any_of(self, values: _Iterable[bytes | str]) -> GqlExprPart:
        return self._multi_op('notIn', map(HashFilter.__convert, values))

    def __eq__(self, other: bytes | str) -> GqlExprPart:
        return self._op("eq", HashFilter.__convert(other))

    def __ne__(self, other: bytes | str) -> GqlExprPart:
        return self._op("ne", HashFilter.__convert(other))

    def __gt__(self, other: bytes | str) -> GqlExprPart:
        return self._op("gt", HashFilter.__convert(other))

    def __lt__(self, other: bytes | str) -> GqlExprPart:
        return self._op("lt", HashFilter.__convert(other))

    def __le__(self, other: bytes | str) -> GqlExprPart:
        return self._op("le", HashFilter.__convert(other))


class AddressFilter(StringFilter):
    def any_of(self, values: _Iterable[_Address | str]) -> GqlExprPart:
        return self._multi_op('in', map(str, values))

    def not_any_of(self, values: _Iterable[_Address | str]) -> GqlExprPart:
        return self._multi_op('notIn', map(str, values))

    def __eq__(self, other: _Address | str) -> GqlExprPart:
        return self._op("eq", str(other))

    def __ne__(self, other: _Address | str) -> GqlExprPart:
        return self._op("ne", str(other))

    def __gt__(self, other: _Address | str) -> GqlExprPart:
        return self._op("gt", str(other))

    def __lt__(self, other: _Address | str) -> GqlExprPart:
        return self._op("lt", str(other))

    def __le__(self, other: _Address | str) -> GqlExprPart:
        return self._op("le", str(other))


class TransactionTypeFilter(IntFilter):
    def any_of(self, values: _Iterable[_TransactionType | int]) -> GqlExprPart:
        return self._multi_op('in', map(int, values))

    def not_any_of(self, values: _Iterable[_TransactionType | int]) -> GqlExprPart:
        return self._multi_op('notIn', map(int, values))

    def __eq__(self, other: _TransactionType | int) -> GqlExprPart:
        return self._op("eq", int(other))

    def __ne__(self, other: _TransactionType | int) -> GqlExprPart:
        return self._op("ne", int(other))

    def __gt__(self, other: _TransactionType | int) -> GqlExprPart:
        return self._op("gt", int(other))

    def __lt__(self, other: _TransactionType | int) -> GqlExprPart:
        return self._op("lt", int(other))

    def __le__(self, other: _TransactionType | int) -> GqlExprPart:
        return self._op("le", int(other))


class MessageTypeFilter(IntFilter):
    def any_of(self, values: _Iterable[_MessageType | int]) -> GqlExprPart:
        return self._multi_op('in', map(int, values))

    def not_any_of(self, values: _Iterable[_MessageType | int]) -> GqlExprPart:
        return self._multi_op('notIn', map(int, values))

    def __eq__(self, other: _MessageType | int) -> GqlExprPart:
        return self._op("eq", int(other))

    def __ne__(self, other: _MessageType | int) -> GqlExprPart:
        return self._op("ne", int(other))

    def __gt__(self, other: _MessageType | int) -> GqlExprPart:
        return self._op("gt", int(other))

    def __lt__(self, other: _MessageType | int) -> GqlExprPart:
        return self._op("lt", int(other))

    def __le__(self, other: _MessageType | int) -> GqlExprPart:
        return self._op("le", int(other))


class AccountStatusFilter(IntFilter):
    def any_of(self, values: _Iterable[_AccountStatus | int]) -> GqlExprPart:
        return self._multi_op('in', map(int, values))

    def not_any_of(self, values: _Iterable[_AccountStatus | int]) -> GqlExprPart:
        return self._multi_op('notIn', map(int, values))

    def __eq__(self, other: _AccountStatus | int) -> GqlExprPart:
        return self._op("eq", int(other))

    def __ne__(self, other: _AccountStatus | int) -> GqlExprPart:
        return self._op("ne", int(other))

    def __gt__(self, other: _AccountStatus | int) -> GqlExprPart:
        return self._op("gt", int(other))

    def __lt__(self, other: _AccountStatus | int) -> GqlExprPart:
        return self._op("lt", int(other))

    def __le__(self, other: _AccountStatus | int) -> GqlExprPart:
        return self._op("le", int(other))
