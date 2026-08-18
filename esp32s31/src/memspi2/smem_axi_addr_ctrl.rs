#[doc = "Register `SMEM_AXI_ADDR_CTRL` reader"]
pub type R = crate::R<SMEM_AXI_ADDR_CTRL_SPEC>;
#[doc = "Field `ALL_FIFO_EMPTY` reader - "]
pub type ALL_FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `RDATA_AFIFO_REMPTY` reader - "]
pub type RDATA_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `RADDR_AFIFO_REMPTY` reader - "]
pub type RADDR_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `WDATA_AFIFO_REMPTY` reader - "]
pub type WDATA_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `WBLEN_AFIFO_REMPTY` reader - "]
pub type WBLEN_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `ALL_AXI_TRANS_AFIFO_EMPTY` reader - "]
pub type ALL_AXI_TRANS_AFIFO_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 26"]
    #[inline(always)]
    pub fn all_fifo_empty(&self) -> ALL_FIFO_EMPTY_R {
        ALL_FIFO_EMPTY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27"]
    #[inline(always)]
    pub fn rdata_afifo_rempty(&self) -> RDATA_AFIFO_REMPTY_R {
        RDATA_AFIFO_REMPTY_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn raddr_afifo_rempty(&self) -> RADDR_AFIFO_REMPTY_R {
        RADDR_AFIFO_REMPTY_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn wdata_afifo_rempty(&self) -> WDATA_AFIFO_REMPTY_R {
        WDATA_AFIFO_REMPTY_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn wblen_afifo_rempty(&self) -> WBLEN_AFIFO_REMPTY_R {
        WBLEN_AFIFO_REMPTY_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn all_axi_trans_afifo_empty(&self) -> ALL_AXI_TRANS_AFIFO_EMPTY_R {
        ALL_AXI_TRANS_AFIFO_EMPTY_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_AXI_ADDR_CTRL")
            .field("all_fifo_empty", &self.all_fifo_empty())
            .field("rdata_afifo_rempty", &self.rdata_afifo_rempty())
            .field("raddr_afifo_rempty", &self.raddr_afifo_rempty())
            .field("wdata_afifo_rempty", &self.wdata_afifo_rempty())
            .field("wblen_afifo_rempty", &self.wblen_afifo_rempty())
            .field(
                "all_axi_trans_afifo_empty",
                &self.all_axi_trans_afifo_empty(),
            )
            .finish()
    }
}
#[doc = "SPI0 AXI address control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_axi_addr_ctrl::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_AXI_ADDR_CTRL_SPEC;
impl crate::RegisterSpec for SMEM_AXI_ADDR_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_axi_addr_ctrl::R`](R) reader structure"]
impl crate::Readable for SMEM_AXI_ADDR_CTRL_SPEC {}
#[doc = "`reset()` method sets SMEM_AXI_ADDR_CTRL to value 0xfc00_0000"]
impl crate::Resettable for SMEM_AXI_ADDR_CTRL_SPEC {
    const RESET_VALUE: u32 = 0xfc00_0000;
}
