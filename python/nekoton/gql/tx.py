from .filters import IntFilter as _IntFilter, OrderBy as _OrderBy, HashFilter as _HashFilter, BoolFilter as _BoolFilter, \
    AddressFilter as _AddressFilter, TokensFilter as _TokensFilter, IntAsStringFilter as _IntAsStringFilter, \
    TransactionTypeFilter as _TransactionTypeFilter


class WorkchainId(_IntFilter, _OrderBy):
    def __init__(self):
        field = "workchain_id"
        _IntFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class Id(_HashFilter, _OrderBy):
    def __init__(self):
        field = "id"
        _HashFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class Aborted(_BoolFilter):
    def __init__(self):
        _BoolFilter.__init__(self, "aborted")


class AccountAddr(_AddressFilter, _OrderBy):
    def __init__(self):
        field = "account_addr"
        _AddressFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class BalanceDelta(_TokensFilter, _OrderBy):
    def __init__(self):
        field = "balance_delta"
        _TokensFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class BlockId(_HashFilter):
    def __init__(self):
        _HashFilter.__init__(self, "block_id")


class CreditFirst(_BoolFilter):
    def __init__(self):
        _BoolFilter.__init__(self, "credit_first")


class Destroyed(_BoolFilter):
    def __init__(self):
        _BoolFilter.__init__(self, "destroyed")


class Now(_IntFilter, _OrderBy):
    def __init__(self):
        field = "now"
        _IntFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class Lt(_IntAsStringFilter, _OrderBy):
    def __init__(self):
        field = "lt"
        _IntAsStringFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class PrevTransHash(_HashFilter, _OrderBy):
    def __init__(self):
        field = "prev_trans_hash"
        _HashFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class PrevTransLt(_IntAsStringFilter, _OrderBy):
    def __init__(self):
        field = "prev_trans_lt"
        _IntAsStringFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class OldHash(_HashFilter):
    def __init__(self):
        _HashFilter.__init__(self, "old_hash")


class NewHash(_HashFilter):
    def __init__(self):
        _HashFilter.__init__(self, "new_hash")


class TrType(_TransactionTypeFilter):
    def __init__(self):
        _TransactionTypeFilter.__init__(self, "tr_type")
