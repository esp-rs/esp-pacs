#[doc = "Register `SPI_DMA_INT_CLR` writer"]
pub type W = crate::W<SPI_DMA_INT_CLR_SPEC>;
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_CLR` writer - Write 1 to clear SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR` writer - Write 1 to clear SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_EX_QPI_INT_CLR` writer - Write 1 to clear SPI_SLV_EX_QPI_INT interrupt."]
pub type SPI_SLV_EX_QPI_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_EN_QPI_INT_CLR` writer - Write 1 to clear SPI_SLV_EN_QPI_INT interrupt."]
pub type SPI_SLV_EN_QPI_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD7_INT_CLR` writer - Write 1 to clear SPI_SLV_CMD7_INT interrupt."]
pub type SPI_SLV_CMD7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD8_INT_CLR` writer - Write 1 to clear SPI_SLV_CMD8_INT interrupt."]
pub type SPI_SLV_CMD8_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD9_INT_CLR` writer - Write 1 to clear SPI_SLV_CMD9_INT interrupt."]
pub type SPI_SLV_CMD9_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMDA_INT_CLR` writer - Write 1 to clear SPI_SLV_CMDA_INT interrupt."]
pub type SPI_SLV_CMDA_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_CLR` writer - Write 1 to clear SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SPI_SLV_RD_DMA_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_CLR` writer - Write 1 to clear SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SPI_SLV_WR_DMA_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_CLR` writer - Write 1 to clear SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SPI_SLV_RD_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_CLR` writer - Write 1 to clear SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SPI_SLV_WR_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_TRANS_DONE_INT_CLR` writer - Write 1 to clear SPI_TRANS_DONE_INT interrupt."]
pub type SPI_TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_CLR` writer - Write 1 to clear SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SEG_MAGIC_ERR_INT_CLR` writer - Write 1 to clear SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SPI_SEG_MAGIC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_CLR` writer - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_CLR` writer - Write 1 to clear SPI_SLV_CMD_ERR_INT interrupt."]
pub type SPI_SLV_CMD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR` writer - Write 1 to clear SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR` writer - Write 1 to clear SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_APP2_INT_CLR` writer - Write 1 to clear SPI_APP2_INT interrupt."]
pub type SPI_APP2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_APP1_INT_CLR` writer - Write 1 to clear SPI_APP1_INT interrupt."]
pub type SPI_APP1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_DMA_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_clr(
        &mut self,
    ) -> SPI_DMA_INFIFO_FULL_ERR_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_DMA_INFIFO_FULL_ERR_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to clear SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_clr(
        &mut self,
    ) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to clear SPI_SLV_EX_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_clr(&mut self) -> SPI_SLV_EX_QPI_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_EX_QPI_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to clear SPI_SLV_EN_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_clr(&mut self) -> SPI_SLV_EN_QPI_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_EN_QPI_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to clear SPI_SLV_CMD7_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_clr(&mut self) -> SPI_SLV_CMD7_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_CMD7_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to clear SPI_SLV_CMD8_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_clr(&mut self) -> SPI_SLV_CMD8_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_CMD8_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to clear SPI_SLV_CMD9_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_clr(&mut self) -> SPI_SLV_CMD9_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_CMD9_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to clear SPI_SLV_CMDA_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmda_int_clr(&mut self) -> SPI_SLV_CMDA_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_CMDA_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to clear SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_clr(
        &mut self,
    ) -> SPI_SLV_RD_DMA_DONE_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_RD_DMA_DONE_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to clear SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_clr(
        &mut self,
    ) -> SPI_SLV_WR_DMA_DONE_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_WR_DMA_DONE_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to clear SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_clr(
        &mut self,
    ) -> SPI_SLV_RD_BUF_DONE_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_RD_BUF_DONE_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to clear SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_clr(
        &mut self,
    ) -> SPI_SLV_WR_BUF_DONE_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_WR_BUF_DONE_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 to clear SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_trans_done_int_clr(&mut self) -> SPI_TRANS_DONE_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_TRANS_DONE_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 to clear SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_clr(
        &mut self,
    ) -> SPI_DMA_SEG_TRANS_DONE_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_DMA_SEG_TRANS_DONE_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to clear SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_clr(
        &mut self,
    ) -> SPI_SEG_MAGIC_ERR_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SEG_MAGIC_ERR_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_clr(
        &mut self,
    ) -> SPI_SLV_BUF_ADDR_ERR_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_BUF_ADDR_ERR_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Write 1 to clear SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_clr(&mut self) -> SPI_SLV_CMD_ERR_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_SLV_CMD_ERR_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write 1 to clear SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_clr(
        &mut self,
    ) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Write 1 to clear SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_clr(
        &mut self,
    ) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Write 1 to clear SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn spi_app2_int_clr(&mut self) -> SPI_APP2_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_APP2_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Write 1 to clear SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn spi_app1_int_clr(&mut self) -> SPI_APP1_INT_CLR_W<SPI_DMA_INT_CLR_SPEC> {
        SPI_APP1_INT_CLR_W::new(self, 20)
    }
}
#[doc = "SPI interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dma_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for SPI_DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`spi_dma_int_clr::W`](W) writer structure"]
impl crate::Writable for SPI_DMA_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DMA_INT_CLR to value 0"]
impl crate::Resettable for SPI_DMA_INT_CLR_SPEC {}
