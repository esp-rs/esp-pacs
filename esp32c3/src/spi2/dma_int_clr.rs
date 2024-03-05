#[doc = "Register `DMA_INT_CLR` writer"]
pub type W = crate::W<DMA_INT_CLR_SPEC>;
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_CLR` writer - The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DMA_INFIFO_FULL_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_CLR` writer - The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EX_QPI_INT_CLR` writer - The clear bit for SPI slave Ex_QPI interrupt."]
pub type SLV_EX_QPI_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_EN_QPI_INT_CLR` writer - The clear bit for SPI slave En_QPI interrupt."]
pub type SLV_EN_QPI_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD7_INT_CLR` writer - The clear bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD8_INT_CLR` writer - The clear bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD9_INT_CLR` writer - The clear bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMDA_INT_CLR` writer - The clear bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_DMA_DONE_INT_CLR` writer - The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SLV_RD_DMA_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_DMA_DONE_INT_CLR` writer - The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SLV_WR_DMA_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_CLR` writer - The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SLV_RD_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_WR_BUF_DONE_INT_CLR` writer - The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SLV_WR_BUF_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_DONE_INT_CLR` writer - The clear bit for SPI_TRANS_DONE_INT interrupt."]
pub type TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_CLR` writer - The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DMA_SEG_TRANS_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SEG_MAGIC_ERR_INT_CLR` writer - The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SEG_MAGIC_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_CLR` writer - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SLV_BUF_ADDR_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLV_CMD_ERR_INT_CLR` writer - The clear bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SLV_CMD_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_CLR` writer - The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_CLR` writer - The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP2_INT_CLR` writer - The clear bit for SPI_APP2_INT interrupt."]
pub type APP2_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APP1_INT_CLR` writer - The clear bit for SPI_APP1_INT interrupt."]
pub type APP1_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The clear bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_err_int_clr(
        &mut self,
    ) -> DMA_INFIFO_FULL_ERR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        DMA_INFIFO_FULL_ERR_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The clear bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_outfifo_empty_err_int_clr(
        &mut self,
    ) -> DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        DMA_OUTFIFO_EMPTY_ERR_INT_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - The clear bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_ex_qpi_int_clr(&mut self) -> SLV_EX_QPI_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_EX_QPI_INT_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - The clear bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_en_qpi_int_clr(&mut self) -> SLV_EN_QPI_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_EN_QPI_INT_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - The clear bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd7_int_clr(&mut self) -> SLV_CMD7_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_CMD7_INT_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The clear bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd8_int_clr(&mut self) -> SLV_CMD8_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_CMD8_INT_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The clear bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd9_int_clr(&mut self) -> SLV_CMD9_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_CMD9_INT_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The clear bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmda_int_clr(&mut self) -> SLV_CMDA_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_CMDA_INT_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The clear bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_dma_done_int_clr(&mut self) -> SLV_RD_DMA_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_RD_DMA_DONE_INT_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - The clear bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_dma_done_int_clr(&mut self) -> SLV_WR_DMA_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_WR_DMA_DONE_INT_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - The clear bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_buf_done_int_clr(&mut self) -> SLV_RD_BUF_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_RD_BUF_DONE_INT_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - The clear bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_buf_done_int_clr(&mut self) -> SLV_WR_BUF_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_WR_BUF_DONE_INT_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - The clear bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_done_int_clr(&mut self) -> TRANS_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        TRANS_DONE_INT_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - The clear bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_seg_trans_done_int_clr(&mut self) -> DMA_SEG_TRANS_DONE_INT_CLR_W<DMA_INT_CLR_SPEC> {
        DMA_SEG_TRANS_DONE_INT_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - The clear bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn seg_magic_err_int_clr(&mut self) -> SEG_MAGIC_ERR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SEG_MAGIC_ERR_INT_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - The clear bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_buf_addr_err_int_clr(&mut self) -> SLV_BUF_ADDR_ERR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_BUF_ADDR_ERR_INT_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - The clear bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd_err_int_clr(&mut self) -> SLV_CMD_ERR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        SLV_CMD_ERR_INT_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - The clear bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_rx_afifo_wfull_err_int_clr(
        &mut self,
    ) -> MST_RX_AFIFO_WFULL_ERR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        MST_RX_AFIFO_WFULL_ERR_INT_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - The clear bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_tx_afifo_rempty_err_int_clr(
        &mut self,
    ) -> MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W<DMA_INT_CLR_SPEC> {
        MST_TX_AFIFO_REMPTY_ERR_INT_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - The clear bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app2_int_clr(&mut self) -> APP2_INT_CLR_W<DMA_INT_CLR_SPEC> {
        APP2_INT_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - The clear bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app1_int_clr(&mut self) -> APP1_INT_CLR_W<DMA_INT_CLR_SPEC> {
        APP1_INT_CLR_W::new(self, 20)
    }
}
#[doc = "SPI DMA interrupt clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_INT_CLR_SPEC;
impl crate::RegisterSpec for DMA_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_int_clr::W`](W) writer structure"]
impl crate::Writable for DMA_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_INT_CLR to value 0"]
impl crate::Resettable for DMA_INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
