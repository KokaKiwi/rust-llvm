from .prelude import *
from bindgen.gen.c.gen.func import raw_function as c_raw_function
from bindgen.gen.c.gen.func import MethodGenerator as CMethodGenerator

class PrevCMethodGenerator(CMethodGenerator):
    def generate_call(self, writer, ctx):
        const = self.func.const
        iterator_ty = 'const_arg_iterator' if const else 'arg_iterator'

        inst = ctx.args[0]

        writer.declare_var('llvm::Function::%s' % (iterator_ty), 'it', init=inst)
        writer.ret('++it')

class NextCMethodGenerator(CMethodGenerator):
    def generate_call(self, writer, ctx):
        const = self.func.const
        iterator_ty = 'const_arg_iterator' if const else 'arg_iterator'

        inst = ctx.args[0]

        writer.declare_var('llvm::Function::%s' % (iterator_ty), 'it', init=inst)
        writer.ret('--it')

@Argument.body
class Argument:
    new = Constructor((ptr(Type), 'Ty'), (OptionString(const=True), 'Name'), (OptionPointer(Function), 'F'))

    next = Method(ref(Argument, const=True), const=True)
    nextMut = Method(ref(Argument))

    c_raw_function(next)(PrevCMethodGenerator)
    c_raw_function(nextMut)(PrevCMethodGenerator)

    prev = Method(ref(Argument, const=True), const=True)
    prevMut = Method(ref(Argument))

    c_raw_function(prev)(PrevCMethodGenerator)
    c_raw_function(prevMut)(PrevCMethodGenerator)

    getParent = Method(ptr(Function, const=True), const=True)
    getParentMut = Method(ptr(Function)).with_real_name('getParent')

    getArgNo = Method(UnsignedInt, const=True)
