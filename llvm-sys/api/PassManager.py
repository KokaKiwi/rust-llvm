from .prelude import *


@FunctionPassManager.body
class FunctionPassManager:
    new = Constructor((ptr(Module), 'Module'))

    add = Method(Void, (ptr(FunctionPass, owned=True), 'Pass'))
    run = Method(Void, (ref(Function), 'Function'))

    doInitialization = Method(Bool)
    doFinalization = Method(Bool)


@PassManager.body
class PassManager:
    new = Constructor()

    add = Method(Void, (ptr(Pass, owned=True), 'Pass'))
    run = Method(Void, (ref(Module), 'Module'))
