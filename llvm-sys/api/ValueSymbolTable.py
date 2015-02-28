from .prelude import *
from .ADT.StringRef import StringRef

@ValueSymbolTable.body
class ValueSymbolTable:
    new = Constructor()
    delete = Destructor()

    lookup = Method(ptr(Value), (StringRef, 'Name'), const=True)

    empty = Method(Bool, const=True)
    size = Method(UnsignedInt, const=True)

    dump = Method(const=True)
