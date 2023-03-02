from .filters import *


class WorkchainId(IntFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "workchain_id"
        IntFilter.__init__(field)
        OrderBy.__init__(field)


class Id(HashFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "id"
        HashFilter.__init__(field)
        OrderBy.__init__(field)


class Aborted(BoolFilter):
    @classmethod
    def __init__(cls):
        BoolFilter.__init__("aborted")


class AccountAddr(AddressFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "account_addr"
        AddressFilter.__init__(field)
        OrderBy.__init__(field)


class BalanceDelta(TokensFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "balance_delta"
        TokensFilter.__init__(field)
        OrderBy.__init__(field)


class BlockId(HashFilter):
    @classmethod
    def __init__(cls):
        HashFilter.__init__("block_id")


class CreditFirst(BoolFilter):
    @classmethod
    def __init__(cls):
        BoolFilter.__init__("credit_first")


class Destroyed(BoolFilter):
    @classmethod
    def __init__(cls):
        BoolFilter.__init__("destroyed")


class Now(IntFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "now"
        IntFilter.__init__(field)
        OrderBy.__init__(field)


class Lt(IntAsStringFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "lt"
        IntAsStringFilter.__init__(field)
        OrderBy.__init__(field)


class PrevTransHash(HashFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "prev_trans_hash"
        HashFilter.__init__(field)
        OrderBy.__init__(field)


class PrevTransLt(IntAsStringFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "prev_trans_lt"
        IntAsStringFilter.__init__(field)
        OrderBy.__init__(field)


class OldHash(HashFilter):
    @classmethod
    def __init__(cls):
        HashFilter.__init__("old_hash")


class NewHash(HashFilter):
    @classmethod
    def __init__(cls):
        HashFilter.__init__("new_hash")


class TrType(TransactionTypeFilter):
    @classmethod
    def __init__(cls):
        TransactionTypeFilter.__init__("tr_type")
