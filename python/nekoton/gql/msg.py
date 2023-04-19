from .filters import OrderBy as _OrderBy, IntFilter as _IntFilter, HashFilter as _HashFilter, TokensFilter as _TokensFilter, \
    BoolFilter as _BoolFilter, AddressFilter as _AddressFilter, MessageTypeFilter as _MessageTypeFilter, \
    IntAsStringFilter as _IntAsStringFilter


class Id(_HashFilter, _OrderBy):
    def __init__(self):
        field = "id"
        _HashFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class BlockId(_HashFilter):
    def __init__(self):
        _HashFilter.__init__(self, "block_id")


class BodyHash(_HashFilter):
    def __init__(self):
        _HashFilter.__init__(self, "body_hash")


class IhrDisabled(_BoolFilter):
    def __init__(self):
        _BoolFilter.__init__(self, "ihr_disabled")


class IhrFee(_TokensFilter, _OrderBy):
    def __init__(self):
        field = "ihr_fee"
        _TokensFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class ImportFee(_TokensFilter, _OrderBy):
    def __init__(self):
        field = "import_fee"
        _TokensFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class Bounce(_BoolFilter):
    def __init__(self):
        _BoolFilter.__init__(self, "bounce")


class Bounced(_BoolFilter):
    def __init__(self):
        _BoolFilter.__init__(self, "bounced")


class Src(_AddressFilter, _OrderBy):
    def __init__(self):
        field = "src"
        _AddressFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class SrcWorkchainId(_IntFilter, _OrderBy):
    def __init__(self):
        field = "src_workchain_id"
        _IntFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class Dst(_AddressFilter, _OrderBy):
    def __init__(self):
        field = "dst"
        _AddressFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class DstWorkchainId(_IntFilter, _OrderBy):
    def __init__(self):
        field = "dst_workchain_id"
        _IntFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class MsgType(_MessageTypeFilter):
    def __init__(self):
        _MessageTypeFilter.__init__(self, "msg_type")


class CreatedAt(_IntFilter, _OrderBy):
    def __init__(self):
        field = "created_at"
        _IntFilter.__init__(self, field)
        _OrderBy.__init__(self, field)


class CreatedLt(_IntAsStringFilter, _OrderBy):
    def __init__(self):
        field = "created_lt"
        _IntAsStringFilter.__init__(self, field)
        _OrderBy.__init__(self, field)
