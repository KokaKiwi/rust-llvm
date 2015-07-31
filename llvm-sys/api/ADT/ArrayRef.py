from rust_bindgen.ast import *
from rust_bindgen.ast.visit import VisitorGenerator, TYPE_VISIT_ENTRY
from rust_bindgen.gen.c import CBindingGenerator
from rust_bindgen.gen.c.gen.ty import CTypeGenerator, TypeConvert
from rust_bindgen.gen.c.gen.ty import ENTRY as C_TYPE_ENTRY
from rust_bindgen.gen.rust import RustBindingGenerator
from rust_bindgen.gen.rust.ffi.gen.ty import RustFFITypeGenerator
from rust_bindgen.gen.rust.ffi.gen.ty import ENTRY as RUST_FFI_TYPE_ENTRY
from rust_bindgen.gen.registry import register
from ..ns import llvm


@llvm.body
class llvm:
    _includes_ = {
        'llvm/ADT/ArrayRef.h',
    }


class ArrayRefTypeVisitorGenerator(VisitorGenerator):

    def visit(self, ty):
        self.visitor.visit(ty.subtype)

        return super().visit(ty)


class ArrayRefCTypeGenerator(CTypeGenerator):

    def generate_def(self, writer):
        data_tygen = self.typegen(self.data_type)

        struct_def = writer.gen.struct_def([
            (data_tygen.ffi_name, 'data'),
            ('size_t', 'size'),
        ])
        writer.typedef(self.ffi_name, struct_def)

    @property
    def data_type(self):
        return Pointer(self.ty.subtype, const=True)

    @property
    def cpp_name(self):
        subgen = self.typegen(self.ty.subtype)
        return 'llvm::ArrayRef<%s>' % (subgen.cpp_name)

    @property
    def ffi_name(self):
        subgen = self.typegen(self.ty.subtype)
        return 'llvm_ArrayRef_%s' % (subgen.flat_name)

    @property
    def flat_name(self):
        return self.ffi_name

    def convert(self, out=False):
        def convert_in(writer, dest, expr):
            data = writer.gen.member(expr, 'data')
            size = writer.gen.member(expr, 'size')

            writer.writeln('%s %s(%s, %s);' %
                           (self.cpp_name, dest, data, size))

        def convert_out(writer, expr):
            data_meth = writer.gen.member(expr, 'data')
            data = writer.gen.call(data_meth)

            size_meth = writer.gen.member(expr, 'size')
            size = writer.gen.call(size_meth)

            return writer.gen.struct_init([
                ('data', data),
                ('size', size),
            ])

        if out:
            return TypeConvert('inline', convert_out)
        else:
            return TypeConvert('complex', convert_in)


class ArrayRefRustFFITypeGenerator(RustFFITypeGenerator):

    def generate_def(self, writer):
        data_tygen = self.typegen(self.data_type)
        size_tygen = self.typegen(SizeTy)

        writer.writeln()
        writer.attr('repr(C)')
        writer.struct_def(self.ffi_name(), [
            (data_tygen.ffi_name(), 'data', True),
            (size_tygen.ffi_name(), 'size', True),
        ], pub=True)

    @property
    def data_type(self):
        return Pointer(self.ty.subtype, const=True)

    def proxy(self, root=[], out=False):
        from rust_bindgen.gen.rust.ffi.gen.ty import Proxy

        subgen = self.typegen(self.ty.subtype)
        subproxy = subgen.proxy(root, out=out)

        data_tygen = self.typegen(self.data_type)
        data_proxy = data_tygen.proxy(root, out=out)

        size_tygen = self.typegen(SizeTy)
        size_proxy = size_tygen.proxy(root, out=out)

        name = '&[%s]' % (subgen.ffi_name(root))

        def convert_in(writer, expr):
            data = '%s.as_ptr()' % (expr)
            size = size_proxy(writer, '%s.len()' % (expr))

            return writer.gen.struct_init(self.ffi_name(root), [
                ('data', data),
                ('size', size),
            ])

        def convert_out(writer, expr):
            return expr

        return Proxy(name, convert_out if out else convert_in)

    def ffi_name(self, root=[]):
        subgen = self.typegen(self.ty.subtype)
        name = 'llvm_ArrayRef_%s' % (subgen.flat_name)
        return self.gen_rust_name(root + [name])

    @property
    def flat_name(self):
        return self.ffi_name()


@register(TYPE_VISIT_ENTRY, None, ArrayRefTypeVisitorGenerator)
@register(C_TYPE_ENTRY, CBindingGenerator.LANG, ArrayRefCTypeGenerator)
@register(RUST_FFI_TYPE_ENTRY, RustBindingGenerator.LANG, ArrayRefRustFFITypeGenerator)
class ArrayRef(Type):

    def __init__(self, subtype):
        super().__init__()

        self.subtype = subtype

    @property
    def tyname(self):
        return 'ArrayRef<%s>' % (self.subtype.tyname)

    def _hash(self):
        return hash(self.__class__) + hash(self.subtype)
