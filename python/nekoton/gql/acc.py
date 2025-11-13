from .filters import AccountStatusFilter as _AccountStatusFilter
from .filters import AddressFilter as _AddressFilter
from .filters import HashFilter as _HashFilter
from .filters import IntAsStringFilter as _IntAsStringFilter
from .filters import IntFilter as _IntFilter
from .filters import OrderBy as _OrderBy
from .filters import TokensFilter as _TokensFilter


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
