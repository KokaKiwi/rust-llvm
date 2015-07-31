from .prelude import *
from rust_bindgen.ast.visit import VisitorGenerator, TYPE_VISIT_ENTRY
from rust_bindgen.gen.c import CBindingGenerator
from rust_bindgen.gen.c.gen.ty import ClassTypeGenerator as CClassTypeGenerator
from rust_bindgen.gen.c.gen.ty import ENTRY as C_TYPE_ENTRY
from rust_bindgen.gen.registry import register


class iplistTypeVisitorGenerator(VisitorGenerator):

    def visit(self, ty):
        self.visitor.visit(ty.subtype)

        return super().visit(ty)


class iplistCClassTypeGenerator(CClassTypeGenerator):

    @property
    def cpp_name(self):
        subgen = self.typegen(self.ty.subtype)

        real_path = self.ty.real_path
        real_path[-1] = 'iplist<%s>' % (subgen.cpp_name)

        return self.gen_cpp_name(real_path)


@register(TYPE_VISIT_ENTRY, None, iplistTypeVisitorGenerator)
@register(C_TYPE_ENTRY, CBindingGenerator.LANG, iplistCClassTypeGenerator)
class iplist(Class):

    def __init__(self, subtype):
        super().__init__('iplist_%s' % (subtype.tyname))

        llvm.add_item(self)
        self.subtype = subtype

        @self.body
        class body:
            new = Constructor()
            delete = Destructor()

            first = Method(
                ref(self.subtype, const=True), const=True).with_real_name('front')
            firstMut = Method(ref(self.subtype)).with_real_name('front')

            last = Method(
                ref(self.subtype, const=True), const=True).with_real_name('back')
            lastMut = Method(ref(self.subtype)).with_real_name('back')

            max_size = Method(SizeTy, const=True)
            size = Method(SizeTy, const=True)

            clear = Method()

    def _hash(self):
        return super()._hash() + hash(self.subtype)
