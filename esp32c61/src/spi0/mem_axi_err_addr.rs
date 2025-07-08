#[doc = "Register `MEM_AXI_ERR_ADDR` reader"]
pub type R = crate::R<MEM_AXI_ERR_ADDR_SPEC>;
#[doc = "Field `MEM_AXI_ERR_ADDR` reader - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
pub type MEM_AXI_ERR_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:28 - This bits show the first AXI write/read invalid error or AXI write flash error address. It is cleared by when SPI_MEM_AXI_WADDR_ERR_INT_CLR, SPI_MEM_AXI_WR_FLASH_ERR_IN_CLR or SPI_MEM_AXI_RADDR_ERR_IN_CLR bit is set."]
    #[inline(always)]
    pub fn mem_axi_err_addr(&self) -> MEM_AXI_ERR_ADDR_R {
        MEM_AXI_ERR_ADDR_R::new(self.bits & 0x1fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_AXI_ERR_ADDR")
            .field("mem_axi_err_addr", &self.mem_axi_err_addr())
            .finish()
    }
}
#[doc = "SPI0 AXI request error address.\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_axi_err_addr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_AXI_ERR_ADDR_SPEC;
impl crate::RegisterSpec for MEM_AXI_ERR_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_axi_err_addr::R`](R) reader structure"]
impl crate::Readable for MEM_AXI_ERR_ADDR_SPEC {}
#[doc = "`reset()` method sets MEM_AXI_ERR_ADDR to value 0"]
impl crate::Resettable for MEM_AXI_ERR_ADDR_SPEC {}
