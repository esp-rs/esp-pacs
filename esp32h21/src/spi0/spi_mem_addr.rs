#[doc = "Register `SPI_MEM_ADDR` reader"]
pub type R = crate::R<SPI_MEM_ADDR_SPEC>;
#[doc = "Field `SPI_MEM_USR_ADDR_VALUE` reader - In SPI0 USR_CMD mode when SPI_MEM_USR is set, it is the memory address."]
pub type SPI_MEM_USR_ADDR_VALUE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - In SPI0 USR_CMD mode when SPI_MEM_USR is set, it is the memory address."]
    #[inline(always)]
    pub fn spi_mem_usr_addr_value(&self) -> SPI_MEM_USR_ADDR_VALUE_R {
        SPI_MEM_USR_ADDR_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_ADDR")
            .field("spi_mem_usr_addr_value", &self.spi_mem_usr_addr_value())
            .finish()
    }
}
#[doc = "SPI0 USR_CMD address register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_mem_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_addr::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_ADDR_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_ADDR_SPEC {}
