#[doc = "Register `DMA_INT_ST` reader"]
pub struct R(crate::R<DMA_INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_ST` reader - The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DMA_INFIFO_FULL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_ST` reader - The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_EX_QPI_INT_ST` reader - The status bit for SPI slave Ex_QPI interrupt."]
pub type SLV_EX_QPI_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_EN_QPI_INT_ST` reader - The status bit for SPI slave En_QPI interrupt."]
pub type SLV_EN_QPI_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD7_INT_ST` reader - The status bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD8_INT_ST` reader - The status bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD9_INT_ST` reader - The status bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMDA_INT_ST` reader - The status bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_RD_DMA_DONE_INT_ST` reader - The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SLV_RD_DMA_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_WR_DMA_DONE_INT_ST` reader - The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SLV_WR_DMA_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE_INT_ST` reader - The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SLV_RD_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE_INT_ST` reader - The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SLV_WR_BUF_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `TRANS_DONE_INT_ST` reader - The status bit for SPI_TRANS_DONE_INT interrupt."]
pub type TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_ST` reader - The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DMA_SEG_TRANS_DONE_INT_ST_R = crate::BitReader;
#[doc = "Field `SEG_MAGIC_ERR_INT_ST` reader - The status bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SEG_MAGIC_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_ST` reader - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SLV_BUF_ADDR_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR_INT_ST` reader - The status bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SLV_CMD_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_ST` reader - The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MST_RX_AFIFO_WFULL_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_ST` reader - The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `APP2_INT_ST` reader - The status bit for SPI_APP2_INT interrupt."]
pub type APP2_INT_ST_R = crate::BitReader;
#[doc = "Field `APP1_INT_ST` reader - The status bit for SPI_APP1_INT interrupt."]
pub type APP1_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_st(&self) -> DMA_INFIFO_FULL_ERR_INT_ST_R {
        DMA_INFIFO_FULL_ERR_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_st(&self) -> DMA_OUTFIFO_EMPTY_ERR_INT_ST_R {
        DMA_OUTFIFO_EMPTY_ERR_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn slv_ex_qpi_int_st(&self) -> SLV_EX_QPI_INT_ST_R {
        SLV_EX_QPI_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn slv_en_qpi_int_st(&self) -> SLV_EN_QPI_INT_ST_R {
        SLV_EN_QPI_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_st(&self) -> SLV_CMD7_INT_ST_R {
        SLV_CMD7_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The status bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_st(&self) -> SLV_CMD8_INT_ST_R {
        SLV_CMD8_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The status bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_st(&self) -> SLV_CMD9_INT_ST_R {
        SLV_CMD9_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The status bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_st(&self) -> SLV_CMDA_INT_ST_R {
        SLV_CMDA_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The status bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_st(&self) -> SLV_RD_DMA_DONE_INT_ST_R {
        SLV_RD_DMA_DONE_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The status bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_st(&self) -> SLV_WR_DMA_DONE_INT_ST_R {
        SLV_WR_DMA_DONE_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The status bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_st(&self) -> SLV_RD_BUF_DONE_INT_ST_R {
        SLV_RD_BUF_DONE_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The status bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_st(&self) -> SLV_WR_BUF_DONE_INT_ST_R {
        SLV_WR_BUF_DONE_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The status bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn trans_done_int_st(&self) -> TRANS_DONE_INT_ST_R {
        TRANS_DONE_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The status bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_st(&self) -> DMA_SEG_TRANS_DONE_INT_ST_R {
        DMA_SEG_TRANS_DONE_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The status bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn seg_magic_err_int_st(&self) -> SEG_MAGIC_ERR_INT_ST_R {
        SEG_MAGIC_ERR_INT_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The status bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_st(&self) -> SLV_BUF_ADDR_ERR_INT_ST_R {
        SLV_BUF_ADDR_ERR_INT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The status bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd_err_int_st(&self) -> SLV_CMD_ERR_INT_ST_R {
        SLV_CMD_ERR_INT_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The status bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_st(&self) -> MST_RX_AFIFO_WFULL_ERR_INT_ST_R {
        MST_RX_AFIFO_WFULL_ERR_INT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The status bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_st(&self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ST_R {
        MST_TX_AFIFO_REMPTY_ERR_INT_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The status bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn app2_int_st(&self) -> APP2_INT_ST_R {
        APP2_INT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The status bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn app1_int_st(&self) -> APP1_INT_ST_R {
        APP1_INT_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ST")
            .field(
                "dma_infifo_full_err_int_st",
                &format_args!("{}", self.dma_infifo_full_err_int_st().bit()),
            )
            .field(
                "dma_outfifo_empty_err_int_st",
                &format_args!("{}", self.dma_outfifo_empty_err_int_st().bit()),
            )
            .field(
                "slv_ex_qpi_int_st",
                &format_args!("{}", self.slv_ex_qpi_int_st().bit()),
            )
            .field(
                "slv_en_qpi_int_st",
                &format_args!("{}", self.slv_en_qpi_int_st().bit()),
            )
            .field(
                "slv_cmd7_int_st",
                &format_args!("{}", self.slv_cmd7_int_st().bit()),
            )
            .field(
                "slv_cmd8_int_st",
                &format_args!("{}", self.slv_cmd8_int_st().bit()),
            )
            .field(
                "slv_cmd9_int_st",
                &format_args!("{}", self.slv_cmd9_int_st().bit()),
            )
            .field(
                "slv_cmda_int_st",
                &format_args!("{}", self.slv_cmda_int_st().bit()),
            )
            .field(
                "slv_rd_dma_done_int_st",
                &format_args!("{}", self.slv_rd_dma_done_int_st().bit()),
            )
            .field(
                "slv_wr_dma_done_int_st",
                &format_args!("{}", self.slv_wr_dma_done_int_st().bit()),
            )
            .field(
                "slv_rd_buf_done_int_st",
                &format_args!("{}", self.slv_rd_buf_done_int_st().bit()),
            )
            .field(
                "slv_wr_buf_done_int_st",
                &format_args!("{}", self.slv_wr_buf_done_int_st().bit()),
            )
            .field(
                "trans_done_int_st",
                &format_args!("{}", self.trans_done_int_st().bit()),
            )
            .field(
                "dma_seg_trans_done_int_st",
                &format_args!("{}", self.dma_seg_trans_done_int_st().bit()),
            )
            .field(
                "seg_magic_err_int_st",
                &format_args!("{}", self.seg_magic_err_int_st().bit()),
            )
            .field(
                "slv_buf_addr_err_int_st",
                &format_args!("{}", self.slv_buf_addr_err_int_st().bit()),
            )
            .field(
                "slv_cmd_err_int_st",
                &format_args!("{}", self.slv_cmd_err_int_st().bit()),
            )
            .field(
                "mst_rx_afifo_wfull_err_int_st",
                &format_args!("{}", self.mst_rx_afifo_wfull_err_int_st().bit()),
            )
            .field(
                "mst_tx_afifo_rempty_err_int_st",
                &format_args!("{}", self.mst_tx_afifo_rempty_err_int_st().bit()),
            )
            .field("app2_int_st", &format_args!("{}", self.app2_int_st().bit()))
            .field("app1_int_st", &format_args!("{}", self.app1_int_st().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SPI interrupt status register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_st](index.html) module"]
pub struct DMA_INT_ST_SPEC;
impl crate::RegisterSpec for DMA_INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_st::R](R) reader structure"]
impl crate::Readable for DMA_INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INT_ST to value 0"]
impl crate::Resettable for DMA_INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
