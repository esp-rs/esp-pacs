#[doc = "Register `SPI_SMEM_PMS%s_ADDR` reader"]
pub type R = crate::R<SPI_SMEM_PMS_ADDR_SPEC>;
#[doc = "Field `S` reader - SPI1 external RAM PMS section %s start address value"]
pub type S_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - SPI1 external RAM PMS section %s start address value"]
    #[inline(always)]
    pub fn s(&self) -> S_R {
        S_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_PMS_ADDR")
            .field("s", &self.s())
            .finish()
    }
}
#[doc = "SPI1 external RAM PMS section %s start address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_smem_pms_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_PMS_ADDR_SPEC;
impl crate::RegisterSpec for SPI_SMEM_PMS_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_pms_addr::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_PMS_ADDR_SPEC {}
#[doc = "`reset()` method sets SPI_SMEM_PMS%s_ADDR to value 0"]
impl crate::Resettable for SPI_SMEM_PMS_ADDR_SPEC {}
