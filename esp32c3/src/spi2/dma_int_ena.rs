#[doc = "Register `DMA_INT_ENA` reader"]
pub struct R(crate::R<DMA_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_INT_ENA` writer"]
pub struct W(crate::W<DMA_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<DMA_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_ENA` reader - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DMA_INFIFO_FULL_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `DMA_INFIFO_FULL_ERR_INT_ENA` writer - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type DMA_INFIFO_FULL_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_ENA` reader - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `DMA_OUTFIFO_EMPTY_ERR_INT_ENA` writer - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_EX_QPI_INT_ENA` reader - The enable bit for SPI slave Ex_QPI interrupt."]
pub type SLV_EX_QPI_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_EX_QPI_INT_ENA` writer - The enable bit for SPI slave Ex_QPI interrupt."]
pub type SLV_EX_QPI_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_EN_QPI_INT_ENA` reader - The enable bit for SPI slave En_QPI interrupt."]
pub type SLV_EN_QPI_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_EN_QPI_INT_ENA` writer - The enable bit for SPI slave En_QPI interrupt."]
pub type SLV_EN_QPI_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_CMD7_INT_ENA` reader - The enable bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD7_INT_ENA` writer - The enable bit for SPI slave CMD7 interrupt."]
pub type SLV_CMD7_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_CMD8_INT_ENA` reader - The enable bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD8_INT_ENA` writer - The enable bit for SPI slave CMD8 interrupt."]
pub type SLV_CMD8_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_CMD9_INT_ENA` reader - The enable bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD9_INT_ENA` writer - The enable bit for SPI slave CMD9 interrupt."]
pub type SLV_CMD9_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_CMDA_INT_ENA` reader - The enable bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMDA_INT_ENA` writer - The enable bit for SPI slave CMDA interrupt."]
pub type SLV_CMDA_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_RD_DMA_DONE_INT_ENA` reader - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SLV_RD_DMA_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_RD_DMA_DONE_INT_ENA` writer - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SLV_RD_DMA_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_WR_DMA_DONE_INT_ENA` reader - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SLV_WR_DMA_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_WR_DMA_DONE_INT_ENA` writer - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SLV_WR_DMA_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_RD_BUF_DONE_INT_ENA` reader - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SLV_RD_BUF_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_RD_BUF_DONE_INT_ENA` writer - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SLV_RD_BUF_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_WR_BUF_DONE_INT_ENA` reader - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SLV_WR_BUF_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_WR_BUF_DONE_INT_ENA` writer - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SLV_WR_BUF_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `TRANS_DONE_INT_ENA` reader - The enable bit for SPI_TRANS_DONE_INT interrupt."]
pub type TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_DONE_INT_ENA` writer - The enable bit for SPI_TRANS_DONE_INT interrupt."]
pub type TRANS_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_ENA` reader - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DMA_SEG_TRANS_DONE_INT_ENA_R = crate::BitReader;
#[doc = "Field `DMA_SEG_TRANS_DONE_INT_ENA` writer - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type DMA_SEG_TRANS_DONE_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SEG_MAGIC_ERR_INT_ENA` reader - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SEG_MAGIC_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SEG_MAGIC_ERR_INT_ENA` writer - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SEG_MAGIC_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_ENA` reader - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SLV_BUF_ADDR_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_BUF_ADDR_ERR_INT_ENA` writer - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SLV_BUF_ADDR_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `SLV_CMD_ERR_INT_ENA` reader - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SLV_CMD_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLV_CMD_ERR_INT_ENA` writer - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SLV_CMD_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_ENA` reader - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MST_RX_AFIFO_WFULL_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `MST_RX_AFIFO_WFULL_ERR_INT_ENA` writer - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_ENA` reader - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `MST_TX_AFIFO_REMPTY_ERR_INT_ENA` writer - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `APP2_INT_ENA` reader - The enable bit for SPI_APP2_INT interrupt."]
pub type APP2_INT_ENA_R = crate::BitReader;
#[doc = "Field `APP2_INT_ENA` writer - The enable bit for SPI_APP2_INT interrupt."]
pub type APP2_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
#[doc = "Field `APP1_INT_ENA` reader - The enable bit for SPI_APP1_INT interrupt."]
pub type APP1_INT_ENA_R = crate::BitReader;
#[doc = "Field `APP1_INT_ENA` writer - The enable bit for SPI_APP1_INT interrupt."]
pub type APP1_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_infifo_full_err_int_ena(&self) -> DMA_INFIFO_FULL_ERR_INT_ENA_R {
        DMA_INFIFO_FULL_ERR_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn dma_outfifo_empty_err_int_ena(&self) -> DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R {
        DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn slv_ex_qpi_int_ena(&self) -> SLV_EX_QPI_INT_ENA_R {
        SLV_EX_QPI_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn slv_en_qpi_int_ena(&self) -> SLV_EN_QPI_INT_ENA_R {
        SLV_EN_QPI_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn slv_cmd7_int_ena(&self) -> SLV_CMD7_INT_ENA_R {
        SLV_CMD7_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn slv_cmd8_int_ena(&self) -> SLV_CMD8_INT_ENA_R {
        SLV_CMD8_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn slv_cmd9_int_ena(&self) -> SLV_CMD9_INT_ENA_R {
        SLV_CMD9_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn slv_cmda_int_ena(&self) -> SLV_CMDA_INT_ENA_R {
        SLV_CMDA_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_dma_done_int_ena(&self) -> SLV_RD_DMA_DONE_INT_ENA_R {
        SLV_RD_DMA_DONE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_dma_done_int_ena(&self) -> SLV_WR_DMA_DONE_INT_ENA_R {
        SLV_WR_DMA_DONE_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_rd_buf_done_int_ena(&self) -> SLV_RD_BUF_DONE_INT_ENA_R {
        SLV_RD_BUF_DONE_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn slv_wr_buf_done_int_ena(&self) -> SLV_WR_BUF_DONE_INT_ENA_R {
        SLV_WR_BUF_DONE_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn trans_done_int_ena(&self) -> TRANS_DONE_INT_ENA_R {
        TRANS_DONE_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn dma_seg_trans_done_int_ena(&self) -> DMA_SEG_TRANS_DONE_INT_ENA_R {
        DMA_SEG_TRANS_DONE_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn seg_magic_err_int_ena(&self) -> SEG_MAGIC_ERR_INT_ENA_R {
        SEG_MAGIC_ERR_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_buf_addr_err_int_ena(&self) -> SLV_BUF_ADDR_ERR_INT_ENA_R {
        SLV_BUF_ADDR_ERR_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn slv_cmd_err_int_ena(&self) -> SLV_CMD_ERR_INT_ENA_R {
        SLV_CMD_ERR_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_rx_afifo_wfull_err_int_ena(&self) -> MST_RX_AFIFO_WFULL_ERR_INT_ENA_R {
        MST_RX_AFIFO_WFULL_ERR_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn mst_tx_afifo_rempty_err_int_ena(&self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R {
        MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The enable bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn app2_int_ena(&self) -> APP2_INT_ENA_R {
        APP2_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The enable bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn app1_int_ena(&self) -> APP1_INT_ENA_R {
        APP1_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_ENA")
            .field(
                "dma_infifo_full_err_int_ena",
                &format_args!("{}", self.dma_infifo_full_err_int_ena().bit()),
            )
            .field(
                "dma_outfifo_empty_err_int_ena",
                &format_args!("{}", self.dma_outfifo_empty_err_int_ena().bit()),
            )
            .field(
                "slv_ex_qpi_int_ena",
                &format_args!("{}", self.slv_ex_qpi_int_ena().bit()),
            )
            .field(
                "slv_en_qpi_int_ena",
                &format_args!("{}", self.slv_en_qpi_int_ena().bit()),
            )
            .field(
                "slv_cmd7_int_ena",
                &format_args!("{}", self.slv_cmd7_int_ena().bit()),
            )
            .field(
                "slv_cmd8_int_ena",
                &format_args!("{}", self.slv_cmd8_int_ena().bit()),
            )
            .field(
                "slv_cmd9_int_ena",
                &format_args!("{}", self.slv_cmd9_int_ena().bit()),
            )
            .field(
                "slv_cmda_int_ena",
                &format_args!("{}", self.slv_cmda_int_ena().bit()),
            )
            .field(
                "slv_rd_dma_done_int_ena",
                &format_args!("{}", self.slv_rd_dma_done_int_ena().bit()),
            )
            .field(
                "slv_wr_dma_done_int_ena",
                &format_args!("{}", self.slv_wr_dma_done_int_ena().bit()),
            )
            .field(
                "slv_rd_buf_done_int_ena",
                &format_args!("{}", self.slv_rd_buf_done_int_ena().bit()),
            )
            .field(
                "slv_wr_buf_done_int_ena",
                &format_args!("{}", self.slv_wr_buf_done_int_ena().bit()),
            )
            .field(
                "trans_done_int_ena",
                &format_args!("{}", self.trans_done_int_ena().bit()),
            )
            .field(
                "dma_seg_trans_done_int_ena",
                &format_args!("{}", self.dma_seg_trans_done_int_ena().bit()),
            )
            .field(
                "seg_magic_err_int_ena",
                &format_args!("{}", self.seg_magic_err_int_ena().bit()),
            )
            .field(
                "slv_buf_addr_err_int_ena",
                &format_args!("{}", self.slv_buf_addr_err_int_ena().bit()),
            )
            .field(
                "slv_cmd_err_int_ena",
                &format_args!("{}", self.slv_cmd_err_int_ena().bit()),
            )
            .field(
                "mst_rx_afifo_wfull_err_int_ena",
                &format_args!("{}", self.mst_rx_afifo_wfull_err_int_ena().bit()),
            )
            .field(
                "mst_tx_afifo_rempty_err_int_ena",
                &format_args!("{}", self.mst_tx_afifo_rempty_err_int_ena().bit()),
            )
            .field(
                "app2_int_ena",
                &format_args!("{}", self.app2_int_ena().bit()),
            )
            .field(
                "app1_int_ena",
                &format_args!("{}", self.app1_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_err_int_ena(&mut self) -> DMA_INFIFO_FULL_ERR_INT_ENA_W<0> {
        DMA_INFIFO_FULL_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_outfifo_empty_err_int_ena(&mut self) -> DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<1> {
        DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The enable bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_ex_qpi_int_ena(&mut self) -> SLV_EX_QPI_INT_ENA_W<2> {
        SLV_EX_QPI_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The enable bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_en_qpi_int_ena(&mut self) -> SLV_EN_QPI_INT_ENA_W<3> {
        SLV_EN_QPI_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The enable bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd7_int_ena(&mut self) -> SLV_CMD7_INT_ENA_W<4> {
        SLV_CMD7_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The enable bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd8_int_ena(&mut self) -> SLV_CMD8_INT_ENA_W<5> {
        SLV_CMD8_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The enable bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd9_int_ena(&mut self) -> SLV_CMD9_INT_ENA_W<6> {
        SLV_CMD9_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The enable bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmda_int_ena(&mut self) -> SLV_CMDA_INT_ENA_W<7> {
        SLV_CMDA_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_dma_done_int_ena(&mut self) -> SLV_RD_DMA_DONE_INT_ENA_W<8> {
        SLV_RD_DMA_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_dma_done_int_ena(&mut self) -> SLV_WR_DMA_DONE_INT_ENA_W<9> {
        SLV_WR_DMA_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rd_buf_done_int_ena(&mut self) -> SLV_RD_BUF_DONE_INT_ENA_W<10> {
        SLV_RD_BUF_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_wr_buf_done_int_ena(&mut self) -> SLV_WR_BUF_DONE_INT_ENA_W<11> {
        SLV_WR_BUF_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The enable bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn trans_done_int_ena(&mut self) -> TRANS_DONE_INT_ENA_W<12> {
        TRANS_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn dma_seg_trans_done_int_ena(&mut self) -> DMA_SEG_TRANS_DONE_INT_ENA_W<13> {
        DMA_SEG_TRANS_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn seg_magic_err_int_ena(&mut self) -> SEG_MAGIC_ERR_INT_ENA_W<14> {
        SEG_MAGIC_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_buf_addr_err_int_ena(&mut self) -> SLV_BUF_ADDR_ERR_INT_ENA_W<15> {
        SLV_BUF_ADDR_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn slv_cmd_err_int_ena(&mut self) -> SLV_CMD_ERR_INT_ENA_W<16> {
        SLV_CMD_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_rx_afifo_wfull_err_int_ena(&mut self) -> MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<17> {
        MST_RX_AFIFO_WFULL_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn mst_tx_afifo_rempty_err_int_ena(&mut self) -> MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<18> {
        MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - The enable bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app2_int_ena(&mut self) -> APP2_INT_ENA_W<19> {
        APP2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - The enable bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn app1_int_ena(&mut self) -> APP1_INT_ENA_W<20> {
        APP1_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_ena](index.html) module"]
pub struct DMA_INT_ENA_SPEC;
impl crate::RegisterSpec for DMA_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_ena::R](R) reader structure"]
impl crate::Readable for DMA_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_int_ena::W](W) writer structure"]
impl crate::Writable for DMA_INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_INT_ENA to value 0"]
impl crate::Resettable for DMA_INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
