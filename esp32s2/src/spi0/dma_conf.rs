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
#[doc = "Field `IN_RST` reader - The bit is used to reset in dma fsm and in data fifo pointer."]
pub type IN_RST_R = crate::BitReader;
#[doc = "Field `IN_RST` writer - The bit is used to reset in dma fsm and in data fifo pointer."]
pub type IN_RST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `OUT_RST` reader - The bit is used to reset out dma fsm and out data fifo pointer."]
pub type OUT_RST_R = crate::BitReader;
#[doc = "Field `OUT_RST` writer - The bit is used to reset out dma fsm and out data fifo pointer."]
pub type OUT_RST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `AHBM_FIFO_RST` reader - Reset spi dma ahb master fifo pointer."]
pub type AHBM_FIFO_RST_R = crate::BitReader;
#[doc = "Field `AHBM_FIFO_RST` writer - Reset spi dma ahb master fifo pointer."]
pub type AHBM_FIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `AHBM_RST` reader - Reset spi dma ahb master."]
pub type AHBM_RST_R = crate::BitReader;
#[doc = "Field `AHBM_RST` writer - Reset spi dma ahb master."]
pub type AHBM_RST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `IN_LOOP_TEST` reader - Set bit to test in link."]
pub type IN_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `IN_LOOP_TEST` writer - Set bit to test in link."]
pub type IN_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `OUT_LOOP_TEST` reader - Set bit to test out link."]
pub type OUT_LOOP_TEST_R = crate::BitReader;
#[doc = "Field `OUT_LOOP_TEST` writer - Set bit to test out link."]
pub type OUT_LOOP_TEST_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `OUT_AUTO_WRBACK` reader - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
pub type OUT_AUTO_WRBACK_R = crate::BitReader;
#[doc = "Field `OUT_AUTO_WRBACK` writer - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
pub type OUT_AUTO_WRBACK_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `OUT_EOF_MODE` reader - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
pub type OUT_EOF_MODE_R = crate::BitReader;
#[doc = "Field `OUT_EOF_MODE` writer - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
pub type OUT_EOF_MODE_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `OUTDSCR_BURST_EN` reader - read descriptor use burst mode when read data for memory."]
pub type OUTDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUTDSCR_BURST_EN` writer - read descriptor use burst mode when read data for memory."]
pub type OUTDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `INDSCR_BURST_EN` reader - read descriptor use burst mode when write data to memory."]
pub type INDSCR_BURST_EN_R = crate::BitReader;
#[doc = "Field `INDSCR_BURST_EN` writer - read descriptor use burst mode when write data to memory."]
pub type INDSCR_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `OUT_DATA_BURST_EN` reader - spi dma read data from memory in burst mode."]
pub type OUT_DATA_BURST_EN_R = crate::BitReader;
#[doc = "Field `OUT_DATA_BURST_EN` writer - spi dma read data from memory in burst mode."]
pub type OUT_DATA_BURST_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `MEM_TRANS_EN` reader - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
pub type MEM_TRANS_EN_R = crate::BitReader;
#[doc = "Field `MEM_TRANS_EN` writer - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
pub type MEM_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_RX_STOP` reader - spi dma read data stop when in continue tx/rx mode."]
pub type DMA_RX_STOP_R = crate::BitReader;
#[doc = "Field `DMA_RX_STOP` writer - spi dma read data stop when in continue tx/rx mode."]
pub type DMA_RX_STOP_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_TX_STOP` reader - spi dma write data stop when in continue tx/rx mode."]
pub type DMA_TX_STOP_R = crate::BitReader;
#[doc = "Field `DMA_TX_STOP` writer - spi dma write data stop when in continue tx/rx mode."]
pub type DMA_TX_STOP_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_CONTINUE` reader - spi dma continue tx/rx data."]
pub type DMA_CONTINUE_R = crate::BitReader;
#[doc = "Field `DMA_CONTINUE` writer - spi dma continue tx/rx data."]
pub type DMA_CONTINUE_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `SLV_LAST_SEG_POP_CLR` reader - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
pub type SLV_LAST_SEG_POP_CLR_R = crate::BitReader;
#[doc = "Field `SLV_LAST_SEG_POP_CLR` writer - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
pub type SLV_LAST_SEG_POP_CLR_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` reader - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub type DMA_SLV_SEG_TRANS_EN_R = crate::BitReader;
#[doc = "Field `DMA_SLV_SEG_TRANS_EN` writer - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
pub type DMA_SLV_SEG_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
pub type SLV_RX_SEG_TRANS_CLR_EN_R = crate::BitReader;
#[doc = "Field `SLV_RX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
pub type SLV_RX_SEG_TRANS_CLR_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` reader - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
pub type SLV_TX_SEG_TRANS_CLR_EN_R = crate::BitReader;
#[doc = "Field `SLV_TX_SEG_TRANS_CLR_EN` writer - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
pub type SLV_TX_SEG_TRANS_CLR_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `RX_EOF_EN` reader - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\] in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
pub type RX_EOF_EN_R = crate::BitReader;
#[doc = "Field `RX_EOF_EN` writer - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\] in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
pub type RX_EOF_EN_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_INFIFO_FULL_CLR` reader - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
pub type DMA_INFIFO_FULL_CLR_R = crate::BitReader;
#[doc = "Field `DMA_INFIFO_FULL_CLR` writer - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
pub type DMA_INFIFO_FULL_CLR_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `DMA_OUTFIFO_EMPTY_CLR` reader - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
pub type DMA_OUTFIFO_EMPTY_CLR_R = crate::BitReader;
#[doc = "Field `DMA_OUTFIFO_EMPTY_CLR` writer - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
pub type DMA_OUTFIFO_EMPTY_CLR_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
#[doc = "Field `EXT_MEM_BK_SIZE` reader - Select the external memory block size."]
pub type EXT_MEM_BK_SIZE_R = crate::FieldReader;
#[doc = "Field `EXT_MEM_BK_SIZE` writer - Select the external memory block size."]
pub type EXT_MEM_BK_SIZE_W<'a, const O: u8> = crate::FieldWriter<'a, DMA_CONF_SPEC, 2, O>;
#[doc = "Field `DMA_SEG_TRANS_CLR` reader - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
pub type DMA_SEG_TRANS_CLR_R = crate::BitReader;
#[doc = "Field `DMA_SEG_TRANS_CLR` writer - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
pub type DMA_SEG_TRANS_CLR_W<'a, const O: u8> = crate::BitWriter<'a, DMA_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    pub fn in_rst(&self) -> IN_RST_R {
        IN_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    pub fn out_rst(&self) -> OUT_RST_R {
        OUT_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    pub fn ahbm_fifo_rst(&self) -> AHBM_FIFO_RST_R {
        AHBM_FIFO_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Reset spi dma ahb master."]
    #[inline(always)]
    pub fn ahbm_rst(&self) -> AHBM_RST_R {
        AHBM_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    pub fn in_loop_test(&self) -> IN_LOOP_TEST_R {
        IN_LOOP_TEST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    pub fn out_loop_test(&self) -> OUT_LOOP_TEST_R {
        OUT_LOOP_TEST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
    #[inline(always)]
    pub fn out_auto_wrback(&self) -> OUT_AUTO_WRBACK_R {
        OUT_AUTO_WRBACK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    pub fn out_eof_mode(&self) -> OUT_EOF_MODE_R {
        OUT_EOF_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    pub fn outdscr_burst_en(&self) -> OUTDSCR_BURST_EN_R {
        OUTDSCR_BURST_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    pub fn indscr_burst_en(&self) -> INDSCR_BURST_EN_R {
        INDSCR_BURST_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    pub fn out_data_burst_en(&self) -> OUT_DATA_BURST_EN_R {
        OUT_DATA_BURST_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
    #[inline(always)]
    pub fn mem_trans_en(&self) -> MEM_TRANS_EN_R {
        MEM_TRANS_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_rx_stop(&self) -> DMA_RX_STOP_R {
        DMA_RX_STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    pub fn dma_tx_stop(&self) -> DMA_TX_STOP_R {
        DMA_TX_STOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    pub fn dma_continue(&self) -> DMA_CONTINUE_R {
        DMA_CONTINUE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
    #[inline(always)]
    pub fn slv_last_seg_pop_clr(&self) -> SLV_LAST_SEG_POP_CLR_R {
        SLV_LAST_SEG_POP_CLR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    pub fn dma_slv_seg_trans_en(&self) -> DMA_SLV_SEG_TRANS_EN_R {
        DMA_SLV_SEG_TRANS_EN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    pub fn slv_rx_seg_trans_clr_en(&self) -> SLV_RX_SEG_TRANS_CLR_EN_R {
        SLV_RX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    pub fn slv_tx_seg_trans_clr_en(&self) -> SLV_TX_SEG_TRANS_CLR_EN_R {
        SLV_TX_SEG_TRANS_CLR_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\] in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
    #[inline(always)]
    pub fn rx_eof_en(&self) -> RX_EOF_EN_R {
        RX_EOF_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
    #[inline(always)]
    pub fn dma_infifo_full_clr(&self) -> DMA_INFIFO_FULL_CLR_R {
        DMA_INFIFO_FULL_CLR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
    #[inline(always)]
    pub fn dma_outfifo_empty_clr(&self) -> DMA_OUTFIFO_EMPTY_CLR_R {
        DMA_OUTFIFO_EMPTY_CLR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Select the external memory block size."]
    #[inline(always)]
    pub fn ext_mem_bk_size(&self) -> EXT_MEM_BK_SIZE_R {
        EXT_MEM_BK_SIZE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bit 28 - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
    #[inline(always)]
    pub fn dma_seg_trans_clr(&self) -> DMA_SEG_TRANS_CLR_R {
        DMA_SEG_TRANS_CLR_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_CONF")
            .field("in_rst", &format_args!("{}", self.in_rst().bit()))
            .field("out_rst", &format_args!("{}", self.out_rst().bit()))
            .field(
                "ahbm_fifo_rst",
                &format_args!("{}", self.ahbm_fifo_rst().bit()),
            )
            .field("ahbm_rst", &format_args!("{}", self.ahbm_rst().bit()))
            .field(
                "in_loop_test",
                &format_args!("{}", self.in_loop_test().bit()),
            )
            .field(
                "out_loop_test",
                &format_args!("{}", self.out_loop_test().bit()),
            )
            .field(
                "out_auto_wrback",
                &format_args!("{}", self.out_auto_wrback().bit()),
            )
            .field(
                "out_eof_mode",
                &format_args!("{}", self.out_eof_mode().bit()),
            )
            .field(
                "outdscr_burst_en",
                &format_args!("{}", self.outdscr_burst_en().bit()),
            )
            .field(
                "indscr_burst_en",
                &format_args!("{}", self.indscr_burst_en().bit()),
            )
            .field(
                "out_data_burst_en",
                &format_args!("{}", self.out_data_burst_en().bit()),
            )
            .field(
                "mem_trans_en",
                &format_args!("{}", self.mem_trans_en().bit()),
            )
            .field("dma_rx_stop", &format_args!("{}", self.dma_rx_stop().bit()))
            .field("dma_tx_stop", &format_args!("{}", self.dma_tx_stop().bit()))
            .field(
                "dma_continue",
                &format_args!("{}", self.dma_continue().bit()),
            )
            .field(
                "slv_last_seg_pop_clr",
                &format_args!("{}", self.slv_last_seg_pop_clr().bit()),
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
            .field(
                "dma_infifo_full_clr",
                &format_args!("{}", self.dma_infifo_full_clr().bit()),
            )
            .field(
                "dma_outfifo_empty_clr",
                &format_args!("{}", self.dma_outfifo_empty_clr().bit()),
            )
            .field(
                "ext_mem_bk_size",
                &format_args!("{}", self.ext_mem_bk_size().bits()),
            )
            .field(
                "dma_seg_trans_clr",
                &format_args!("{}", self.dma_seg_trans_clr().bit()),
            )
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
    #[doc = "Bit 2 - The bit is used to reset in dma fsm and in data fifo pointer."]
    #[inline(always)]
    #[must_use]
    pub fn in_rst(&mut self) -> IN_RST_W<2> {
        IN_RST_W::new(self)
    }
    #[doc = "Bit 3 - The bit is used to reset out dma fsm and out data fifo pointer."]
    #[inline(always)]
    #[must_use]
    pub fn out_rst(&mut self) -> OUT_RST_W<3> {
        OUT_RST_W::new(self)
    }
    #[doc = "Bit 4 - Reset spi dma ahb master fifo pointer."]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_fifo_rst(&mut self) -> AHBM_FIFO_RST_W<4> {
        AHBM_FIFO_RST_W::new(self)
    }
    #[doc = "Bit 5 - Reset spi dma ahb master."]
    #[inline(always)]
    #[must_use]
    pub fn ahbm_rst(&mut self) -> AHBM_RST_W<5> {
        AHBM_RST_W::new(self)
    }
    #[doc = "Bit 6 - Set bit to test in link."]
    #[inline(always)]
    #[must_use]
    pub fn in_loop_test(&mut self) -> IN_LOOP_TEST_W<6> {
        IN_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 7 - Set bit to test out link."]
    #[inline(always)]
    #[must_use]
    pub fn out_loop_test(&mut self) -> OUT_LOOP_TEST_W<7> {
        OUT_LOOP_TEST_W::new(self)
    }
    #[doc = "Bit 8 - when the bit is set, DMA continue to use the next inlink node when the length of inlink is 0."]
    #[inline(always)]
    #[must_use]
    pub fn out_auto_wrback(&mut self) -> OUT_AUTO_WRBACK_W<8> {
        OUT_AUTO_WRBACK_W::new(self)
    }
    #[doc = "Bit 9 - out eof flag generation mode . 1: when dma pop all data from fifo 0:when ahb push all data to fifo."]
    #[inline(always)]
    #[must_use]
    pub fn out_eof_mode(&mut self) -> OUT_EOF_MODE_W<9> {
        OUT_EOF_MODE_W::new(self)
    }
    #[doc = "Bit 10 - read descriptor use burst mode when read data for memory."]
    #[inline(always)]
    #[must_use]
    pub fn outdscr_burst_en(&mut self) -> OUTDSCR_BURST_EN_W<10> {
        OUTDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 11 - read descriptor use burst mode when write data to memory."]
    #[inline(always)]
    #[must_use]
    pub fn indscr_burst_en(&mut self) -> INDSCR_BURST_EN_W<11> {
        INDSCR_BURST_EN_W::new(self)
    }
    #[doc = "Bit 12 - spi dma read data from memory in burst mode."]
    #[inline(always)]
    #[must_use]
    pub fn out_data_burst_en(&mut self) -> OUT_DATA_BURST_EN_W<12> {
        OUT_DATA_BURST_EN_W::new(self)
    }
    #[doc = "Bit 13 - 1: Internal memory data transfer enable bit. Send SPI DMA RX buffer data to SPI DMA TX buffer. 0: Disable this function."]
    #[inline(always)]
    #[must_use]
    pub fn mem_trans_en(&mut self) -> MEM_TRANS_EN_W<13> {
        MEM_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 14 - spi dma read data stop when in continue tx/rx mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma_rx_stop(&mut self) -> DMA_RX_STOP_W<14> {
        DMA_RX_STOP_W::new(self)
    }
    #[doc = "Bit 15 - spi dma write data stop when in continue tx/rx mode."]
    #[inline(always)]
    #[must_use]
    pub fn dma_tx_stop(&mut self) -> DMA_TX_STOP_W<15> {
        DMA_TX_STOP_W::new(self)
    }
    #[doc = "Bit 16 - spi dma continue tx/rx data."]
    #[inline(always)]
    #[must_use]
    pub fn dma_continue(&mut self) -> DMA_CONTINUE_W<16> {
        DMA_CONTINUE_W::new(self)
    }
    #[doc = "Bit 17 - 1: Clear spi_slv_seg_frt_pop_mask. 0 : others"]
    #[inline(always)]
    #[must_use]
    pub fn slv_last_seg_pop_clr(&mut self) -> SLV_LAST_SEG_POP_CLR_W<17> {
        SLV_LAST_SEG_POP_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Enable dma segment transfer in spi dma half slave mode. 1: enable. 0: disable."]
    #[inline(always)]
    #[must_use]
    pub fn dma_slv_seg_trans_en(&mut self) -> DMA_SLV_SEG_TRANS_EN_W<18> {
        DMA_SLV_SEG_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 19 - 1: spi_dma_infifo_full_vld is cleared by spi slave CMD5. 0: spi_dma_infifo_full_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn slv_rx_seg_trans_clr_en(&mut self) -> SLV_RX_SEG_TRANS_CLR_EN_W<19> {
        SLV_RX_SEG_TRANS_CLR_EN_W::new(self)
    }
    #[doc = "Bit 20 - 1: spi_dma_outfifo_empty_vld is cleared by spi slave CMD6. 0: spi_dma_outfifo_empty_vld is cleared by SPI_TRANS_DONE."]
    #[inline(always)]
    #[must_use]
    pub fn slv_tx_seg_trans_clr_en(&mut self) -> SLV_TX_SEG_TRANS_CLR_EN_W<20> {
        SLV_TX_SEG_TRANS_CLR_EN_W::new(self)
    }
    #[doc = "Bit 21 - 1: SPI_IN_SUC_EOF_INT_RAW is set when the number of dma pushed data bytes is equal to the value of SPI_SLV_DMA_RD_BYTELEN\\[19:0\\]/ SPI_MST_DMA_RD_BYTELEN\\[19:0\\] in spi dma transition. 0: SPI_IN_SUC_EOF_INT_RAW is set by SPI_TRANS_DONE in non-seg-trans or SPI_DMA_SEG_TRANS_DONE in seg-trans."]
    #[inline(always)]
    #[must_use]
    pub fn rx_eof_en(&mut self) -> RX_EOF_EN_W<21> {
        RX_EOF_EN_W::new(self)
    }
    #[doc = "Bit 22 - 1:Clear spi_dma_infifo_full_vld. 0: Do not control it."]
    #[inline(always)]
    #[must_use]
    pub fn dma_infifo_full_clr(&mut self) -> DMA_INFIFO_FULL_CLR_W<22> {
        DMA_INFIFO_FULL_CLR_W::new(self)
    }
    #[doc = "Bit 23 - 1:Clear spi_dma_outfifo_empty_vld. 0: Do not control it."]
    #[inline(always)]
    #[must_use]
    pub fn dma_outfifo_empty_clr(&mut self) -> DMA_OUTFIFO_EMPTY_CLR_W<23> {
        DMA_OUTFIFO_EMPTY_CLR_W::new(self)
    }
    #[doc = "Bits 26:27 - Select the external memory block size."]
    #[inline(always)]
    #[must_use]
    pub fn ext_mem_bk_size(&mut self) -> EXT_MEM_BK_SIZE_W<26> {
        EXT_MEM_BK_SIZE_W::new(self)
    }
    #[doc = "Bit 28 - 1: End slave seg-trans, which acts as 0x05 command. 2 or more end seg-trans signals will induce error in DMA RX. 0: others. Will be cleared in 1 APB CLK cycles by hardware.."]
    #[inline(always)]
    #[must_use]
    pub fn dma_seg_trans_clr(&mut self) -> DMA_SEG_TRANS_CLR_W<28> {
        DMA_SEG_TRANS_CLR_W::new(self)
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
#[doc = "`reset()` method sets DMA_CONF to value 0x0200"]
impl crate::Resettable for DMA_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
