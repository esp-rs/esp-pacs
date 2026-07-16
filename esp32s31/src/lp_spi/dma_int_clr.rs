#[doc = "Register `DMA_INT_CLR` writer"]
pub type W = crate::W<DMA_INT_CLR_SPEC>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_CLR` writer - "]
pub type SLV_RD_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE_INT_CLR` writer - "]
pub type SLV_WR_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE_INT_CLR` writer - "]
pub type TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_WAKEUP_INT_CLR` writer - "]
pub type SPI_WAKEUP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_CLR` writer - "]
pub type SLV_BUF_ADDR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD_ERR_INT_CLR` writer - "]
pub type SLV_CMD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_CLR` writer - "]
pub type MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_CLR` writer - "]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP2_INT_CLR` writer - "]
pub type APP2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP1_INT_CLR` writer - "]
pub type APP1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_clr(&mut self) -> SLV_RD_BUF_DONE_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        SLV_RD_BUF_DONE_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_clr(&mut self) -> SLV_WR_BUF_DONE_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        SLV_WR_BUF_DONE_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn trans_done_int_clr(&mut self) -> TRANS_DONE_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        TRANS_DONE_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn spi_wakeup_int_clr(&mut self) -> SPI_WAKEUP_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        SPI_WAKEUP_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_clr(&mut self) -> SLV_BUF_ADDR_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        SLV_BUF_ADDR_ERR_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slv_cmd_err_int_clr(&mut self) -> SLV_CMD_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        SLV_CMD_ERR_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_clr(
        &mut self,
    ) -> MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        MST_RX_AFIFO_WFULL_ERR_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_clr(
        &mut self,
    ) -> MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app2_int_clr(&mut self) -> APP2_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        APP2_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app1_int_clr(&mut self) -> APP1_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        APP1_INT_CLR_W::new(self, 20)
    }
}
#[doc = "\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int_clr::W`](W) writer structure"]
impl crate::Writable for DMA_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_CLR to value 0"]
impl crate::Resettable for DMA_INT_CLR_SPEC {}
