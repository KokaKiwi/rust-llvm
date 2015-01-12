from bindgen.ast.objects import *
from bindgen.ast.utils import submodpath
from .ns import llvm
from .ADT.APInt import APInt
from .Type import Type
from .User import User
from .Value import Value

@llvm.body
class llvm_body:
    _includes_ = ['llvm/IR/Constants.h']

Constant = llvm.Class('Constant', User)
Constant.modpath = submodpath(['constant'])

BlockAddress = llvm.Class('BlockAddress', Constant)
ConstantAggregateZero = llvm.Class('ConstantAggregateZero', Constant)
ConstantArray = llvm.Class('ConstantArray', Constant)
ConstantDataSequential = llvm.Class('ConstantDataSequential', Constant)
ConstantExpr = llvm.Class('ConstantExpr', Constant)
ConstantFP = llvm.Class('ConstantFP', Constant)
ConstantInt = llvm.Class('ConstantInt', Constant)
ConstantPointerNull = llvm.Class('ConstantPointerNull', Constant)
ConstantStruct = llvm.Class('ConstantStruct', Constant)
ConstantVector = llvm.Class('ConstantVector', Constant)
GlobalValue = llvm.Class('GlobalValue', Constant)
UndefValue = llvm.Class('UndefValue', Constant)

ConstantDataArray = llvm.Class('ConstantDataArray', ConstantDataSequential)
ConstantDataVector = llvm.Class('ConstantDataVector', ConstantDataSequential)

@Constant.body
class Constant:
    def test_value():
        return Method(Bool, const=True)

    isNullValue = test_value()
    isAllOnesValue = test_value()
    isNegativeZeroValue = test_value()
    isZeroValue = test_value()
    isMinSignedValue = test_value()

    canTrap = test_value()
    isThreadDependent = test_value()
    isDLLImportDependent = test_value()
    isConstantUsed = test_value()

    getAggregateElement = Method(ptr(Constant), (UnsignedInt, 'Elt'), const=True)
    getAggregateElementConstant = Method(ptr(Constant), (ptr(Constant), 'Elt'), const=True).with_call_name('getAggregateElement')
    getSplatValue = Method(ptr(Constant), const=True)

    destroyConstant = Method()
    # replaceUsesOfWithOnConstant = Method(Void, ptr(Value), ptr(Value), ptr(Use))
    removeDeadConstantUsers = Method(const=True)

    stripPointerCasts = Method(ptr(Constant, const=True), const=True)
    stripPointerCastsMut = Method(ptr(Constant)).with_call_name('stripPointerCasts')

    classof = StaticMethod(Bool, (ptr(Value, const=True), 'V'))

    getNullValue = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))
    getAllOnesValue = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'))
    getIntegerValue = StaticMethod(ptr(Constant), (ptr(Type), 'Ty'), (APInt, 'Value'))
