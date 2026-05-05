#[doc = "Register `DMA_INT_ENA` reader"]
pub type R = crate::R<DMA_INT_ENA_SPEC>;
#[doc = "Register `DMA_INT_ENA` writer"]
pub type W = crate::W<DMA_INT_ENA_SPEC>;
#[doc = "Field `LP_REG_SLV_RD_BUF_DONE_INT_ENA` reader - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_RD_BUF_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_RD_BUF_DONE_INT_ENA` writer - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_RD_BUF_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_WR_BUF_DONE_INT_ENA` reader - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_WR_BUF_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_WR_BUF_DONE_INT_ENA` writer - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type LP_REG_SLV_WR_BUF_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_TRANS_DONE_INT_ENA` reader - The enable bit for SPI_TRANS_DONE_INT interrupt."]
pub type LP_REG_TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_TRANS_DONE_INT_ENA` writer - The enable bit for SPI_TRANS_DONE_INT interrupt."]
pub type LP_REG_TRANS_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SPI_WAKEUP_INT_ENA` reader - The enable bit for SPI_WAKEUP_INT interrupt"]
pub type LP_REG_SPI_WAKEUP_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_SPI_WAKEUP_INT_ENA` writer - The enable bit for SPI_WAKEUP_INT interrupt"]
pub type LP_REG_SPI_WAKEUP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_BUF_ADDR_ERR_INT_ENA` reader - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type LP_REG_SLV_BUF_ADDR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_BUF_ADDR_ERR_INT_ENA` writer - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type LP_REG_SLV_BUF_ADDR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_SLV_CMD_ERR_INT_ENA` reader - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type LP_REG_SLV_CMD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_SLV_CMD_ERR_INT_ENA` writer - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type LP_REG_SLV_CMD_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA` reader - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA` writer - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA` reader - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA` writer - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_APP2_INT_ENA` reader - The enable bit for SPI_APP2_INT interrupt."]
pub type LP_REG_APP2_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_APP2_INT_ENA` writer - The enable bit for SPI_APP2_INT interrupt."]
pub type LP_REG_APP2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_REG_APP1_INT_ENA` reader - The enable bit for SPI_APP1_INT interrupt."]
pub type LP_REG_APP1_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_REG_APP1_INT_ENA` writer - The enable bit for SPI_APP1_INT interrupt."]
pub type LP_REG_APP1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 10 - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_rd_buf_done_int_ena(&self) -> LP_REG_SLV_RD_BUF_DONE_INT_ENA_R {
        LP_REG_SLV_RD_BUF_DONE_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_wr_buf_done_int_ena(&self) -> LP_REG_SLV_WR_BUF_DONE_INT_ENA_R {
        LP_REG_SLV_WR_BUF_DONE_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_trans_done_int_ena(&self) -> LP_REG_TRANS_DONE_INT_ENA_R {
        LP_REG_TRANS_DONE_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - The enable bit for SPI_WAKEUP_INT interrupt"]
    #[inline(always)]
    pub fn lp_reg_spi_wakeup_int_ena(&self) -> LP_REG_SPI_WAKEUP_INT_ENA_R {
        LP_REG_SPI_WAKEUP_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_buf_addr_err_int_ena(&self) -> LP_REG_SLV_BUF_ADDR_ERR_INT_ENA_R {
        LP_REG_SLV_BUF_ADDR_ERR_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_cmd_err_int_ena(&self) -> LP_REG_SLV_CMD_ERR_INT_ENA_R {
        LP_REG_SLV_CMD_ERR_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_rx_afifo_wfull_err_int_ena(&self) -> LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R {
        LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_tx_afifo_rempty_err_int_ena(
        &self,
    ) -> LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R {
        LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The enable bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app2_int_ena(&self) -> LP_REG_APP2_INT_ENA_R {
        LP_REG_APP2_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The enable bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app1_int_ena(&self) -> LP_REG_APP1_INT_ENA_R {
        LP_REG_APP1_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ENA")
            .field(
                "lp_reg_slv_rd_buf_done_int_ena",
                &self.lp_reg_slv_rd_buf_done_int_ena(),
            )
            .field(
                "lp_reg_slv_wr_buf_done_int_ena",
                &self.lp_reg_slv_wr_buf_done_int_ena(),
            )
            .field(
                "lp_reg_trans_done_int_ena",
                &self.lp_reg_trans_done_int_ena(),
            )
            .field(
                "lp_reg_spi_wakeup_int_ena",
                &self.lp_reg_spi_wakeup_int_ena(),
            )
            .field(
                "lp_reg_slv_buf_addr_err_int_ena",
                &self.lp_reg_slv_buf_addr_err_int_ena(),
            )
            .field(
                "lp_reg_slv_cmd_err_int_ena",
                &self.lp_reg_slv_cmd_err_int_ena(),
            )
            .field(
                "lp_reg_mst_rx_afifo_wfull_err_int_ena",
                &self.lp_reg_mst_rx_afifo_wfull_err_int_ena(),
            )
            .field(
                "lp_reg_mst_tx_afifo_rempty_err_int_ena",
                &self.lp_reg_mst_tx_afifo_rempty_err_int_ena(),
            )
            .field("lp_reg_app2_int_ena", &self.lp_reg_app2_int_ena())
            .field("lp_reg_app1_int_ena", &self.lp_reg_app1_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 10 - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_rd_buf_done_int_ena(
        &mut self,
    ) -> LP_REG_SLV_RD_BUF_DONE_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_SLV_RD_BUF_DONE_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_wr_buf_done_int_ena(
        &mut self,
    ) -> LP_REG_SLV_WR_BUF_DONE_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_SLV_WR_BUF_DONE_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - The enable bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_trans_done_int_ena(
        &mut self,
    ) -> LP_REG_TRANS_DONE_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_TRANS_DONE_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 14 - The enable bit for SPI_WAKEUP_INT interrupt"]
    #[inline(always)]
    pub fn lp_reg_spi_wakeup_int_ena(
        &mut self,
    ) -> LP_REG_SPI_WAKEUP_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_SPI_WAKEUP_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_buf_addr_err_int_ena(
        &mut self,
    ) -> LP_REG_SLV_BUF_ADDR_ERR_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_SLV_BUF_ADDR_ERR_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_slv_cmd_err_int_ena(
        &mut self,
    ) -> LP_REG_SLV_CMD_ERR_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_SLV_CMD_ERR_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_rx_afifo_wfull_err_int_ena(
        &mut self,
    ) -> LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_mst_tx_afifo_rempty_err_int_ena(
        &mut self,
    ) -> LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - The enable bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app2_int_ena(&mut self) -> LP_REG_APP2_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_APP2_INT_ENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - The enable bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn lp_reg_app1_int_ena(&mut self) -> LP_REG_APP1_INT_ENA_W<'_, DMA_INT_ENA_SPEC> {
        LP_REG_APP1_INT_ENA_W::new(self, 20)
    }
}
#[doc = "SPI DMA interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_ENA_SPEC;
impl crate::RegisterSpec for DMA_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_int_ena::R`](R) reader structure"]
impl crate::Readable for DMA_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_int_ena::W`](W) writer structure"]
impl crate::Writable for DMA_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMA_INT_ENA to value 0"]
impl crate::Resettable for DMA_INT_ENA_SPEC {}
