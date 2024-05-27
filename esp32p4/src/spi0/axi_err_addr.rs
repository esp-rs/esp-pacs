#[doc = "Register `AXI_ERR_ADDR` reader"]
pub type R = crate::R<AXI_ERR_ADDR_SPEC>;
#[doc = "Field `AXI_ERR_ADDR` reader - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
pub type AXI_ERR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:26 - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
    #[inline(always)]
    pub fn axi_err_addr(&self) -> AXI_ERR_ADDR_R {
        AXI_ERR_ADDR_R::new(self.bits & 0x07ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_ERR_ADDR")
            .field("axi_err_addr", &self.axi_err_addr())
            .finish()
    }
}
#[doc = "SPI0 AXI request error address.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`axi_err_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_ERR_ADDR_SPEC;
impl crate::RegisterSpec for AXI_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_err_addr::R`](R) reader structure"]
impl crate::Readable for AXI_ERR_ADDR_SPEC {}
#[doc = "`reset()` method sets AXI_ERR_ADDR to value 0"]
impl crate::Resettable for AXI_ERR_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
