from typing import List as _List

from nekoton.nekoton import GqlExprPart

from . import acc as acc
from . import filters as filters
from . import msg as msg
from . import tx as tx


def and_(expressions: str | GqlExprPart | _List[GqlExprPart]) -> GqlExprPart:
    if isinstance(expressions, GqlExprPart):
        return expressions
    if isinstance(expressions, str):
        return GqlExprPart(expressions)
    else:
        return GqlExprPart(",".join(map(str, expressions)))


def or_(expressions: _List[str | GqlExprPart | _List[GqlExprPart]]) -> GqlExprPart:
    last_part = None
    for expr in reversed(expressions):
        element = and_(expr)
        if last_part is not None:
            last_part = GqlExprPart("{},OR:{{{}}}".format(element, last_part))
        else:
            last_part = element

    if last_part is None:
        raise RuntimeError("Empty OR for gql filter")

    return last_part
