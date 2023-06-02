#[doc = "Register `IO_MUX_MTMS` reader"]
pub struct R(crate::R<IO_MUX_MTMS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IO_MUX_MTMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IO_MUX_MTMS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IO_MUX_MTMS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IO_MUX_MTMS` writer"]
pub struct W(crate::W<IO_MUX_MTMS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IO_MUX_MTMS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<IO_MUX_MTMS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IO_MUX_MTMS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `Register` reader - "]
pub type REGISTER_R = crate::FieldReader<u32, u32>;
#[doc = "Field `Register` writer - "]
pub type REGISTER_W<'a, const O: u8> = crate::FieldWriter<'a, IO_MUX_MTMS_SPEC, 32, O, u32, u32>;
#[doc = "Field `SLEEP_ENABLE` reader - configures output enable during sleep mode"]
pub type SLEEP_ENABLE_R = crate::BitReader;
#[doc = "Field `SLEEP_ENABLE` writer - configures output enable during sleep mode"]
pub type SLEEP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, IO_MUX_MTMS_SPEC, O>;
#[doc = "Field `SLEEP_PULLUP` reader - configures pull up during sleep mode"]
pub type SLEEP_PULLUP_R = crate::BitReader;
#[doc = "Field `SLEEP_PULLUP` writer - configures pull up during sleep mode"]
pub type SLEEP_PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, IO_MUX_MTMS_SPEC, O>;
#[doc = "Field `FUNCTION_SELECT_LOW_BITS` reader - configures IO_MUX function, bottom 2 bits"]
pub type FUNCTION_SELECT_LOW_BITS_R = crate::FieldReader;
#[doc = "Field `FUNCTION_SELECT_LOW_BITS` writer - configures IO_MUX function, bottom 2 bits"]
pub type FUNCTION_SELECT_LOW_BITS_W<'a, const O: u8> =
    crate::FieldWriter<'a, IO_MUX_MTMS_SPEC, 2, O>;
#[doc = "Field `PULLUP` reader - configures pull up"]
pub type PULLUP_R = crate::BitReader;
#[doc = "Field `PULLUP` writer - configures pull up"]
pub type PULLUP_W<'a, const O: u8> = crate::BitWriter<'a, IO_MUX_MTMS_SPEC, O>;
#[doc = "Field `FUNCTION_SELECT_HIGH_BIT` reader - configures IO_MUX function, upper bit"]
pub type FUNCTION_SELECT_HIGH_BIT_R = crate::BitReader;
#[doc = "Field `FUNCTION_SELECT_HIGH_BIT` writer - configures IO_MUX function, upper bit"]
pub type FUNCTION_SELECT_HIGH_BIT_W<'a, const O: u8> = crate::BitWriter<'a, IO_MUX_MTMS_SPEC, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn register(&self) -> REGISTER_R {
        REGISTER_R::new(self.bits)
    }
    #[doc = "Bit 0 - configures output enable during sleep mode"]
    #[inline(always)]
    pub fn sleep_enable(&self) -> SLEEP_ENABLE_R {
        SLEEP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - configures pull up during sleep mode"]
    #[inline(always)]
    pub fn sleep_pullup(&self) -> SLEEP_PULLUP_R {
        SLEEP_PULLUP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - configures IO_MUX function, bottom 2 bits"]
    #[inline(always)]
    pub fn function_select_low_bits(&self) -> FUNCTION_SELECT_LOW_BITS_R {
        FUNCTION_SELECT_LOW_BITS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - configures pull up"]
    #[inline(always)]
    pub fn pullup(&self) -> PULLUP_R {
        PULLUP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - configures IO_MUX function, upper bit"]
    #[inline(always)]
    pub fn function_select_high_bit(&self) -> FUNCTION_SELECT_HIGH_BIT_R {
        FUNCTION_SELECT_HIGH_BIT_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IO_MUX_MTMS")
            .field("register", &format_args!("{}", self.register().bits()))
            .field(
                "function_select_low_bits",
                &format_args!("{}", self.function_select_low_bits().bits()),
            )
            .field(
                "function_select_high_bit",
                &format_args!("{}", self.function_select_high_bit().bit()),
            )
            .field("pullup", &format_args!("{}", self.pullup().bit()))
            .field(
                "sleep_pullup",
                &format_args!("{}", self.sleep_pullup().bit()),
            )
            .field(
                "sleep_enable",
                &format_args!("{}", self.sleep_enable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<IO_MUX_MTMS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn register(&mut self) -> REGISTER_W<0> {
        REGISTER_W::new(self)
    }
    #[doc = "Bit 0 - configures output enable during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_enable(&mut self) -> SLEEP_ENABLE_W<0> {
        SLEEP_ENABLE_W::new(self)
    }
    #[doc = "Bit 3 - configures pull up during sleep mode"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_pullup(&mut self) -> SLEEP_PULLUP_W<3> {
        SLEEP_PULLUP_W::new(self)
    }
    #[doc = "Bits 4:5 - configures IO_MUX function, bottom 2 bits"]
    #[inline(always)]
    #[must_use]
    pub fn function_select_low_bits(&mut self) -> FUNCTION_SELECT_LOW_BITS_W<4> {
        FUNCTION_SELECT_LOW_BITS_W::new(self)
    }
    #[doc = "Bit 7 - configures pull up"]
    #[inline(always)]
    #[must_use]
    pub fn pullup(&mut self) -> PULLUP_W<7> {
        PULLUP_W::new(self)
    }
    #[doc = "Bit 8 - configures IO_MUX function, upper bit"]
    #[inline(always)]
    #[must_use]
    pub fn function_select_high_bit(&mut self) -> FUNCTION_SELECT_HIGH_BIT_W<8> {
        FUNCTION_SELECT_HIGH_BIT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IO_MUX_MTMS\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [io_mux_mtms](index.html) module"]
pub struct IO_MUX_MTMS_SPEC;
impl crate::RegisterSpec for IO_MUX_MTMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [io_mux_mtms::R](R) reader structure"]
impl crate::Readable for IO_MUX_MTMS_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [io_mux_mtms::W](W) writer structure"]
impl crate::Writable for IO_MUX_MTMS_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets IO_MUX_MTMS to value 0"]
impl crate::Resettable for IO_MUX_MTMS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
