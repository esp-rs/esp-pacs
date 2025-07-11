#[doc = "Register `SPI_DMA_INT_ENA` reader"]
pub type R = crate::R<SPI_DMA_INT_ENA_SPEC>;
#[doc = "Register `SPI_DMA_INT_ENA` writer"]
pub type W = crate::W<SPI_DMA_INT_ENA_SPEC>;
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_ENA` reader - Write 1 to enable SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_ENA` writer - Write 1 to enable SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA` reader - Write 1 to enable SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA` writer - Write 1 to enable SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_EX_QPI_INT_ENA` reader - Write 1 to enable SPI_SLV_EX_QPI_INT interrupt."]
pub type SPI_SLV_EX_QPI_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_EX_QPI_INT_ENA` writer - Write 1 to enable SPI_SLV_EX_QPI_INT interrupt."]
pub type SPI_SLV_EX_QPI_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_EN_QPI_INT_ENA` reader - Write 1 to enable SPI_SLV_EN_QPI_INT interrupt."]
pub type SPI_SLV_EN_QPI_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_EN_QPI_INT_ENA` writer - Write 1 to enable SPI_SLV_EN_QPI_INT interrupt."]
pub type SPI_SLV_EN_QPI_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD7_INT_ENA` reader - Write 1 to enable SPI_SLV_CMD7_INT interrupt."]
pub type SPI_SLV_CMD7_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD7_INT_ENA` writer - Write 1 to enable SPI_SLV_CMD7_INT interrupt."]
pub type SPI_SLV_CMD7_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD8_INT_ENA` reader - Write 1 to enable SPI_SLV_CMD8_INT interrupt."]
pub type SPI_SLV_CMD8_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD8_INT_ENA` writer - Write 1 to enable SPI_SLV_CMD8_INT interrupt."]
pub type SPI_SLV_CMD8_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD9_INT_ENA` reader - Write 1 to enable SPI_SLV_CMD9_INT interrupt."]
pub type SPI_SLV_CMD9_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD9_INT_ENA` writer - Write 1 to enable SPI_SLV_CMD9_INT interrupt."]
pub type SPI_SLV_CMD9_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMDA_INT_ENA` reader - Write 1 to enable SPI_SLV_CMDA_INT interrupt."]
pub type SPI_SLV_CMDA_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMDA_INT_ENA` writer - Write 1 to enable SPI_SLV_CMDA_INT interrupt."]
pub type SPI_SLV_CMDA_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_ENA` reader - Write 1 to enable SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SPI_SLV_RD_DMA_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_ENA` writer - Write 1 to enable SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SPI_SLV_RD_DMA_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_ENA` reader - Write 1 to enable SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SPI_SLV_WR_DMA_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_ENA` writer - Write 1 to enable SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SPI_SLV_WR_DMA_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_ENA` reader - Write 1 to enable SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SPI_SLV_RD_BUF_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_ENA` writer - Write 1 to enable SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SPI_SLV_RD_BUF_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_ENA` reader - Write 1 to enable SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SPI_SLV_WR_BUF_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_ENA` writer - Write 1 to enable SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SPI_SLV_WR_BUF_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_TRANS_DONE_INT_ENA` reader - Write 1 to enable SPI_TRANS_DONE_INT interrupt."]
pub type SPI_TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_TRANS_DONE_INT_ENA` writer - Write 1 to enable SPI_TRANS_DONE_INT interrupt."]
pub type SPI_TRANS_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_ENA` reader - Write 1 to enable SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_ENA` writer - Write 1 to enable SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SEG_MAGIC_ERR_INT_ENA` reader - Write 1 to enable SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SPI_SEG_MAGIC_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SEG_MAGIC_ERR_INT_ENA` writer - Write 1 to enable SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SPI_SEG_MAGIC_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_ENA` reader - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_ENA` writer - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_ENA` reader - Write 1 to enable SPI_SLV_CMD_ERR_INT interrupt."]
pub type SPI_SLV_CMD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_ENA` writer - Write 1 to enable SPI_SLV_CMD_ERR_INT interrupt."]
pub type SPI_SLV_CMD_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA` reader - Write 1 to enable SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA` writer - Write 1 to enable SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA` reader - Write 1 to enable SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA` writer - Write 1 to enable SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_APP2_INT_ENA` reader - Write 1 to enable SPI_APP2_INT interrupt."]
pub type SPI_APP2_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_APP2_INT_ENA` writer - Write 1 to enable SPI_APP2_INT interrupt."]
pub type SPI_APP2_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SPI_APP1_INT_ENA` reader - Write 1 to enable SPI_APP1_INT interrupt."]
pub type SPI_APP1_INT_ENA_R = crate::BitReader;
#[doc = "Field `SPI_APP1_INT_ENA` writer - Write 1 to enable SPI_APP1_INT interrupt."]
pub type SPI_APP1_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Write 1 to enable SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_ena(&self) -> SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R {
        SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to enable SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_ena(&self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to enable SPI_SLV_EX_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_ena(&self) -> SPI_SLV_EX_QPI_INT_ENA_R {
        SPI_SLV_EX_QPI_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to enable SPI_SLV_EN_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_ena(&self) -> SPI_SLV_EN_QPI_INT_ENA_R {
        SPI_SLV_EN_QPI_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write 1 to enable SPI_SLV_CMD7_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_ena(&self) -> SPI_SLV_CMD7_INT_ENA_R {
        SPI_SLV_CMD7_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write 1 to enable SPI_SLV_CMD8_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_ena(&self) -> SPI_SLV_CMD8_INT_ENA_R {
        SPI_SLV_CMD8_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to enable SPI_SLV_CMD9_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_ena(&self) -> SPI_SLV_CMD9_INT_ENA_R {
        SPI_SLV_CMD9_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to enable SPI_SLV_CMDA_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmda_int_ena(&self) -> SPI_SLV_CMDA_INT_ENA_R {
        SPI_SLV_CMDA_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to enable SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_ena(&self) -> SPI_SLV_RD_DMA_DONE_INT_ENA_R {
        SPI_SLV_RD_DMA_DONE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to enable SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_ena(&self) -> SPI_SLV_WR_DMA_DONE_INT_ENA_R {
        SPI_SLV_WR_DMA_DONE_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Write 1 to enable SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_ena(&self) -> SPI_SLV_RD_BUF_DONE_INT_ENA_R {
        SPI_SLV_RD_BUF_DONE_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Write 1 to enable SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_ena(&self) -> SPI_SLV_WR_BUF_DONE_INT_ENA_R {
        SPI_SLV_WR_BUF_DONE_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Write 1 to enable SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_trans_done_int_ena(&self) -> SPI_TRANS_DONE_INT_ENA_R {
        SPI_TRANS_DONE_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Write 1 to enable SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_ena(&self) -> SPI_DMA_SEG_TRANS_DONE_INT_ENA_R {
        SPI_DMA_SEG_TRANS_DONE_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Write 1 to enable SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_ena(&self) -> SPI_SEG_MAGIC_ERR_INT_ENA_R {
        SPI_SEG_MAGIC_ERR_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_ena(&self) -> SPI_SLV_BUF_ADDR_ERR_INT_ENA_R {
        SPI_SLV_BUF_ADDR_ERR_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Write 1 to enable SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_ena(&self) -> SPI_SLV_CMD_ERR_INT_ENA_R {
        SPI_SLV_CMD_ERR_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Write 1 to enable SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_ena(&self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Write 1 to enable SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_ena(&self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Write 1 to enable SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn spi_app2_int_ena(&self) -> SPI_APP2_INT_ENA_R {
        SPI_APP2_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Write 1 to enable SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn spi_app1_int_ena(&self) -> SPI_APP1_INT_ENA_R {
        SPI_APP1_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DMA_INT_ENA")
            .field(
                "spi_dma_infifo_full_err_int_ena",
                &self.spi_dma_infifo_full_err_int_ena(),
            )
            .field(
                "spi_dma_outfifo_empty_err_int_ena",
                &self.spi_dma_outfifo_empty_err_int_ena(),
            )
            .field("spi_slv_ex_qpi_int_ena", &self.spi_slv_ex_qpi_int_ena())
            .field("spi_slv_en_qpi_int_ena", &self.spi_slv_en_qpi_int_ena())
            .field("spi_slv_cmd7_int_ena", &self.spi_slv_cmd7_int_ena())
            .field("spi_slv_cmd8_int_ena", &self.spi_slv_cmd8_int_ena())
            .field("spi_slv_cmd9_int_ena", &self.spi_slv_cmd9_int_ena())
            .field("spi_slv_cmda_int_ena", &self.spi_slv_cmda_int_ena())
            .field(
                "spi_slv_rd_dma_done_int_ena",
                &self.spi_slv_rd_dma_done_int_ena(),
            )
            .field(
                "spi_slv_wr_dma_done_int_ena",
                &self.spi_slv_wr_dma_done_int_ena(),
            )
            .field(
                "spi_slv_rd_buf_done_int_ena",
                &self.spi_slv_rd_buf_done_int_ena(),
            )
            .field(
                "spi_slv_wr_buf_done_int_ena",
                &self.spi_slv_wr_buf_done_int_ena(),
            )
            .field("spi_trans_done_int_ena", &self.spi_trans_done_int_ena())
            .field(
                "spi_dma_seg_trans_done_int_ena",
                &self.spi_dma_seg_trans_done_int_ena(),
            )
            .field(
                "spi_seg_magic_err_int_ena",
                &self.spi_seg_magic_err_int_ena(),
            )
            .field(
                "spi_slv_buf_addr_err_int_ena",
                &self.spi_slv_buf_addr_err_int_ena(),
            )
            .field("spi_slv_cmd_err_int_ena", &self.spi_slv_cmd_err_int_ena())
            .field(
                "spi_mst_rx_afifo_wfull_err_int_ena",
                &self.spi_mst_rx_afifo_wfull_err_int_ena(),
            )
            .field(
                "spi_mst_tx_afifo_rempty_err_int_ena",
                &self.spi_mst_tx_afifo_rempty_err_int_ena(),
            )
            .field("spi_app2_int_ena", &self.spi_app2_int_ena())
            .field("spi_app1_int_ena", &self.spi_app1_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to enable SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_ena(
        &mut self,
    ) -> SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to enable SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_ena(
        &mut self,
    ) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to enable SPI_SLV_EX_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_ena(&mut self) -> SPI_SLV_EX_QPI_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_EX_QPI_INT_ENA_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to enable SPI_SLV_EN_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_ena(&mut self) -> SPI_SLV_EN_QPI_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_EN_QPI_INT_ENA_W::new(self, 3)
    }
    #[doc = "Bit 4 - Write 1 to enable SPI_SLV_CMD7_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_ena(&mut self) -> SPI_SLV_CMD7_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_CMD7_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bit 5 - Write 1 to enable SPI_SLV_CMD8_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_ena(&mut self) -> SPI_SLV_CMD8_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_CMD8_INT_ENA_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to enable SPI_SLV_CMD9_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_ena(&mut self) -> SPI_SLV_CMD9_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_CMD9_INT_ENA_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to enable SPI_SLV_CMDA_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmda_int_ena(&mut self) -> SPI_SLV_CMDA_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_CMDA_INT_ENA_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to enable SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_ena(
        &mut self,
    ) -> SPI_SLV_RD_DMA_DONE_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_RD_DMA_DONE_INT_ENA_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to enable SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_ena(
        &mut self,
    ) -> SPI_SLV_WR_DMA_DONE_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_WR_DMA_DONE_INT_ENA_W::new(self, 9)
    }
    #[doc = "Bit 10 - Write 1 to enable SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_ena(
        &mut self,
    ) -> SPI_SLV_RD_BUF_DONE_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_RD_BUF_DONE_INT_ENA_W::new(self, 10)
    }
    #[doc = "Bit 11 - Write 1 to enable SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_ena(
        &mut self,
    ) -> SPI_SLV_WR_BUF_DONE_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_WR_BUF_DONE_INT_ENA_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 to enable SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_trans_done_int_ena(&mut self) -> SPI_TRANS_DONE_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_TRANS_DONE_INT_ENA_W::new(self, 12)
    }
    #[doc = "Bit 13 - Write 1 to enable SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_ena(
        &mut self,
    ) -> SPI_DMA_SEG_TRANS_DONE_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_DMA_SEG_TRANS_DONE_INT_ENA_W::new(self, 13)
    }
    #[doc = "Bit 14 - Write 1 to enable SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_ena(
        &mut self,
    ) -> SPI_SEG_MAGIC_ERR_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SEG_MAGIC_ERR_INT_ENA_W::new(self, 14)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_ena(
        &mut self,
    ) -> SPI_SLV_BUF_ADDR_ERR_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_BUF_ADDR_ERR_INT_ENA_W::new(self, 15)
    }
    #[doc = "Bit 16 - Write 1 to enable SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_ena(&mut self) -> SPI_SLV_CMD_ERR_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_SLV_CMD_ERR_INT_ENA_W::new(self, 16)
    }
    #[doc = "Bit 17 - Write 1 to enable SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_ena(
        &mut self,
    ) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W::new(self, 17)
    }
    #[doc = "Bit 18 - Write 1 to enable SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_ena(
        &mut self,
    ) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W::new(self, 18)
    }
    #[doc = "Bit 19 - Write 1 to enable SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn spi_app2_int_ena(&mut self) -> SPI_APP2_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_APP2_INT_ENA_W::new(self, 19)
    }
    #[doc = "Bit 20 - Write 1 to enable SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn spi_app1_int_ena(&mut self) -> SPI_APP1_INT_ENA_W<SPI_DMA_INT_ENA_SPEC> {
        SPI_APP1_INT_ENA_W::new(self, 20)
    }
}
#[doc = "SPI interrupt enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dma_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spi_dma_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DMA_INT_ENA_SPEC;
impl crate::RegisterSpec for SPI_DMA_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dma_int_ena::R`](R) reader structure"]
impl crate::Readable for SPI_DMA_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spi_dma_int_ena::W`](W) writer structure"]
impl crate::Writable for SPI_DMA_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SPI_DMA_INT_ENA to value 0"]
impl crate::Resettable for SPI_DMA_INT_ENA_SPEC {}
