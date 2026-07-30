#[doc = "Register `SPI_MEM_PMS_REJECT_ADDR` reader"]
pub type R = crate::R<SPI_MEM_PMS_REJECT_ADDR_SPEC>;
#[doc = "Field `SPI_MEM_REJECT_ADDR` reader - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
pub type SPI_MEM_REJECT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - This bits show the first SPI1 access error address. It is cleared by when SPI_MEM_PMS_REJECT_INT_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_reject_addr(&self) -> SPI_MEM_REJECT_ADDR_R {
        SPI_MEM_REJECT_ADDR_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_PMS_REJECT_ADDR")
            .field("spi_mem_reject_addr", &self.spi_mem_reject_addr())
            .finish()
    }
}
#[doc = "SPI1 access reject addr register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_pms_reject_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_PMS_REJECT_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_PMS_REJECT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_pms_reject_addr::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_PMS_REJECT_ADDR_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_PMS_REJECT_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_PMS_REJECT_ADDR_SPEC {}
