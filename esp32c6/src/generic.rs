use core::marker;
#[doc = " Raw register type (`u8`, `u16`, `u32`, ...)"]
pub trait RawReg:
    Copy
    + Default
    + From<bool>
    + core::ops::BitOr<Output = Self>
    + core::ops::BitAnd<Output = Self>
    + core::ops::BitOrAssign
    + core::ops::BitAndAssign
    + core::ops::Not<Output = Self>
    + core::ops::Shl<u8, Output = Self>
{
    #[doc = " Mask for bits of width `WI`"]
    fn mask<const WI: u8>() -> Self;
    #[doc = " Mask for bits of width 1"]
    fn one() -> Self;
}
macro_rules! raw_reg {
    ($ U : ty , $ size : literal , $ mask : ident) => {
        impl RawReg for $U {
            #[inline(always)]
            fn mask<const WI: u8>() -> Self {
                $mask::<WI>()
            }
            #[inline(always)]
            fn one() -> Self {
                1
            }
        }
        const fn $mask<const WI: u8>() -> $U {
            <$U>::MAX >> ($size - WI)
        }
    };
}
raw_reg!(u8, 8, mask_u8);
raw_reg!(u16, 16, mask_u16);
raw_reg!(u32, 32, mask_u32);
raw_reg!(u64, 64, mask_u64);
#[doc = " Raw register type"]
pub trait RegisterSpec {
    #[doc = " Raw register type (`u8`, `u16`, `u32`, ...)."]
    type Ux: RawReg;
}
#[doc = " Trait implemented by readable registers to enable the `read` method."]
#[doc = ""]
#[doc = " Registers marked with `Writable` can be also be `modify`'ed."]
pub trait Readable: RegisterSpec {
    #[doc = " Result from a call to `read` and argument to `modify`."]
    type Reader: From<R<Self>> + core::ops::Deref<Target = R<Self>>;
}
#[doc = " Trait implemented by writeable registers."]
#[doc = ""]
#[doc = " This enables the  `write`, `write_with_zero` and `reset` methods."]
#[doc = ""]
#[doc = " Registers marked with `Readable` can be also be `modify`'ed."]
pub trait Writable: RegisterSpec {
    #[doc = " Writer type argument to `write`, et al."]
    type Writer: From<W<Self>> + core::ops::DerefMut<Target = W<Self>>;
    #[doc = " Specifies the register bits that are not changed if you pass `1` and are changed if you pass `0`"]
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux;
    #[doc = " Specifies the register bits that are not changed if you pass `0` and are changed if you pass `1`"]
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux;
}
#[doc = " Reset value of the register."]
#[doc = ""]
#[doc = " This value is the initial value for the `write` method. It can also be directly written to the"]
#[doc = " register by using the `reset` method."]
pub trait Resettable: RegisterSpec {
    #[doc = " Reset value of the register."]
    const RESET_VALUE: Self::Ux;
    #[doc = " Reset value of the register."]
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        Self::RESET_VALUE
    }
}
#[doc = " This structure provides volatile access to registers."]
#[repr(transparent)]
pub struct Reg<REG: RegisterSpec> {
    register: vcell::VolatileCell<REG::Ux>,
    _marker: marker::PhantomData<REG>,
}
unsafe impl<REG: RegisterSpec> Send for Reg<REG> where REG::Ux: Send {}
impl<REG: RegisterSpec> Reg<REG> {
    #[doc = " Returns the underlying memory address of register."]
    #[doc = ""]
    #[doc = " ```ignore"]
    #[doc = " let reg_ptr = periph.reg.as_ptr();"]
    #[doc = " ```"]
    #[inline(always)]
    pub fn as_ptr(&self) -> *mut REG::Ux {
        self.register.as_ptr()
    }
}
impl<REG: Readable> Reg<REG> {
    #[doc = " Reads the contents of a `Readable` register."]
    #[doc = ""]
    #[doc = " You can read the raw contents of a register by using `bits`:"]
    #[doc = " ```ignore"]
    #[doc = " let bits = periph.reg.read().bits();"]
    #[doc = " ```"]
    #[doc = " or get the content of a particular field of a register:"]
    #[doc = " ```ignore"]
    #[doc = " let reader = periph.reg.read();"]
    #[doc = " let bits = reader.field1().bits();"]
    #[doc = " let flag = reader.field2().bit_is_set();"]
    #[doc = " ```"]
    #[inline(always)]
    pub fn read(&self) -> REG::Reader {
        REG::Reader::from(R {
            bits: self.register.get(),
            _reg: marker::PhantomData,
        })
    }
}
impl<REG: Resettable + Writable> Reg<REG> {
    #[doc = " Writes the reset value to `Writable` register."]
    #[doc = ""]
    #[doc = " Resets the register to its initial state."]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(REG::RESET_VALUE)
    }
    #[doc = " Writes bits to a `Writable` register."]
    #[doc = ""]
    #[doc = " You can write raw bits into a register:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| unsafe { w.bits(rawbits) });"]
    #[doc = " ```"]
    #[doc = " or write only the fields you need:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| w"]
    #[doc = "     .field1().bits(newfield1bits)"]
    #[doc = "     .field2().set_bit()"]
    #[doc = "     .field3().variant(VARIANT)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[doc = " or an alternative way of saying the same:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.write(|w| {"]
    #[doc = "     w.field1().bits(newfield1bits);"]
    #[doc = "     w.field2().set_bit();"]
    #[doc = "     w.field3().variant(VARIANT)"]
    #[doc = " });"]
    #[doc = " ```"]
    #[doc = " In the latter case, other fields will be set to their reset value."]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            f(&mut REG::Writer::from(W {
                bits: REG::RESET_VALUE & !REG::ONE_TO_MODIFY_FIELDS_BITMAP
                    | REG::ZERO_TO_MODIFY_FIELDS_BITMAP,
                _reg: marker::PhantomData,
            }))
            .bits,
        );
    }
}
impl<REG: Writable> Reg<REG> {
    #[doc = " Writes 0 to a `Writable` register."]
    #[doc = ""]
    #[doc = " Similar to `write`, but unused bits will contain 0."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " Unsafe to use with registers which don't allow to write 0."]
    #[inline(always)]
    pub unsafe fn write_with_zero<F>(&self, f: F)
    where
        F: FnOnce(&mut REG::Writer) -> &mut W<REG>,
    {
        self.register.set(
            f(&mut REG::Writer::from(W {
                bits: REG::Ux::default(),
                _reg: marker::PhantomData,
            }))
            .bits,
        );
    }
}
impl<REG: Readable + Writable> Reg<REG> {
    #[doc = " Modifies the contents of the register by reading and then writing it."]
    #[doc = ""]
    #[doc = " E.g. to do a read-modify-write sequence to change parts of a register:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|r, w| unsafe { w.bits("]
    #[doc = "    r.bits() | 3"]
    #[doc = " ) });"]
    #[doc = " ```"]
    #[doc = " or"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|_, w| w"]
    #[doc = "     .field1().bits(newfield1bits)"]
    #[doc = "     .field2().set_bit()"]
    #[doc = "     .field3().variant(VARIANT)"]
    #[doc = " );"]
    #[doc = " ```"]
    #[doc = " or an alternative way of saying the same:"]
    #[doc = " ```ignore"]
    #[doc = " periph.reg.modify(|_, w| {"]
    #[doc = "     w.field1().bits(newfield1bits);"]
    #[doc = "     w.field2().set_bit();"]
    #[doc = "     w.field3().variant(VARIANT)"]
    #[doc = " });"]
    #[doc = " ```"]
    #[doc = " Other fields will have the value they had before the call to `modify`."]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&REG::Reader, &'w mut REG::Writer) -> &'w mut W<REG>,
    {
        let bits = self.register.get();
        self.register.set(
            f(
                &REG::Reader::from(R {
                    bits,
                    _reg: marker::PhantomData,
                }),
                &mut REG::Writer::from(W {
                    bits: bits & !REG::ONE_TO_MODIFY_FIELDS_BITMAP
                        | REG::ZERO_TO_MODIFY_FIELDS_BITMAP,
                    _reg: marker::PhantomData,
                }),
            )
            .bits,
        );
    }
}
#[doc = " Register reader."]
#[doc = ""]
#[doc = " Result of the `read` methods of registers. Also used as a closure argument in the `modify`"]
#[doc = " method."]
pub struct R<REG: RegisterSpec + ?Sized> {
    pub(crate) bits: REG::Ux,
    _reg: marker::PhantomData<REG>,
}
impl<REG: RegisterSpec> R<REG> {
    #[doc = " Reads raw bits from register."]
    #[inline(always)]
    pub fn bits(&self) -> REG::Ux {
        self.bits
    }
}
impl<REG: RegisterSpec, FI> PartialEq<FI> for R<REG>
where
    REG::Ux: PartialEq,
    FI: Copy,
    REG::Ux: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&REG::Ux::from(*other))
    }
}
#[doc = " Register writer."]
#[doc = ""]
#[doc = " Used as an argument to the closures in the `write` and `modify` methods of the register."]
pub struct W<REG: RegisterSpec + ?Sized> {
    #[doc = "Writable bits"]
    pub(crate) bits: REG::Ux,
    _reg: marker::PhantomData<REG>,
}
impl<REG: RegisterSpec> W<REG> {
    #[doc = " Writes raw bits to the register."]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " Read datasheet or reference manual to find what values are allowed to pass."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: REG::Ux) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc(hidden)]
pub struct FieldReaderRaw<U, FI> {
    pub(crate) bits: U,
    _reg: marker::PhantomData<FI>,
}
impl<U, FI> FieldReaderRaw<U, FI>
where
    U: Copy,
{
    #[doc = " Creates a new instance of the reader."]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(bits: U) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}
#[doc(hidden)]
pub struct BitReaderRaw<FI> {
    pub(crate) bits: bool,
    _reg: marker::PhantomData<FI>,
}
impl<FI> BitReaderRaw<FI> {
    #[doc = " Creates a new instance of the reader."]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(bits: bool) -> Self {
        Self {
            bits,
            _reg: marker::PhantomData,
        }
    }
}
#[doc = " Field reader."]
#[doc = ""]
#[doc = " Result of the `read` methods of fields."]
pub type FieldReader<N = u8, FI = u8> = FieldReaderRaw<N, FI>;
#[doc = " Bit-wise field reader"]
pub type BitReader<FI = bool> = BitReaderRaw<FI>;
impl<N, FI> FieldReader<N, FI>
where
    N: Copy,
{
    #[doc = " Reads raw bits from field."]
    #[inline(always)]
    pub fn bits(&self) -> N {
        self.bits
    }
}
impl<N, FI> PartialEq<FI> for FieldReader<N, FI>
where
    FI: Copy,
    N: PartialEq + From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&N::from(*other))
    }
}
impl<FI> PartialEq<FI> for BitReader<FI>
where
    FI: Copy,
    bool: From<FI>,
{
    #[inline(always)]
    fn eq(&self, other: &FI) -> bool {
        self.bits.eq(&bool::from(*other))
    }
}
impl<FI> BitReader<FI> {
    #[doc = " Value of the field as raw bits."]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = " Returns `true` if the bit is clear (0)."]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = " Returns `true` if the bit is set (1)."]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc(hidden)]
pub struct Safe;
#[doc(hidden)]
pub struct Unsafe;
#[doc(hidden)]
pub struct FieldWriterRaw<'a, REG, const WI: u8, const O: u8, N, FI, Safety>
where
    REG: Writable + RegisterSpec,
    N: From<FI>,
{
    pub(crate) w: &'a mut REG::Writer,
    _field: marker::PhantomData<(N, FI, Safety)>,
}
impl<'a, REG, const WI: u8, const O: u8, N, FI, Safety>
    FieldWriterRaw<'a, REG, WI, O, N, FI, Safety>
where
    REG: Writable + RegisterSpec,
    N: From<FI>,
{
    #[doc = " Creates a new instance of the writer"]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut REG::Writer) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}
#[doc(hidden)]
pub struct BitWriterRaw<'a, REG, const O: u8, FI, M>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    pub(crate) w: &'a mut REG::Writer,
    _field: marker::PhantomData<(FI, M)>,
}
impl<'a, REG, const O: u8, FI, M> BitWriterRaw<'a, REG, O, FI, M>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = " Creates a new instance of the writer"]
    #[allow(unused)]
    #[inline(always)]
    pub(crate) fn new(w: &'a mut REG::Writer) -> Self {
        Self {
            w,
            _field: marker::PhantomData,
        }
    }
}
#[doc = " Write field Proxy with unsafe `bits`"]
pub type FieldWriter<'a, REG, const WI: u8, const O: u8, N = u8, FI = u8> =
    FieldWriterRaw<'a, REG, WI, O, N, FI, Unsafe>;
#[doc = " Write field Proxy with safe `bits`"]
pub type FieldWriterSafe<'a, REG, const WI: u8, const O: u8, N = u8, FI = u8> =
    FieldWriterRaw<'a, REG, WI, O, N, FI, Safe>;
impl<'a, REG, const WI: u8, const OF: u8, N, FI> FieldWriter<'a, REG, WI, OF, N, FI>
where
    REG: Writable + RegisterSpec,
    N: From<FI>,
{
    #[doc = " Field width"]
    pub const WIDTH: u8 = WI;
}
impl<'a, REG, const WI: u8, const OF: u8, N, FI> FieldWriterSafe<'a, REG, WI, OF, N, FI>
where
    REG: Writable + RegisterSpec,
    N: From<FI>,
{
    #[doc = " Field width"]
    pub const WIDTH: u8 = WI;
}
macro_rules! bit_proxy {
    ($ writer : ident , $ mwv : ident) => {
        #[doc(hidden)]
        pub struct $mwv;
        #[doc = " Bit-wise write field proxy"]
        pub type $writer<'a, REG, const O: u8, FI = bool> = BitWriterRaw<'a, REG, O, FI, $mwv>;
        impl<'a, REG, const OF: u8, FI> $writer<'a, REG, OF, FI>
        where
            REG: Writable + RegisterSpec,
            bool: From<FI>,
        {
            #[doc = " Field width"]
            pub const WIDTH: u8 = 1;
        }
    };
}
macro_rules! impl_bit_proxy {
    ($ writer : ident) => {
        impl<'a, REG, const OF: u8, FI> $writer<'a, REG, OF, FI>
        where
            REG: Writable + RegisterSpec,
            bool: From<FI>,
        {
            #[doc = " Writes bit to the field"]
            #[inline(always)]
            pub fn bit(self, value: bool) -> &'a mut REG::Writer {
                self.w.bits &= !(REG::Ux::one() << OF);
                self.w.bits |= (REG::Ux::from(value) & REG::Ux::one()) << OF;
                self.w
            }
            #[doc = " Writes `variant` to the field"]
            #[inline(always)]
            pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
                self.bit(bool::from(variant))
            }
        }
    };
}
bit_proxy!(BitWriter, BitM);
bit_proxy!(BitWriter1S, Bit1S);
bit_proxy!(BitWriter0C, Bit0C);
bit_proxy!(BitWriter1C, Bit1C);
bit_proxy!(BitWriter0S, Bit0S);
bit_proxy!(BitWriter1T, Bit1T);
bit_proxy!(BitWriter0T, Bit0T);
impl<'a, REG, const WI: u8, const OF: u8, N, FI> FieldWriter<'a, REG, WI, OF, N, FI>
where
    REG: Writable + RegisterSpec,
    REG::Ux: From<N>,
    N: From<FI>,
{
    #[doc = " Writes raw bits to the field"]
    #[doc = ""]
    #[doc = " # Safety"]
    #[doc = ""]
    #[doc = " Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(self, value: N) -> &'a mut REG::Writer {
        self.w.bits &= !(REG::Ux::mask::<WI>() << OF);
        self.w.bits |= (REG::Ux::from(value) & REG::Ux::mask::<WI>()) << OF;
        self.w
    }
    #[doc = " Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
        unsafe { self.bits(N::from(variant)) }
    }
}
impl<'a, REG, const WI: u8, const OF: u8, N, FI> FieldWriterSafe<'a, REG, WI, OF, N, FI>
where
    REG: Writable + RegisterSpec,
    REG::Ux: From<N>,
    N: From<FI>,
{
    #[doc = " Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: N) -> &'a mut REG::Writer {
        self.w.bits &= !(REG::Ux::mask::<WI>() << OF);
        self.w.bits |= (REG::Ux::from(value) & REG::Ux::mask::<WI>()) << OF;
        self.w
    }
    #[doc = " Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FI) -> &'a mut REG::Writer {
        self.bits(N::from(variant))
    }
}
impl_bit_proxy!(BitWriter);
impl_bit_proxy!(BitWriter1S);
impl_bit_proxy!(BitWriter0C);
impl_bit_proxy!(BitWriter1C);
impl_bit_proxy!(BitWriter0S);
impl_bit_proxy!(BitWriter1T);
impl_bit_proxy!(BitWriter0T);
impl<'a, REG, const OF: u8, FI> BitWriter<'a, REG, OF, FI>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = " Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= REG::Ux::one() << OF;
        self.w
    }
    #[doc = " Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(REG::Ux::one() << OF);
        self.w
    }
}
impl<'a, REG, const OF: u8, FI> BitWriter1S<'a, REG, OF, FI>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = " Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= REG::Ux::one() << OF;
        self.w
    }
}
impl<'a, REG, const OF: u8, FI> BitWriter0C<'a, REG, OF, FI>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = " Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(REG::Ux::one() << OF);
        self.w
    }
}
impl<'a, REG, const OF: u8, FI> BitWriter1C<'a, REG, OF, FI>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = "Clears the field bit by passing one"]
    #[inline(always)]
    pub fn clear_bit_by_one(self) -> &'a mut REG::Writer {
        self.w.bits |= REG::Ux::one() << OF;
        self.w
    }
}
impl<'a, REG, const OF: u8, FI> BitWriter0S<'a, REG, OF, FI>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = "Sets the field bit by passing zero"]
    #[inline(always)]
    pub fn set_bit_by_zero(self) -> &'a mut REG::Writer {
        self.w.bits &= !(REG::Ux::one() << OF);
        self.w
    }
}
impl<'a, REG, const OF: u8, FI> BitWriter1T<'a, REG, OF, FI>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = "Toggle the field bit by passing one"]
    #[inline(always)]
    pub fn toggle_bit(self) -> &'a mut REG::Writer {
        self.w.bits |= REG::Ux::one() << OF;
        self.w
    }
}
impl<'a, REG, const OF: u8, FI> BitWriter0T<'a, REG, OF, FI>
where
    REG: Writable + RegisterSpec,
    bool: From<FI>,
{
    #[doc = "Toggle the field bit by passing zero"]
    #[inline(always)]
    pub fn toggle_bit(self) -> &'a mut REG::Writer {
        self.w.bits &= !(REG::Ux::one() << OF);
        self.w
    }
}
#[doc = " Access an array of `COUNT` items of type `T` with the items `STRIDE` bytes"]
#[doc = " apart.  This is a zero-sized-type.  No objects of this type are ever"]
#[doc = " actually created, it is only a convenience for wrapping pointer arithmetic."]
#[doc = ""]
#[doc = " There is no safe way to produce items of this type.  Unsafe code can produce"]
#[doc = " references by pointer casting.  It is up to the unsafe code doing that, to"]
#[doc = " ensure that the memory really is backed by appropriate content."]
#[doc = ""]
#[doc = " Typically, this is used for accessing hardware registers."]
pub struct ArrayProxy<T, const COUNT: usize, const STRIDE: usize> {
    #[doc = " As well as providing a PhantomData, this field is non-public, and"]
    #[doc = " therefore ensures that code outside of this module can never create"]
    #[doc = " an ArrayProxy."]
    _array: marker::PhantomData<T>,
}
#[allow(clippy::len_without_is_empty)]
impl<T, const C: usize, const S: usize> ArrayProxy<T, C, S> {
    #[doc = " Get a reference from an [ArrayProxy] with no bounds checking."]
    pub unsafe fn get_ref(&self, index: usize) -> &T {
        let base = self as *const Self as usize;
        let address = base + S * index;
        &*(address as *const T)
    }
    #[doc = " Get a reference from an [ArrayProxy], or return `None` if the index"]
    #[doc = " is out of bounds."]
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < C {
            Some(unsafe { self.get_ref(index) })
        } else {
            None
        }
    }
    #[doc = " Return the number of items."]
    pub fn len(&self) -> usize {
        C
    }
}
impl<T, const C: usize, const S: usize> core::ops::Index<usize> for ArrayProxy<T, C, S> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        #[allow(clippy::no_effect)]
        [(); C][index];
        unsafe { self.get_ref(index) }
    }
}
