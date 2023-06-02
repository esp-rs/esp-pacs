#[doc = "Register `DMA_CONF` reader"]
pub struct R(crate::R<DMA_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DMA_CONF` writer"]
pub struct W(crate::W<DMA_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DMA_CONF_SPEC>;
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
impl From<crate::W<DMA_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DMA_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMA_OUTFIFO_EMPTY` reader - Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data."]
pub type DMA_OUTFIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `DMA_INFIFO_FULL` reader - Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data."]
pub type DMA_INFIFO_FULL_R = crate::BitReader;
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` reader - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub type DMA_SLV_SEG_TRANS_EN_R = crate::BitReader;
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` writer - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub type DMA_SLV_SEG_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
pub type SLV_RX_SEG_TRANS_CLR_EN_R = crate::BitReader;
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
pub type SLV_RX_SEG_TRANS_CLR_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
pub type SLV_TX_SEG_TRANS_CLR_EN_R = crate::BitReader;
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
pub type SLV_TX_SEG_TRANS_CLR_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `RX_EOF_EN` reader - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
pub type RX_EOF_EN_R = crate::BitReader;
#[doc = "Field `RX_EOF_EN` writer - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
pub type RX_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_RX_ENA` reader - Set this bit to enable SPI DMA controlled receive data mode."]
pub type DMA_RX_ENA_R = crate::BitReader;
#[doc = "Field `DMA_RX_ENA` writer - Set this bit to enable SPI DMA controlled receive data mode."]
pub type DMA_RX_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_TX_ENA` reader - Set this bit to enable SPI DMA controlled send data mode."]
pub type DMA_TX_ENA_R = crate::BitReader;
#[doc = "Field `DMA_TX_ENA` writer - Set this bit to enable SPI DMA controlled send data mode."]
pub type DMA_TX_ENA_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `RX_AFIFO_RST` writer - Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer."]
pub type RX_AFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `BUF_AFIFO_RST` writer - Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer."]
pub type BUF_AFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_AFIFO_RST` writer - Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer."]
pub type DMA_AFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Records the status of DMA TX FIFO. 1: DMA TX FIFO is not ready for sending data. 0: DMA TX FIFO is ready for sending data."]
    #[inline(always)]
    pub fn dma_outfifo_empty(&self) -> DMA_OUTFIFO_EMPTY_R {
        DMA_OUTFIFO_EMPTY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Records the status of DMA RX FIFO. 1: DMA RX FIFO is not ready for receiving data. 0: DMA RX FIFO is ready for receiving data."]
    #[inline(always)]
    pub fn dma_infifo_full(&self) -> DMA_INFIFO_FULL_R {
        DMA_INFIFO_FULL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&self) -> DMA_SLV_SEG_TRANS_EN_R {
        DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&self) -> SLV_RX_SEG_TRANS_CLR_EN_R {
        SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&self) -> SLV_TX_SEG_TRANS_CLR_EN_R {
        SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
    #[inline(always)]
    pub fn rx_eof_en(&self) -> RX_EOF_EN_R {
        RX_EOF_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable SPI DMA controlled receive data mode."]
    #[inline(always)]
    pub fn dma_rx_ena(&self) -> DMA_RX_ENA_R {
        DMA_RX_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable SPI DMA controlled send data mode."]
    #[inline(always)]
    pub fn dma_tx_ena(&self) -> DMA_TX_ENA_R {
        DMA_TX_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CONF")
            .field(
                "dma_outfifo_empty",
                &format_args!("{}", self.dma_outfifo_empty().bit()),
            )
            .field(
                "dma_infifo_full",
                &format_args!("{}", self.dma_infifo_full().bit()),
            )
            .field(
                "dma_slv_seg_trans_en",
                &format_args!("{}", self.dma_slv_seg_trans_en().bit()),
            )
            .field(
                "slv_rx_seg_trans_clr_en",
                &format_args!("{}", self.slv_rx_seg_trans_clr_en().bit()),
            )
            .field(
                "slv_tx_seg_trans_clr_en",
                &format_args!("{}", self.slv_tx_seg_trans_clr_en().bit()),
            )
            .field("rx_eof_en", &format_args!("{}", self.rx_eof_en().bit()))
            .field("dma_rx_ena", &format_args!("{}", self.dma_rx_ena().bit()))
            .field("dma_tx_ena", &format_args!("{}", self.dma_tx_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_slv_seg_trans_en(&mut self) -> DMA_SLV_SEG_TRANS_EN_W<18> {
        DMA_SLV_SEG_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave cmd 5. 0: spi_dma_infifo_full_vld is cleared by spi_trans_done."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rx_seg_trans_clr_en(&mut self) -> SLV_RX_SEG_TRANS_CLR_EN_W<19> {
        SLV_RX_SEG_TRANS_CLR_EN_W::new(self)
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave cmd 6. 0: spi_dma_outfifo_empty_vld is cleared by spi_trans_done."]
    #[inline(always)]
    #[must_use]
    pub fn slv_tx_seg_trans_clr_en(&mut self) -> SLV_TX_SEG_TRANS_CLR_EN_W<20> {
        SLV_TX_SEG_TRANS_CLR_EN_W::new(self)
    }
    #[doc = "Bit 21 - 1: spi_dma_inlink_eof is set when the number of dma pushed data bytes is equal to the value of spi_slv/mst_dma_rd_bytelen\\[19:0\\] in spi dma transition. 0: spi_dma_inlink_eof is set by spi_trans_done in non-seg-trans or spi_dma_seg_trans_done in seg-trans."]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof_en(&mut self) -> RX_EOF_EN_W<21> {
        RX_EOF_EN_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to enable SPI DMA controlled receive data mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_ena(&mut self) -> DMA_RX_ENA_W<27> {
        DMA_RX_ENA_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to enable SPI DMA controlled send data mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_ena(&mut self) -> DMA_TX_ENA_W<28> {
        DMA_TX_ENA_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to reset RX AFIFO, which is used to receive data in SPI master and slave mode transfer."]
    #[inline(always)]
    #[must_use]
    pub fn rx_afifo_rst(&mut self) -> RX_AFIFO_RST_W<29> {
        RX_AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to reset BUF TX AFIFO, which is used send data out in SPI slave CPU controlled mode transfer and master mode transfer."]
    #[inline(always)]
    #[must_use]
    pub fn buf_afifo_rst(&mut self) -> BUF_AFIFO_RST_W<30> {
        BUF_AFIFO_RST_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to reset DMA TX AFIFO, which is used to send data out in SPI slave DMA controlled mode transfer."]
    #[inline(always)]
    #[must_use]
    pub fn dma_afifo_rst(&mut self) -> DMA_AFIFO_RST_W<31> {
        DMA_AFIFO_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI DMA control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_conf](index.html) module"]
pub struct DMA_CONF_SPEC;
impl crate::RegisterSpec for DMA_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_conf::R](R) reader structure"]
impl crate::Readable for DMA_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dma_conf::W](W) writer structure"]
impl crate::Writable for DMA_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_CONF to value 0x03"]
impl crate::Resettable for DMA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
