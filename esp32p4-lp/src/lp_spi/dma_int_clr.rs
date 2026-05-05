#[doc = "Register `DMA_INT_CLR` writer"]
pub type W = crate::W<DMA_INT_CLR_SPEC>;
#[doc = "Field `LP_REG_SLV_RD_BUF_DONE_INT_CLR` writer - The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_RD_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_WR_BUF_DONE_INT_CLR` writer - The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_WR_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_TRANS_DONE_INT_CLR` writer - The clear bit for SPI_TRANS_DONE_INT interrupt."]
pub type LP_REG_TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SPI_WAKEUP_INT_CLR` writer - The clear bit for SPI_WAKEUP_INT interrupt"]
pub type LP_REG_SPI_WAKEUP_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_BUF_ADDR_ERR_INT_CLR` writer - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type LP_REG_SLV_BUF_ADDR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_CMD_ERR_INT_CLR` writer - The clear bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type LP_REG_SLV_CMD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_CLR` writer - The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_CLR` writer - The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_APP2_INT_CLR` writer - The clear bit for SPI_APP2_INT interrupt."]
pub type LP_REG_APP2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_APP1_INT_CLR` writer - The clear bit for SPI_APP1_INT interrupt."]
pub type LP_REG_APP1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 10 - The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_rd_buf_done_int_clr(
        &mut self,
    ) -> LP_REG_SLV_RD_BUF_DONE_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_SLV_RD_BUF_DONE_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_wr_buf_done_int_clr(
        &mut self,
    ) -> LP_REG_SLV_WR_BUF_DONE_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_SLV_WR_BUF_DONE_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - The clear bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_trans_done_int_clr(
        &mut self,
    ) -> LP_REG_TRANS_DONE_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_TRANS_DONE_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 14 - The clear bit for SPI_WAKEUP_INT interrupt"]
    #[inline(always)]
    pub fn lp_reg_spi_wakeup_int_clr(
        &mut self,
    ) -> LP_REG_SPI_WAKEUP_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_SPI_WAKEUP_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_buf_addr_err_int_clr(
        &mut self,
    ) -> LP_REG_SLV_BUF_ADDR_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_SLV_BUF_ADDR_ERR_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - The clear bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_cmd_err_int_clr(
        &mut self,
    ) -> LP_REG_SLV_CMD_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_SLV_CMD_ERR_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_rx_afifo_wfull_err_int_clr(
        &mut self,
    ) -> LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_tx_afifo_rempty_err_int_clr(
        &mut self,
    ) -> LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - The clear bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app2_int_clr(&mut self) -> LP_REG_APP2_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_APP2_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - The clear bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app1_int_clr(&mut self) -> LP_REG_APP1_INT_CLR_W<'_, DMA_INT_CLR_SPEC> {
        LP_REG_APP1_INT_CLR_W::new(self, 20)
    }
}
#[doc = "SPI DMA interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
