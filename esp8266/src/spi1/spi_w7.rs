#[doc = "Register `SPI_W7` reader"]
pub struct R(crate::R<SPI_W7_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_W7_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_W7_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_W7_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_W7` writer"]
pub struct W(crate::W<SPI_W7_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_W7_SPEC>;
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
impl From<crate::W<SPI_W7_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_W7_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `spi_w7` reader - the data inside the buffer of the SPI module, word 7"]
pub type SPI_W7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `spi_w7` writer - the data inside the buffer of the SPI module, word 7"]
pub type SPI_W7_W<'a, const O: u8> = crate::FieldWriter<'a, SPI_W7_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 7"]
    #[inline(always)]
    pub fn spi_w7(&self) -> SPI_W7_R {
        SPI_W7_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_W7")
            .field("spi_w7", &format_args!("{}", self.spi_w7().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_W7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - the data inside the buffer of the SPI module, word 7"]
    #[inline(always)]
    #[must_use]
    pub fn spi_w7(&mut self) -> SPI_W7_W<0> {
        SPI_W7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "the data inside the buffer of the SPI module, word 7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w7](index.html) module"]
pub struct SPI_W7_SPEC;
impl crate::RegisterSpec for SPI_W7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_w7::R](R) reader structure"]
impl crate::Readable for SPI_W7_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_w7::W](W) writer structure"]
impl crate::Writable for SPI_W7_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_W7 to value 0"]
impl crate::Resettable for SPI_W7_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
