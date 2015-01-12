from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath
from .ns import llvm
from .Value import Value

User = llvm.Class('User', Value)
User.modpath = submodpath(['user'])

Operator = llvm.Class('Operator', User)

@User.body
class User:
    delete = Destructor()

    getOperand = Method(ptr(Value), (UnsignedInt, 'idx'), const=True)
    setOperand = Method(Void, (UnsignedInt, 'idx'), (ptr(Value), 'Val'))
    getNumOperands = Method(UnsignedInt, const=True)

    dropAllReferences = Method()
    replaceUsesOfWith = Method(Void, (ptr(Value), 'From'), (ptr(Value), 'To'))

    classof = StaticMethod(Bool, (ptr(Value), 'V'))
