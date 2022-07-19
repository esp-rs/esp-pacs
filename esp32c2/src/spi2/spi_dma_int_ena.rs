#[doc = "Register `SPI_DMA_INT_ENA` reader"]
pub struct R(crate::R<SPI_DMA_INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_DMA_INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_DMA_INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_DMA_INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_DMA_INT_ENA` writer"]
pub struct W(crate::W<SPI_DMA_INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_DMA_INT_ENA_SPEC>;
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
impl From<crate::W<SPI_DMA_INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_DMA_INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_ENA` reader - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_ENA` writer - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 0>;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA` reader - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA` writer - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 1>;
#[doc = "Field `SPI_SLV_EX_QPI_INT_ENA` reader - The enable bit for SPI slave Ex_QPI interrupt."]
pub type SPI_SLV_EX_QPI_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_EX_QPI_INT_ENA` writer - The enable bit for SPI slave Ex_QPI interrupt."]
pub type SPI_SLV_EX_QPI_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 2>;
#[doc = "Field `SPI_SLV_EN_QPI_INT_ENA` reader - The enable bit for SPI slave En_QPI interrupt."]
pub type SPI_SLV_EN_QPI_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_EN_QPI_INT_ENA` writer - The enable bit for SPI slave En_QPI interrupt."]
pub type SPI_SLV_EN_QPI_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 3>;
#[doc = "Field `SPI_SLV_CMD7_INT_ENA` reader - The enable bit for SPI slave CMD7 interrupt."]
pub type SPI_SLV_CMD7_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_CMD7_INT_ENA` writer - The enable bit for SPI slave CMD7 interrupt."]
pub type SPI_SLV_CMD7_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 4>;
#[doc = "Field `SPI_SLV_CMD8_INT_ENA` reader - The enable bit for SPI slave CMD8 interrupt."]
pub type SPI_SLV_CMD8_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_CMD8_INT_ENA` writer - The enable bit for SPI slave CMD8 interrupt."]
pub type SPI_SLV_CMD8_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 5>;
#[doc = "Field `SPI_SLV_CMD9_INT_ENA` reader - The enable bit for SPI slave CMD9 interrupt."]
pub type SPI_SLV_CMD9_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_CMD9_INT_ENA` writer - The enable bit for SPI slave CMD9 interrupt."]
pub type SPI_SLV_CMD9_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 6>;
#[doc = "Field `SPI_SLV_CMDA_INT_ENA` reader - The enable bit for SPI slave CMDA interrupt."]
pub type SPI_SLV_CMDA_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_CMDA_INT_ENA` writer - The enable bit for SPI slave CMDA interrupt."]
pub type SPI_SLV_CMDA_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 7>;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_ENA` reader - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SPI_SLV_RD_DMA_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_ENA` writer - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SPI_SLV_RD_DMA_DONE_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 8>;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_ENA` reader - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SPI_SLV_WR_DMA_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_ENA` writer - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SPI_SLV_WR_DMA_DONE_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 9>;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_ENA` reader - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SPI_SLV_RD_BUF_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_ENA` writer - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SPI_SLV_RD_BUF_DONE_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 10>;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_ENA` reader - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SPI_SLV_WR_BUF_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_ENA` writer - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SPI_SLV_WR_BUF_DONE_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 11>;
#[doc = "Field `SPI_TRANS_DONE_INT_ENA` reader - The enable bit for SPI_TRANS_DONE_INT interrupt."]
pub type SPI_TRANS_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_TRANS_DONE_INT_ENA` writer - The enable bit for SPI_TRANS_DONE_INT interrupt."]
pub type SPI_TRANS_DONE_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 12>;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_ENA` reader - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_ENA` writer - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 13>;
#[doc = "Field `SPI_SEG_MAGIC_ERR_INT_ENA` reader - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SPI_SEG_MAGIC_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SEG_MAGIC_ERR_INT_ENA` writer - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SPI_SEG_MAGIC_ERR_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 14>;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_ENA` reader - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_ENA` writer - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 15>;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_ENA` reader - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SPI_SLV_CMD_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_ENA` writer - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SPI_SLV_CMD_ERR_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 16>;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA` reader - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA` writer - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 17>;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA` reader - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA` writer - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W<'a> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 18>;
#[doc = "Field `SPI_APP2_INT_ENA` reader - The enable bit for SPI_APP2_INT interrupt."]
pub type SPI_APP2_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_APP2_INT_ENA` writer - The enable bit for SPI_APP2_INT interrupt."]
pub type SPI_APP2_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 19>;
#[doc = "Field `SPI_APP1_INT_ENA` reader - The enable bit for SPI_APP1_INT interrupt."]
pub type SPI_APP1_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_APP1_INT_ENA` writer - The enable bit for SPI_APP1_INT interrupt."]
pub type SPI_APP1_INT_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_INT_ENA_SPEC, bool, 20>;
impl R {
    #[doc = "Bit 0 - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_ena(&self) -> SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R {
        SPI_DMA_INFIFO_FULL_ERR_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_ena(&self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The enable bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_ena(&self) -> SPI_SLV_EX_QPI_INT_ENA_R {
        SPI_SLV_EX_QPI_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The enable bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_ena(&self) -> SPI_SLV_EN_QPI_INT_ENA_R {
        SPI_SLV_EN_QPI_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The enable bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_ena(&self) -> SPI_SLV_CMD7_INT_ENA_R {
        SPI_SLV_CMD7_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The enable bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_ena(&self) -> SPI_SLV_CMD8_INT_ENA_R {
        SPI_SLV_CMD8_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The enable bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_ena(&self) -> SPI_SLV_CMD9_INT_ENA_R {
        SPI_SLV_CMD9_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The enable bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmda_int_ena(&self) -> SPI_SLV_CMDA_INT_ENA_R {
        SPI_SLV_CMDA_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_ena(&self) -> SPI_SLV_RD_DMA_DONE_INT_ENA_R {
        SPI_SLV_RD_DMA_DONE_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_ena(&self) -> SPI_SLV_WR_DMA_DONE_INT_ENA_R {
        SPI_SLV_WR_DMA_DONE_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_ena(&self) -> SPI_SLV_RD_BUF_DONE_INT_ENA_R {
        SPI_SLV_RD_BUF_DONE_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_ena(&self) -> SPI_SLV_WR_BUF_DONE_INT_ENA_R {
        SPI_SLV_WR_BUF_DONE_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The enable bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_trans_done_int_ena(&self) -> SPI_TRANS_DONE_INT_ENA_R {
        SPI_TRANS_DONE_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_ena(&self) -> SPI_DMA_SEG_TRANS_DONE_INT_ENA_R {
        SPI_DMA_SEG_TRANS_DONE_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_ena(&self) -> SPI_SEG_MAGIC_ERR_INT_ENA_R {
        SPI_SEG_MAGIC_ERR_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_ena(&self) -> SPI_SLV_BUF_ADDR_ERR_INT_ENA_R {
        SPI_SLV_BUF_ADDR_ERR_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_ena(&self) -> SPI_SLV_CMD_ERR_INT_ENA_R {
        SPI_SLV_CMD_ERR_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_ena(&self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_ena(&self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The enable bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn spi_app2_int_ena(&self) -> SPI_APP2_INT_ENA_R {
        SPI_APP2_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The enable bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn spi_app1_int_ena(&self) -> SPI_APP1_INT_ENA_R {
        SPI_APP1_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - The enable bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_infifo_full_err_int_ena(&mut self) -> SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W {
        SPI_DMA_INFIFO_FULL_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The enable bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty_err_int_ena(&mut self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The enable bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    pub fn spi_slv_ex_qpi_int_ena(&mut self) -> SPI_SLV_EX_QPI_INT_ENA_W {
        SPI_SLV_EX_QPI_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The enable bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    pub fn spi_slv_en_qpi_int_ena(&mut self) -> SPI_SLV_EN_QPI_INT_ENA_W {
        SPI_SLV_EN_QPI_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The enable bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd7_int_ena(&mut self) -> SPI_SLV_CMD7_INT_ENA_W {
        SPI_SLV_CMD7_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The enable bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd8_int_ena(&mut self) -> SPI_SLV_CMD8_INT_ENA_W {
        SPI_SLV_CMD8_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The enable bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd9_int_ena(&mut self) -> SPI_SLV_CMD9_INT_ENA_W {
        SPI_SLV_CMD9_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The enable bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmda_int_ena(&mut self) -> SPI_SLV_CMDA_INT_ENA_W {
        SPI_SLV_CMDA_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The enable bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_dma_done_int_ena(&mut self) -> SPI_SLV_RD_DMA_DONE_INT_ENA_W {
        SPI_SLV_RD_DMA_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The enable bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_dma_done_int_ena(&mut self) -> SPI_SLV_WR_DMA_DONE_INT_ENA_W {
        SPI_SLV_WR_DMA_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The enable bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_rd_buf_done_int_ena(&mut self) -> SPI_SLV_RD_BUF_DONE_INT_ENA_W {
        SPI_SLV_RD_BUF_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The enable bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_wr_buf_done_int_ena(&mut self) -> SPI_SLV_WR_BUF_DONE_INT_ENA_W {
        SPI_SLV_WR_BUF_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The enable bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_trans_done_int_ena(&mut self) -> SPI_TRANS_DONE_INT_ENA_W {
        SPI_TRANS_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The enable bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    pub fn spi_dma_seg_trans_done_int_ena(&mut self) -> SPI_DMA_SEG_TRANS_DONE_INT_ENA_W {
        SPI_DMA_SEG_TRANS_DONE_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The enable bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_seg_magic_err_int_ena(&mut self) -> SPI_SEG_MAGIC_ERR_INT_ENA_W {
        SPI_SEG_MAGIC_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The enable bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_buf_addr_err_int_ena(&mut self) -> SPI_SLV_BUF_ADDR_ERR_INT_ENA_W {
        SPI_SLV_BUF_ADDR_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - The enable bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_slv_cmd_err_int_ena(&mut self) -> SPI_SLV_CMD_ERR_INT_ENA_W {
        SPI_SLV_CMD_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - The enable bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_rx_afifo_wfull_err_int_ena(&mut self) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - The enable bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    pub fn spi_mst_tx_afifo_rempty_err_int_ena(&mut self) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - The enable bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    pub fn spi_app2_int_ena(&mut self) -> SPI_APP2_INT_ENA_W {
        SPI_APP2_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - The enable bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    pub fn spi_app1_int_ena(&mut self) -> SPI_APP1_INT_ENA_W {
        SPI_APP1_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI interrupt enable register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_int_ena](index.html) module"]
pub struct SPI_DMA_INT_ENA_SPEC;
impl crate::RegisterSpec for SPI_DMA_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_dma_int_ena::R](R) reader structure"]
impl crate::Readable for SPI_DMA_INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_ena::W](W) writer structure"]
impl crate::Writable for SPI_DMA_INT_ENA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_DMA_INT_ENA to value 0"]
impl crate::Resettable for SPI_DMA_INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
