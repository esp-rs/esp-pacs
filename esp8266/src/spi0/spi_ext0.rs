#[doc = "Register `SPI_EXT0` reader"]
pub struct R(crate::R<SPI_EXT0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_EXT0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_EXT0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_EXT0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_EXT0` writer"]
pub struct W(crate::W<SPI_EXT0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_EXT0_SPEC>;
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
impl From<crate::W<SPI_EXT0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_EXT0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `pp_time` reader - "]
pub type PP_TIME_R = crate::FieldReader<u16>;
#[doc = "Field `pp_time` writer - "]
pub type PP_TIME_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_EXT0_SPEC, 12, O, u16>;
#[doc = "Field `pp_shift` reader - "]
pub type PP_SHIFT_R = crate::FieldReader;
#[doc = "Field `pp_shift` writer - "]
pub type PP_SHIFT_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_EXT0_SPEC, 4, O>;
#[doc = "Field `pp_enable` reader - "]
pub type PP_ENABLE_R = crate::BitReader;
#[doc = "Field `pp_enable` writer - "]
pub type PP_ENABLE_W<'a, const O: u8> = crate::BitWriter<'a, SPI_EXT0_SPEC, O>;
impl R {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    pub fn pp_time(&self) -> PP_TIME_R {
        PP_TIME_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    pub fn pp_shift(&self) -> PP_SHIFT_R {
        PP_SHIFT_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn pp_enable(&self) -> PP_ENABLE_R {
        PP_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_EXT0")
            .field("pp_enable", &format_args!("{}", self.pp_enable().bit()))
            .field("pp_shift", &format_args!("{}", self.pp_shift().bits()))
            .field("pp_time", &format_args!("{}", self.pp_time().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_EXT0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11"]
    #[inline(always)]
    #[must_use]
    pub fn pp_time(&mut self) -> PP_TIME_W<0> {
        PP_TIME_W::new(self)
    }
    #[doc = "Bits 16:19"]
    #[inline(always)]
    #[must_use]
    pub fn pp_shift(&mut self) -> PP_SHIFT_W<16> {
        PP_SHIFT_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    #[must_use]
    pub fn pp_enable(&mut self) -> PP_ENABLE_W<31> {
        PP_ENABLE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_ext0](index.html) module"]
pub struct SPI_EXT0_SPEC;
impl crate::RegisterSpec for SPI_EXT0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_ext0::R](R) reader structure"]
impl crate::Readable for SPI_EXT0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_ext0::W](W) writer structure"]
impl crate::Writable for SPI_EXT0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_EXT0 to value 0"]
impl crate::Resettable for SPI_EXT0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
