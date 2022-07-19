#[doc = "Register `SPI_W3` reader"]
pub struct R(crate::R<SPI_W3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_W3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_W3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_W3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_W3` writer"]
pub struct W(crate::W<SPI_W3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_W3_SPEC>;
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
impl From<crate::W<SPI_W3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_W3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_BUF3` reader - data buffer"]
pub type SPI_BUF3_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SPI_BUF3` writer - data buffer"]
pub type SPI_BUF3_W<'a> = crate::FieldWriter<'a, u32, SPI_W3_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf3(&self) -> SPI_BUF3_R {
        SPI_BUF3_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - data buffer"]
    #[inline(always)]
    pub fn spi_buf3(&mut self) -> SPI_BUF3_W {
        SPI_BUF3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI CPU-controlled buffer3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_w3](index.html) module"]
pub struct SPI_W3_SPEC;
impl crate::RegisterSpec for SPI_W3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_w3::R](R) reader structure"]
impl crate::Readable for SPI_W3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_w3::W](W) writer structure"]
impl crate::Writable for SPI_W3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_W3 to value 0"]
impl crate::Resettable for SPI_W3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
