#[doc = "Register `SPI_MEM_AXI_ERR_ADDR` reader"]
pub type R = crate::R<SPI_MEM_AXI_ERR_ADDR_SPEC>;
#[doc = "Field `SPI_MEM_AXI_ERR_ADDR` reader - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
pub type SPI_MEM_AXI_ERR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26 - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
    #[inline(always)]
    pub fn spi_mem_axi_err_addr(&self) -> SPI_MEM_AXI_ERR_ADDR_R {
        SPI_MEM_AXI_ERR_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_AXI_ERR_ADDR")
            .field(
                "spi_mem_axi_err_addr",
                &format_args!("{}", self.spi_mem_axi_err_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_AXI_ERR_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SPI0 AXI request error address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_mem_axi_err_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_MEM_AXI_ERR_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_AXI_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_mem_axi_err_addr::R`](R) reader structure"]
impl crate::Readable for SPI_MEM_AXI_ERR_ADDR_SPEC {}
#[doc = "`reset()` method sets SPI_MEM_AXI_ERR_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_AXI_ERR_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
