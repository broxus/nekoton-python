from .filters import *


class Id(HashFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "id"
        HashFilter.__init__(field)
        OrderBy.__init__(field)


class BlockId(HashFilter):
    @classmethod
    def __init__(cls):
        HashFilter.__init__("block_id")


class BodyHash(HashFilter):
    @classmethod
    def __init__(cls):
        HashFilter.__init__("body_hash")


class IhrDisabled(BoolFilter):
    @classmethod
    def __init__(cls):
        BoolFilter.__init__("ihr_disabled")


class IhrFee(TokensFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "ihr_fee"
        TokensFilter.__init__(field)
        OrderBy.__init__(field)


class ImportFee(TokensFilter):
    @classmethod
    def __init__(cls):
        field = "import_fee"
        TokensFilter.__init__(field)
        OrderBy.__init__(field)


class Bounce(BoolFilter):
    @classmethod
    def __init__(cls):
        BoolFilter.__init__("bounce")


class Bounced(BoolFilter):
    @classmethod
    def __init__(cls):
        BoolFilter.__init__("bounced")


class Src(AddressFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "src"
        AddressFilter.__init__(field)
        OrderBy.__init__(field)


class SrcWorkchainId(IntFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "src_workchain_id"
        IntFilter.__init__(field)
        OrderBy.__init__(field)


class Dst(AddressFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "dst"
        AddressFilter.__init__(field)
        OrderBy.__init__(field)


class DstWorkchainId(IntFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "dst_workchain_id"
        IntFilter.__init__(field)
        OrderBy.__init__(field)


class MsgType(MessageTypeFilter):
    @classmethod
    def __init__(cls):
        MessageTypeFilter.__init__("msg_type")


class CreatedAt(IntFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "created_at"
        IntFilter.__init__(field)
        OrderBy.__init__(field)


class CreatedLt(IntAsStringFilter, OrderBy):
    @classmethod
    def __init__(cls):
        field = "created_lt"
        IntAsStringFilter.__init__(field)
        OrderBy.__init__(field)
