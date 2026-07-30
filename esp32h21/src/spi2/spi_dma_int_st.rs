#[doc = "Register `SPI_DMA_INT_ST` reader"]
pub type R = crate::R<SPI_DMA_INT_ST_SPEC>;
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_ST` reader - The interrupt status of SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST` reader - The interrupt status of SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_EX_QPI_INT_ST` reader - The interrupt status of SPI_SLV_EX_QPI_INT interrupt."]
pub type SPI_SLV_EX_QPI_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_EN_QPI_INT_ST` reader - The interrupt status of SPI_SLV_EN_QPI_INT interrupt."]
pub type SPI_SLV_EN_QPI_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD7_INT_ST` reader - The interrupt status of SPI_SLV_CMD7_INT interrupt."]
pub type SPI_SLV_CMD7_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD8_INT_ST` reader - The interrupt status of SPI_SLV_CMD8_INT interrupt."]
pub type SPI_SLV_CMD8_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD9_INT_ST` reader - The interrupt status of SPI_SLV_CMD9_INT interrupt."]
pub type SPI_SLV_CMD9_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMDA_INT_ST` reader - The interrupt status of SPI_SLV_CMDA_INT interrupt."]
pub type SPI_SLV_CMDA_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_ST` reader - The interrupt status of SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SPI_SLV_RD_DMA_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_ST` reader - The interrupt status of SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SPI_SLV_WR_DMA_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_ST` reader - The interrupt status of SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SPI_SLV_RD_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_ST` reader - The interrupt status of SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SPI_SLV_WR_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_TRANS_DONE_INT_ST` reader - The interrupt status of SPI_TRANS_DONE_INT interrupt."]
pub type SPI_TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_ST` reader - The interrupt status of SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SEG_MAGIC_ERR_INT_ST` reader - The interrupt status of SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SPI_SEG_MAGIC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_ST` reader - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_ST` reader - The interrupt status of SPI_SLV_CMD_ERR_INT interrupt."]
pub type SPI_SLV_CMD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST` reader - The interrupt status of SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST` reader - The interrupt status of SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_APP2_INT_ST` reader - The interrupt status of SPI_APP2_INT interrupt."]
pub type SPI_APP2_INT_ST_R = crate::BitReader;
#[doc = "Field `SPI_APP1_INT_ST` reader - The interrupt status of SPI_APP1_INT interrupt."]
pub type SPI_APP1_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The interrupt status of SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_st(&self) -> SPI_DMA_INFIFO_FULL_ERR_INT_ST_R {
        SPI_DMA_INFIFO_FULL_ERR_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt status of SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_st(&self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_R {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt status of SPI_SLV_EX_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_st(&self) -> SPI_SLV_EX_QPI_INT_ST_R {
        SPI_SLV_EX_QPI_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt status of SPI_SLV_EN_QPI_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_st(&self) -> SPI_SLV_EN_QPI_INT_ST_R {
        SPI_SLV_EN_QPI_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt status of SPI_SLV_CMD7_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_st(&self) -> SPI_SLV_CMD7_INT_ST_R {
        SPI_SLV_CMD7_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt status of SPI_SLV_CMD8_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_st(&self) -> SPI_SLV_CMD8_INT_ST_R {
        SPI_SLV_CMD8_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt status of SPI_SLV_CMD9_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_st(&self) -> SPI_SLV_CMD9_INT_ST_R {
        SPI_SLV_CMD9_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt status of SPI_SLV_CMDA_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmda_int_st(&self) -> SPI_SLV_CMDA_INT_ST_R {
        SPI_SLV_CMDA_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt status of SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_st(&self) -> SPI_SLV_RD_DMA_DONE_INT_ST_R {
        SPI_SLV_RD_DMA_DONE_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt status of SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_st(&self) -> SPI_SLV_WR_DMA_DONE_INT_ST_R {
        SPI_SLV_WR_DMA_DONE_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt status of SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_st(&self) -> SPI_SLV_RD_BUF_DONE_INT_ST_R {
        SPI_SLV_RD_BUF_DONE_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt status of SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_st(&self) -> SPI_SLV_WR_BUF_DONE_INT_ST_R {
        SPI_SLV_WR_BUF_DONE_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt status of SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_trans_done_int_st(&self) -> SPI_TRANS_DONE_INT_ST_R {
        SPI_TRANS_DONE_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt status of SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_st(&self) -> SPI_DMA_SEG_TRANS_DONE_INT_ST_R {
        SPI_DMA_SEG_TRANS_DONE_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt status of SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_st(&self) -> SPI_SEG_MAGIC_ERR_INT_ST_R {
        SPI_SEG_MAGIC_ERR_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_st(&self) -> SPI_SLV_BUF_ADDR_ERR_INT_ST_R {
        SPI_SLV_BUF_ADDR_ERR_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt status of SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_st(&self) -> SPI_SLV_CMD_ERR_INT_ST_R {
        SPI_SLV_CMD_ERR_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt status of SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_st(&self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt status of SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_st(&self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt status of SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn spi_app2_int_st(&self) -> SPI_APP2_INT_ST_R {
        SPI_APP2_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt status of SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn spi_app1_int_st(&self) -> SPI_APP1_INT_ST_R {
        SPI_APP1_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_DMA_INT_ST")
            .field(
                "spi_dma_infifo_full_err_int_st",
                &self.spi_dma_infifo_full_err_int_st(),
            )
            .field(
                "spi_dma_outfifo_empty_err_int_st",
                &self.spi_dma_outfifo_empty_err_int_st(),
            )
            .field("spi_slv_ex_qpi_int_st", &self.spi_slv_ex_qpi_int_st())
            .field("spi_slv_en_qpi_int_st", &self.spi_slv_en_qpi_int_st())
            .field("spi_slv_cmd7_int_st", &self.spi_slv_cmd7_int_st())
            .field("spi_slv_cmd8_int_st", &self.spi_slv_cmd8_int_st())
            .field("spi_slv_cmd9_int_st", &self.spi_slv_cmd9_int_st())
            .field("spi_slv_cmda_int_st", &self.spi_slv_cmda_int_st())
            .field(
                "spi_slv_rd_dma_done_int_st",
                &self.spi_slv_rd_dma_done_int_st(),
            )
            .field(
                "spi_slv_wr_dma_done_int_st",
                &self.spi_slv_wr_dma_done_int_st(),
            )
            .field(
                "spi_slv_rd_buf_done_int_st",
                &self.spi_slv_rd_buf_done_int_st(),
            )
            .field(
                "spi_slv_wr_buf_done_int_st",
                &self.spi_slv_wr_buf_done_int_st(),
            )
            .field("spi_trans_done_int_st", &self.spi_trans_done_int_st())
            .field(
                "spi_dma_seg_trans_done_int_st",
                &self.spi_dma_seg_trans_done_int_st(),
            )
            .field("spi_seg_magic_err_int_st", &self.spi_seg_magic_err_int_st())
            .field(
                "spi_slv_buf_addr_err_int_st",
                &self.spi_slv_buf_addr_err_int_st(),
            )
            .field("spi_slv_cmd_err_int_st", &self.spi_slv_cmd_err_int_st())
            .field(
                "spi_mst_rx_afifo_wfull_err_int_st",
                &self.spi_mst_rx_afifo_wfull_err_int_st(),
            )
            .field(
                "spi_mst_tx_afifo_rempty_err_int_st",
                &self.spi_mst_tx_afifo_rempty_err_int_st(),
            )
            .field("spi_app2_int_st", &self.spi_app2_int_st())
            .field("spi_app1_int_st", &self.spi_app1_int_st())
            .finish()
    }
}
#[doc = "SPI interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`spi_dma_int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPI_DMA_INT_ST_SPEC;
impl crate::RegisterSpec for SPI_DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`spi_dma_int_st::R`](R) reader structure"]
impl crate::Readable for SPI_DMA_INT_ST_SPEC {}
#[doc = "`reset()` method sets SPI_DMA_INT_ST to value 0"]
impl crate::Resettable for SPI_DMA_INT_ST_SPEC {}
