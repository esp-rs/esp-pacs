#[doc = "Register `SPI_MEM_REJECT_ADDR` reader"]
pub struct R(crate::R<SPI_MEM_REJECT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_REJECT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_REJECT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_REJECT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SPI_MEM_REJECT_ADDR` reader - ******* Description ***********"]
pub type SPI_MEM_REJECT_ADDR_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - ******* Description ***********"]
    #[inline(always)]
    pub fn spi_mem_reject_addr(&self) -> SPI_MEM_REJECT_ADDR_R {
        SPI_MEM_REJECT_ADDR_R::new(self.bits)
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_reject_addr](index.html) module"]
pub struct SPI_MEM_REJECT_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_REJECT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_reject_addr::R](R) reader structure"]
impl crate::Readable for SPI_MEM_REJECT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_REJECT_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_REJECT_ADDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
