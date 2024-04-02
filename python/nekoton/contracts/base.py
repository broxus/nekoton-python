import nekoton as _nt


class IGiver:
    """
    Abstract tokens giver.
    """

    async def give(self, target: _nt.Address, amount: _nt.Tokens):
        raise NotImplementedError("IGiver is an abstract class")
