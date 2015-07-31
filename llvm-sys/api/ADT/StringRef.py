from rust_bindgen.ast import *
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
        'llvm/ADT/StringRef.h',
        'llvm/ADT/Twine.h',
    }


class StringRefCTypeGenerator(CTypeGenerator):

    def generate_def(self, writer):
        data_tygen = self.typegen(self.data_type)

        struct_def = writer.gen.struct_def([
            (data_tygen.ffi_name, 'data'),
            ('size_t', 'length'),
        ])
        writer.typedef(self.ffi_name, struct_def)

    @property
    def data_type(self):
        from rust_bindgen.ast import Pointer, Char
        return Pointer(Char, const=True)

    @property
    def cpp_name(self):
        return 'llvm::StringRef'

    @property
    def ffi_name(self):
        return 'llvm_StringRef'

    @property
    def flat_name(self):
        return self.ffi_name

    def convert(self, out=False):
        def convert_in(writer, dest, expr):
            data = writer.gen.member(expr, 'data')
            length = writer.gen.member(expr, 'length')

            writer.writeln('%s %s(%s, %s);' %
                           (self.cpp_name, dest, data, length))

        def convert_out(writer, expr):
            data_meth = writer.gen.member(expr, 'data')
            data = writer.gen.call(data_meth)

            length_meth = writer.gen.member(expr, 'size')
            length = writer.gen.call(length_meth)

            return writer.gen.struct_init([
                ('data', data),
                ('length', length),
            ])

        if out:
            return TypeConvert('inline', convert_out)
        else:
            return TypeConvert('complex', convert_in)


class StringRefRustFFITypeGenerator(RustFFITypeGenerator):

    def generate_def(self, writer):
        data_tygen = self.typegen(self.data_type)
        size_tygen = self.typegen(SizeTy)

        writer.writeln()
        writer.attr('repr(C)')
        writer.struct_def(self.ffi_name(), [
            (data_tygen.ffi_name(), 'data', True),
            (size_tygen.ffi_name(), 'length', True),
        ], pub=True)

    @property
    def data_type(self):
        from rust_bindgen.ast import Pointer, Char
        return Pointer(Char, const=True)

    def proxy(self, root=[], out=False):
        from rust_bindgen.gen.rust.ffi.gen.ty import Proxy

        name = self.ffi_name(root) if out else '&str'

        size_ty = self.typegen(SizeTy)
        size_proxy = size_ty.proxy(root, out=out)

        def convert_in(writer, expr):
            data = '%s.as_ptr()' % (expr)
            data = 'unsafe { ::std::mem::transmute(%s) }' % (data)

            length = size_proxy(writer, '%s.len()' % (expr))

            return writer.gen.struct_init(self.ffi_name(root), [
                ('data', data),
                ('length', length),
            ])

        def convert_out(writer, expr):
            return expr

        return Proxy(name, convert_out if out else convert_in)

    def ffi_name(self, root=[]):
        name = 'llvm_StringRef'
        return self.gen_rust_name(root + [name])

    @property
    def flat_name(self):
        return self.ffi_name()


@register(C_TYPE_ENTRY, CBindingGenerator.LANG, StringRefCTypeGenerator)
@register(RUST_FFI_TYPE_ENTRY, RustBindingGenerator.LANG, StringRefRustFFITypeGenerator)
class _StringRef(Type):
    pass

StringRef = _StringRef()
