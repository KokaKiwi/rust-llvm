from bindgen.ast.objects import *
from .ArrayRef import ArrayRef
from ..ns import llvm

BigVal = ArrayRef(UnsignedInt64)

@llvm.body
class llvm_body:
    _includes_ = ['llvm/ADT/APInt.h']
    _types_ = [BigVal]

class _APInt(ConvertibleType):
    def write_def(self, lang, writer):
        if lang == 'rust':
            writer.attr('repr', ['C'])
            writer.attr('derive', ['Copy'])

        writer.struct(members=[
            (UnsignedInt.ffi_name(lang), 'num_bits'),
            (BigVal.ffi_name(lang), 'value'),
        ], name=self.flat_name())

    def flat_name(self):
        return 'llvm_APInt'

    def ffi_name(self, lang, **kwargs):
        path = kwargs.get('path', []) + [self.flat_name()]
        return '::'.join(path)

    def lib_name(self, lang, **kwargs):
        if lang == 'rust':
            return '(u32, %s)' % (BigVal.lib_name(lang, **kwargs))

        return super().lib_name(lang, **kwargs)

    def convert_from_ffi(self, writer, lang, expr, **kwargs):
        if lang == 'c':
            value = writer.gen.member(expr, 'value')
            value = BigVal.convert_from_ffi(writer, lang, value, **kwargs)

            args = [writer.gen.member(expr, 'num_bits'), value]
            return writer.gen.call('::llvm::APInt', args)
        elif lang == 'rust':
            return expr

        return super().convert_from_ffi(writer, lang, expr, **kwargs)

    def convert_to_ffi(self, writer, lang, expr, **kwargs):
        if lang == 'c':
            return expr
        elif lang == 'rust':
            struct_name = '::ffi::%s' % (self.ffi_name(lang))
            num_bits = writer.gen.member(expr, '0')
            value = writer.gen.member(expr, '1')
            value = BigVal.convert_to_ffi(writer, lang, value, **kwargs)
            return writer.gen.init_struct(struct_name, [
                ('num_bits', writer.gen.cast(num_bits, UnsignedInt.ffi_name(lang))),
                ('value', value),
            ])

        return super().convert_to_ffi(writer, lang, expr, **kwargs)

APInt = _APInt()
