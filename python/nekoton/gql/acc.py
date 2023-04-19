from .filters import IntFilter as _IntFilter, OrderBy as _OrderBy, AddressFilter as _AddressFilter, \
    AccountStatusFilter as _AccountStatusFilter, TokensFilter as _TokensFilter, HashFilter as _HashFilter, \
    IntAsStringFilter as _IntAsStringFilter


class WorkchainId(_IntFilter, _OrderBy):
    def __init__(self):
        field = "workchain_id"
        _IntFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class Id(_AddressFilter, _OrderBy):
    def __init__(self):
        field = "id"
        _AddressFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class Status(_AccountStatusFilter):
    def __init__(self):
        _AccountStatusFilter.__init__(self, "acc_type")


class Balance(_TokensFilter, _OrderBy):
    def __init__(self):
        field = "balance"
        _TokensFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class LastTransLt(_IntAsStringFilter, _OrderBy):
    def __init__(self):
        field = "last_trans_lt"
        _IntAsStringFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class LastPaid(_IntFilter, _OrderBy):
    def __init__(self):
        field = "last_paid"
        _IntFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class CodeHash(_HashFilter):
    def __init__(self):
        _HashFilter.__init__(self, "code_hash")


class InitCodeHash(_HashFilter):
    def __init__(self):
        _HashFilter.__init__(self, "init_code_hash")
