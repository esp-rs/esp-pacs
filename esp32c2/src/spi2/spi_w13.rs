#[doc = "Register `SPI_W13` reader"]
pub struct R(crate::R<SPI_W13_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_W13_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_W13_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_W13_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_W13` writer"]
pub struct W(crate::W<SPI_W13_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_W13_SPEC>;
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
impl From<crate::W<SPI_W13_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_W13_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_BUF13` reader - data buffer"]
pub type SPI_BUF13_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI_BUF13` writer - data buffer"]
pub type SPI_BUF13_W<'a> = crate::FieldWriter<'a, u32, SPI_W13_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf13(&self) -> SPI_BUF13_R {
        SPI_BUF13_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf13(&mut self) -> SPI_BUF13_W {
        SPI_BUF13_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer13\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w13](index.html) module"]
pub struct SPI_W13_SPEC;
impl crate::RegisterSpec for SPI_W13_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_w13::R](R) reader structure"]
impl crate::Readable for SPI_W13_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_w13::W](W) writer structure"]
impl crate::Writable for SPI_W13_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_W13 to value 0"]
impl crate::Resettable for SPI_W13_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
