#[doc = "Register `SPI_MEM_MOSI_DLEN` reader"]
pub struct R(crate::R<SPI_MEM_MOSI_DLEN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_MOSI_DLEN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_MOSI_DLEN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_MOSI_DLEN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_MOSI_DLEN` writer"]
pub struct W(crate::W<SPI_MEM_MOSI_DLEN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_MOSI_DLEN_SPEC>;
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
impl From<crate::W<SPI_MEM_MOSI_DLEN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_MOSI_DLEN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_USR_MOSI_DBITLEN` reader - The length in bits of write-data. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_MOSI_DBITLEN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `SPI_MEM_USR_MOSI_DBITLEN` writer - The length in bits of write-data. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_MOSI_DBITLEN_W<'a> =
    crate::FieldWriter<'a, u32, SPI_MEM_MOSI_DLEN_SPEC, u16, u16, 10, 0>;
impl R {
    #[doc = "Bits 0:9 - The length in bits of write-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_mosi_dbitlen(&self) -> SPI_MEM_USR_MOSI_DBITLEN_R {
        SPI_MEM_USR_MOSI_DBITLEN_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - The length in bits of write-data. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_mosi_dbitlen(&mut self) -> SPI_MEM_USR_MOSI_DBITLEN_W {
        SPI_MEM_USR_MOSI_DBITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 send data bit length control register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_mosi_dlen](index.html) module"]
pub struct SPI_MEM_MOSI_DLEN_SPEC;
impl crate::RegisterSpec for SPI_MEM_MOSI_DLEN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_mosi_dlen::R](R) reader structure"]
impl crate::Readable for SPI_MEM_MOSI_DLEN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_mosi_dlen::W](W) writer structure"]
impl crate::Writable for SPI_MEM_MOSI_DLEN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_MEM_MOSI_DLEN to value 0"]
impl crate::Resettable for SPI_MEM_MOSI_DLEN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
