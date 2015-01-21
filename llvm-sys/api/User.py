from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath, copymodpath
from .ns import llvm
from .defs import *

@User.body
class User:
    delete = Destructor()

    getOperand = Method(ptr(Value), (UnsignedInt, 'idx'), const=True)
    setOperand = Method(Void, (UnsignedInt, 'idx'), (ptr(Value), 'Val'))
    getNumOperands = Method(UnsignedInt, const=True)

    dropAllReferences = Method()
    replaceUsesOfWith = Method(Void, (ptr(Value), 'From'), (ptr(Value), 'To'))

    classof = StaticMethod(Bool, (ptr(Value), 'V'))

@Use.body
class Use:
    swap = Method(Void, (ref(Use), 'RHS'))

    get = Method(ptr(Value), const=True)
    set = Method(Void, (ptr(Value), 'Val'))

    getUser = Method(ptr(User), const=True)

    getNext = Method(ptr(Use), const=True)

    getOperandNo = Method(UnsignedInt, const=True)

    initTags = StaticMethod(ptr(Use), (ptr(Use), 'Start'), (ptr(Use), 'Stop'))

@Operator.body
class Operator:
    _includes_ = ['llvm/IR/Operator.h']

    getOpcode = Method(UnsignedInt, const=True)
