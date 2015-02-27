from bindgen.ast import *
from bindgen.ast.visit import VisitorGenerator, TYPE_VISIT_ENTRY
from bindgen.gen.c import CBindingGenerator
from bindgen.gen.c.gen.ty import CTypeGenerator, TypeConvert
from bindgen.gen.c.gen.ty import ENTRY as C_TYPE_ENTRY
from bindgen.gen.rust import RustBindingGenerator
from bindgen.gen.rust.ffi.gen.ty import RustFFITypeGenerator
from bindgen.gen.rust.ffi.gen.ty import ENTRY as RUST_FFI_TYPE_ENTRY
from bindgen.gen.registry import register
from .ArrayRef import ArrayRef
from ..ns import llvm

@llvm.body
class llvm:
    _includes_ = {
        'llvm/ADT/APInt.h',
    }

BigVal = ArrayRef(UnsignedInt64)

class APIntTypeVisitorGenerator(VisitorGenerator):
    def visit(self, ty):
        self.visitor.visit(BigVal)

        return super().visit(ty)

class APIntCTypeGenerator(CTypeGenerator):
    def generate_def(self, writer):
        data_tygen = self.typegen(BigVal)

        struct_def = writer.gen.struct_def([
            ('unsigned int', 'numbits'),
            (data_tygen.ffi_name, 'data'),
        ])
        writer.typedef(self.ffi_name, struct_def)

    @property
    def cpp_name(self):
        return 'llvm::APInt'

    @property
    def ffi_name(self):
        return 'llvm_APInt'

    @property
    def flat_name(self):
        return self.ffi_name

    def convert(self, out=False):
        data_tygen = self.typegen(BigVal)

        def convert_in(writer, dest, expr):
            convert = data_tygen.convert()

            numbits = writer.gen.member(expr, 'numbits')

            data = '__%s_data' % (dest)
            convert(writer, data, writer.gen.member(expr, 'data'))

            writer.writeln('%s %s(%s, %s);' % (self.cpp_name, dest, numbits, data))

        if out:
            raise NotImplementedError('APIntCTypeGenerator.convert (out)')
        else:
            return TypeConvert('complex', convert_in)

class APIntRustFFITypeGenerator(RustFFITypeGenerator):

    def generate_def(self, writer):
        data_tygen = self.typegen(BigVal)
        numbits_tygen = self.typegen(UnsignedInt)

        writer.writeln()
        writer.attr('repr(C)')
        writer.struct_def(self.ffi_name(), [
            (numbits_tygen.ffi_name(), 'numbits', True),
            (data_tygen.ffi_name(), 'data', True),
        ], pub=True)

    def proxy(self, root=[], out=False):
        from bindgen.gen.rust.ffi.gen.ty import Proxy

        data_tygen = self.typegen(BigVal)
        data_proxy = data_tygen.proxy(root, out=out)

        numbits_tygen = self.typegen(UnsignedInt)
        numbits_proxy = numbits_tygen.proxy(root, out=out)

        name = '(&[u64], usize)'

        def convert_in(writer, expr):
            data = data_proxy(writer, '%s.0' % (expr))
            numbits = numbits_proxy(writer, '%s.1' % (expr))

            return writer.gen.struct_init(self.ffi_name(root), [
                ('data', data),
                ('numbits', numbits),
            ])

        def convert_out(writer, expr):
            return expr

        return Proxy(name, convert_out if out else convert_in)

    def ffi_name(self, root=[]):
        name = 'llvm_APInt'
        return self.gen_rust_name(root + [name])

    @property
    def flat_name(self):
        return self.ffi_name()

@register(TYPE_VISIT_ENTRY, None, APIntTypeVisitorGenerator)
@register(C_TYPE_ENTRY, CBindingGenerator.LANG, APIntCTypeGenerator)
@register(RUST_FFI_TYPE_ENTRY, RustBindingGenerator.LANG, APIntRustFFITypeGenerator)
class _APInt(Type):
    pass

APInt = _APInt()
