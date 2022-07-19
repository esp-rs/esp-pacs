#[doc = "Register `SPI_DMA_CONF` reader"]
pub struct R(crate::R<SPI_DMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_DMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_DMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_DMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_DMA_CONF` writer"]
pub struct W(crate::W<SPI_DMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_DMA_CONF_SPEC>;
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
impl From<crate::W<SPI_DMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_DMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY` reader - Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data."]
pub type SPI_DMA_OUTFIFO_EMPTY_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_INFIFO_FULL` reader - Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data."]
pub type SPI_DMA_INFIFO_FULL_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_SLV_SEG_TRANS_EN` reader - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub type SPI_DMA_SLV_SEG_TRANS_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_SLV_SEG_TRANS_EN` writer - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub type SPI_DMA_SLV_SEG_TRANS_EN_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 18>;
#[doc = "Field `SPI_SLV_RX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
pub type SPI_SLV_RX_SEG_TRANS_CLR_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_RX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
pub type SPI_SLV_RX_SEG_TRANS_CLR_EN_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 19>;
#[doc = "Field `SPI_SLV_TX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
pub type SPI_SLV_TX_SEG_TRANS_CLR_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_SLV_TX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
pub type SPI_SLV_TX_SEG_TRANS_CLR_EN_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 20>;
#[doc = "Field `SPI_RX_EOF_EN` reader - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
pub type SPI_RX_EOF_EN_R = crate::BitReader<bool>;
#[doc = "Field `SPI_RX_EOF_EN` writer - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
pub type SPI_RX_EOF_EN_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 21>;
#[doc = "Field `SPI_DMA_RX_ENA` reader - Set this bit to enable SPI DMA controlled receive data mode."]
pub type SPI_DMA_RX_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_RX_ENA` writer - Set this bit to enable SPI DMA controlled receive data mode."]
pub type SPI_DMA_RX_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 27>;
#[doc = "Field `SPI_DMA_TX_ENA` reader - Set this bit to enable SPI DMA controlled send data mode."]
pub type SPI_DMA_TX_ENA_R = crate::BitReader<bool>;
#[doc = "Field `SPI_DMA_TX_ENA` writer - Set this bit to enable SPI DMA controlled send data mode."]
pub type SPI_DMA_TX_ENA_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 28>;
#[doc = "Field `SPI_RX_AFIFO_RST` writer - Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer."]
pub type SPI_RX_AFIFO_RST_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 29>;
#[doc = "Field `SPI_BUF_AFIFO_RST` writer - Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer."]
pub type SPI_BUF_AFIFO_RST_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 30>;
#[doc = "Field `SPI_DMA_AFIFO_RST` writer - Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer."]
pub type SPI_DMA_AFIFO_RST_W<'a> = crate::BitWriter<'a, u32, SPI_DMA_CONF_SPEC, bool, 31>;
impl R {
    #[doc = "Bit 0 - Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data."]
    #[inline(always)]
    pub fn spi_dma_outfifo_empty(&self) -> SPI_DMA_OUTFIFO_EMPTY_R {
        SPI_DMA_OUTFIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data."]
    #[inline(always)]
    pub fn spi_dma_infifo_full(&self) -> SPI_DMA_INFIFO_FULL_R {
        SPI_DMA_INFIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    pub fn spi_dma_slv_seg_trans_en(&self) -> SPI_DMA_SLV_SEG_TRANS_EN_R {
        SPI_DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
    #[inline(always)]
    pub fn spi_slv_rx_seg_trans_clr_en(&self) -> SPI_SLV_RX_SEG_TRANS_CLR_EN_R {
        SPI_SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
    #[inline(always)]
    pub fn spi_slv_tx_seg_trans_clr_en(&self) -> SPI_SLV_TX_SEG_TRANS_CLR_EN_R {
        SPI_SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
    #[inline(always)]
    pub fn spi_rx_eof_en(&self) -> SPI_RX_EOF_EN_R {
        SPI_RX_EOF_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable SPI DMA controlled receive data mode."]
    #[inline(always)]
    pub fn spi_dma_rx_ena(&self) -> SPI_DMA_RX_ENA_R {
        SPI_DMA_RX_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable SPI DMA controlled send data mode."]
    #[inline(always)]
    pub fn spi_dma_tx_ena(&self) -> SPI_DMA_TX_ENA_R {
        SPI_DMA_TX_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    pub fn spi_dma_slv_seg_trans_en(&mut self) -> SPI_DMA_SLV_SEG_TRANS_EN_W {
        SPI_DMA_SLV_SEG_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
    #[inline(always)]
    pub fn spi_slv_rx_seg_trans_clr_en(&mut self) -> SPI_SLV_RX_SEG_TRANS_CLR_EN_W {
        SPI_SLV_RX_SEG_TRANS_CLR_EN_W::new(self)
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
    #[inline(always)]
    pub fn spi_slv_tx_seg_trans_clr_en(&mut self) -> SPI_SLV_TX_SEG_TRANS_CLR_EN_W {
        SPI_SLV_TX_SEG_TRANS_CLR_EN_W::new(self)
    }
    #[doc = "Bit 21 - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
    #[inline(always)]
    pub fn spi_rx_eof_en(&mut self) -> SPI_RX_EOF_EN_W {
        SPI_RX_EOF_EN_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to enable SPI DMA controlled receive data mode."]
    #[inline(always)]
    pub fn spi_dma_rx_ena(&mut self) -> SPI_DMA_RX_ENA_W {
        SPI_DMA_RX_ENA_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to enable SPI DMA controlled send data mode."]
    #[inline(always)]
    pub fn spi_dma_tx_ena(&mut self) -> SPI_DMA_TX_ENA_W {
        SPI_DMA_TX_ENA_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer."]
    #[inline(always)]
    pub fn spi_rx_afifo_rst(&mut self) -> SPI_RX_AFIFO_RST_W {
        SPI_RX_AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer."]
    #[inline(always)]
    pub fn spi_buf_afifo_rst(&mut self) -> SPI_BUF_AFIFO_RST_W {
        SPI_BUF_AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer."]
    #[inline(always)]
    pub fn spi_dma_afifo_rst(&mut self) -> SPI_DMA_AFIFO_RST_W {
        SPI_DMA_AFIFO_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_conf](index.html) module"]
pub struct SPI_DMA_CONF_SPEC;
impl crate::RegisterSpec for SPI_DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_dma_conf::R](R) reader structure"]
impl crate::Readable for SPI_DMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_dma_conf::W](W) writer structure"]
impl crate::Writable for SPI_DMA_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SPI_DMA_CONF to value 0x03"]
impl crate::Resettable for SPI_DMA_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x03
    }
}
