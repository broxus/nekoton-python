from .filters import *


class Id(HashFilter, OrderBy):
    def __init__(self):
        field = "id"
        HashFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class BlockId(HashFilter):
    def __init__(self):
        HashFilter.__init__(self, "block_id")


class BodyHash(HashFilter):
    def __init__(self):
        HashFilter.__init__(self, "body_hash")


class IhrDisabled(BoolFilter):
    def __init__(self):
        BoolFilter.__init__(self, "ihr_disabled")


class IhrFee(TokensFilter, OrderBy):
    def __init__(self):
        field = "ihr_fee"
        TokensFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class ImportFee(TokensFilter, OrderBy):
    def __init__(self):
        field = "import_fee"
        TokensFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class Bounce(BoolFilter):
    def __init__(self):
        BoolFilter.__init__(self, "bounce")


class Bounced(BoolFilter):
    def __init__(self):
        BoolFilter.__init__(self, "bounced")


class Src(AddressFilter, OrderBy):
    def __init__(self):
        field = "src"
        AddressFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class SrcWorkchainId(IntFilter, OrderBy):
    def __init__(self):
        field = "src_workchain_id"
        IntFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class Dst(AddressFilter, OrderBy):
    def __init__(self):
        field = "dst"
        AddressFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class DstWorkchainId(IntFilter, OrderBy):
    def __init__(self):
        field = "dst_workchain_id"
        IntFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class MsgType(MessageTypeFilter):
    def __init__(self):
        MessageTypeFilter.__init__(self, "msg_type")


class CreatedAt(IntFilter, OrderBy):
    def __init__(self):
        field = "created_at"
        IntFilter.__init__(self, field)
        OrderBy.__init__(self, field)


class CreatedLt(IntAsStringFilter, OrderBy):
    def __init__(self):
        field = "created_lt"
        IntAsStringFilter.__init__(self, field)
        OrderBy.__init__(self, field)
