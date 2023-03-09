from .filters import *


class WorkchainId(IntFilter, OrderBy):
    def __init__(self):
        field = "workchain_id"
        IntFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class Id(HashFilter, OrderBy):
    def __init__(self):
        field = "id"
        HashFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class Aborted(BoolFilter):
    def __init__(self):
        BoolFilter.__init__(self, "aborted")


class AccountAddr(AddressFilter, OrderBy):
    def __init__(self):
        field = "account_addr"
        AddressFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class BalanceDelta(TokensFilter, OrderBy):
    def __init__(self):
        field = "balance_delta"
        TokensFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class BlockId(HashFilter):
    def __init__(self):
        HashFilter.__init__(self, "block_id")


class CreditFirst(BoolFilter):
    def __init__(self):
        BoolFilter.__init__(self, "credit_first")


class Destroyed(BoolFilter):
    def __init__(self):
        BoolFilter.__init__(self, "destroyed")


class Now(IntFilter, OrderBy):
    def __init__(self):
        field = "now"
        IntFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class Lt(IntAsStringFilter, OrderBy):
    def __init__(self):
        field = "lt"
        IntAsStringFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class PrevTransHash(HashFilter, OrderBy):
    def __init__(self):
        field = "prev_trans_hash"
        HashFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class PrevTransLt(IntAsStringFilter, OrderBy):
    def __init__(self):
        field = "prev_trans_lt"
        IntAsStringFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class OldHash(HashFilter):
    def __init__(self):
        HashFilter.__init__(self, "old_hash")


class NewHash(HashFilter):
    def __init__(self):
        HashFilter.__init__(self, "new_hash")


class TrType(TransactionTypeFilter):
    def __init__(self):
        TransactionTypeFilter.__init__(self, "tr_type")
