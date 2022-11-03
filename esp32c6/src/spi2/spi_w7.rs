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
#[doc = "Field `SPI_BUF7` reader - data buffer"]
pub type SPI_BUF7_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI_BUF7` writer - data buffer"]
pub type SPI_BUF7_W<'a, const O: u8> = crate::FieldWriter<'a, u32, SPI_W7_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf7(&self) -> SPI_BUF7_R {
        SPI_BUF7_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    #[must_use]
    pub fn spi_buf7(&mut self) -> SPI_BUF7_W<0> {
        SPI_BUF7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer7\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w7](index.html) module"]
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
