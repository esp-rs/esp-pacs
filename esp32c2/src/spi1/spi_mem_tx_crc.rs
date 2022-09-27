#[doc = "Register `SPI_MEM_TX_CRC` reader"]
pub struct R(crate::R<SPI_MEM_TX_CRC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_TX_CRC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_TX_CRC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_TX_CRC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DATA` reader - For SPI1, the value of crc32."]
pub type DATA_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - For SPI1, the value of crc32."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[doc = "SPI1 TX CRC data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_tx_crc](index.html) module"]
pub struct SPI_MEM_TX_CRC_SPEC;
impl crate::RegisterSpec for SPI_MEM_TX_CRC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_tx_crc::R](R) reader structure"]
impl crate::Readable for SPI_MEM_TX_CRC_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SPI_MEM_TX_CRC to value 0xffff_ffff"]
impl crate::Resettable for SPI_MEM_TX_CRC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff_ffff
    }
}
