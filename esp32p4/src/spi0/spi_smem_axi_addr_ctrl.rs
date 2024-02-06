#[doc = "Register `SPI_SMEM_AXI_ADDR_CTRL` reader"]
pub type R = crate::R<SPI_SMEM_AXI_ADDR_CTRL_SPEC>;
#[doc = "Field `SPI_MEM_ALL_FIFO_EMPTY` reader - The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others."]
pub type SPI_MEM_ALL_FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `SPI_RDATA_AFIFO_REMPTY` reader - 1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending."]
pub type SPI_RDATA_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `SPI_RADDR_AFIFO_REMPTY` reader - 1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending."]
pub type SPI_RADDR_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `SPI_WDATA_AFIFO_REMPTY` reader - 1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending."]
pub type SPI_WDATA_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `SPI_WBLEN_AFIFO_REMPTY` reader - 1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending."]
pub type SPI_WBLEN_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `SPI_ALL_AXI_TRANS_AFIFO_EMPTY` reader - This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE."]
pub type SPI_ALL_AXI_TRANS_AFIFO_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 26 - The empty status of all AFIFO and SYNC_FIFO in MSPI module. 1: All AXI transfers and SPI0 transfers are done. 0: Others."]
    #[inline(always)]
    pub fn spi_mem_all_fifo_empty(&self) -> SPI_MEM_ALL_FIFO_EMPTY_R {
        SPI_MEM_ALL_FIFO_EMPTY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - 1: RDATA_AFIFO is empty. 0: At least one AXI read transfer is pending."]
    #[inline(always)]
    pub fn spi_rdata_afifo_rempty(&self) -> SPI_RDATA_AFIFO_REMPTY_R {
        SPI_RDATA_AFIFO_REMPTY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - 1: AXI_RADDR_CTL_AFIFO is empty. 0: At least one AXI read transfer is pending."]
    #[inline(always)]
    pub fn spi_raddr_afifo_rempty(&self) -> SPI_RADDR_AFIFO_REMPTY_R {
        SPI_RADDR_AFIFO_REMPTY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - 1: WDATA_AFIFO is empty. 0: At least one AXI write transfer is pending."]
    #[inline(always)]
    pub fn spi_wdata_afifo_rempty(&self) -> SPI_WDATA_AFIFO_REMPTY_R {
        SPI_WDATA_AFIFO_REMPTY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1: WBLEN_AFIFO is empty. 0: At least one AXI write transfer is pending."]
    #[inline(always)]
    pub fn spi_wblen_afifo_rempty(&self) -> SPI_WBLEN_AFIFO_REMPTY_R {
        SPI_WBLEN_AFIFO_REMPTY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - This bit is set when WADDR_AFIFO, WBLEN_AFIFO, WDATA_AFIFO, AXI_RADDR_CTL_AFIFO and RDATA_AFIFO are empty and spi0_mst_st is IDLE."]
    #[inline(always)]
    pub fn spi_all_axi_trans_afifo_empty(&self) -> SPI_ALL_AXI_TRANS_AFIFO_EMPTY_R {
        SPI_ALL_AXI_TRANS_AFIFO_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_SMEM_AXI_ADDR_CTRL")
            .field(
                "spi_mem_all_fifo_empty",
                &format_args!("{}", self.spi_mem_all_fifo_empty().bit()),
            )
            .field(
                "spi_rdata_afifo_rempty",
                &format_args!("{}", self.spi_rdata_afifo_rempty().bit()),
            )
            .field(
                "spi_raddr_afifo_rempty",
                &format_args!("{}", self.spi_raddr_afifo_rempty().bit()),
            )
            .field(
                "spi_wdata_afifo_rempty",
                &format_args!("{}", self.spi_wdata_afifo_rempty().bit()),
            )
            .field(
                "spi_wblen_afifo_rempty",
                &format_args!("{}", self.spi_wblen_afifo_rempty().bit()),
            )
            .field(
                "spi_all_axi_trans_afifo_empty",
                &format_args!("{}", self.spi_all_axi_trans_afifo_empty().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_SMEM_AXI_ADDR_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "SPI0 AXI address control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spi_smem_axi_addr_ctrl::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_SMEM_AXI_ADDR_CTRL_SPEC;
impl crate::RegisterSpec for SPI_SMEM_AXI_ADDR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_smem_axi_addr_ctrl::R`](R) reader structure"]
impl crate::Readable for SPI_SMEM_AXI_ADDR_CTRL_SPEC {}
#[doc = "`reset()` method sets SPI_SMEM_AXI_ADDR_CTRL to value 0xfc00_0000"]
impl crate::Resettable for SPI_SMEM_AXI_ADDR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xfc00_0000;
}
