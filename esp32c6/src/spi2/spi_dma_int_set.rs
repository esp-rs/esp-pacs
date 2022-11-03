#[doc = "Register `SPI_DMA_INT_SET` writer"]
pub struct W(crate::W<SPI_DMA_INT_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_DMA_INT_SET_SPEC>;
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
impl From<crate::W<SPI_DMA_INT_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_DMA_INT_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_DMA_INFIFO_FULL_ERR_INT_SET` writer - The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
pub type SPI_DMA_INFIFO_FULL_ERR_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET` writer - The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
pub type SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_EX_QPI_INT_SET` writer - The software set bit for SPI slave Ex_QPI interrupt."]
pub type SPI_SLV_EX_QPI_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_EN_QPI_INT_SET` writer - The software set bit for SPI slave En_QPI interrupt."]
pub type SPI_SLV_EN_QPI_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_CMD7_INT_SET` writer - The software set bit for SPI slave CMD7 interrupt."]
pub type SPI_SLV_CMD7_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_CMD8_INT_SET` writer - The software set bit for SPI slave CMD8 interrupt."]
pub type SPI_SLV_CMD8_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_CMD9_INT_SET` writer - The software set bit for SPI slave CMD9 interrupt."]
pub type SPI_SLV_CMD9_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_CMDA_INT_SET` writer - The software set bit for SPI slave CMDA interrupt."]
pub type SPI_SLV_CMDA_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_RD_DMA_DONE_INT_SET` writer - The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
pub type SPI_SLV_RD_DMA_DONE_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_WR_DMA_DONE_INT_SET` writer - The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
pub type SPI_SLV_WR_DMA_DONE_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_RD_BUF_DONE_INT_SET` writer - The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
pub type SPI_SLV_RD_BUF_DONE_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_WR_BUF_DONE_INT_SET` writer - The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
pub type SPI_SLV_WR_BUF_DONE_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_TRANS_DONE_INT_SET` writer - The software set bit for SPI_TRANS_DONE_INT interrupt."]
pub type SPI_TRANS_DONE_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_DMA_SEG_TRANS_DONE_INT_SET` writer - The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
pub type SPI_DMA_SEG_TRANS_DONE_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SEG_MAGIC_ERR_INT_SET` writer - The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
pub type SPI_SEG_MAGIC_ERR_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_BUF_ADDR_ERR_INT_SET` writer - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
pub type SPI_SLV_BUF_ADDR_ERR_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_SLV_CMD_ERR_INT_SET` writer - The software set bit for SPI_SLV_CMD_ERR_INT interrupt."]
pub type SPI_SLV_CMD_ERR_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET` writer - The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
pub type SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET` writer - The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
pub type SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_APP2_INT_SET` writer - The software set bit for SPI_APP2_INT interrupt."]
pub type SPI_APP2_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
#[doc = "Field `SPI_APP1_INT_SET` writer - The software set bit for SPI_APP1_INT interrupt."]
pub type SPI_APP1_INT_SET_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SPI_DMA_INT_SET_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - The software set bit for SPI_DMA_INFIFO_FULL_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_infifo_full_err_int_set(&mut self) -> SPI_DMA_INFIFO_FULL_ERR_INT_SET_W<0> {
        SPI_DMA_INFIFO_FULL_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 1 - The software set bit for SPI_DMA_OUTFIFO_EMPTY_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_outfifo_empty_err_int_set(&mut self) -> SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_W<1> {
        SPI_DMA_OUTFIFO_EMPTY_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 2 - The software set bit for SPI slave Ex_QPI interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_ex_qpi_int_set(&mut self) -> SPI_SLV_EX_QPI_INT_SET_W<2> {
        SPI_SLV_EX_QPI_INT_SET_W::new(self)
    }
    #[doc = "Bit 3 - The software set bit for SPI slave En_QPI interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_en_qpi_int_set(&mut self) -> SPI_SLV_EN_QPI_INT_SET_W<3> {
        SPI_SLV_EN_QPI_INT_SET_W::new(self)
    }
    #[doc = "Bit 4 - The software set bit for SPI slave CMD7 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd7_int_set(&mut self) -> SPI_SLV_CMD7_INT_SET_W<4> {
        SPI_SLV_CMD7_INT_SET_W::new(self)
    }
    #[doc = "Bit 5 - The software set bit for SPI slave CMD8 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd8_int_set(&mut self) -> SPI_SLV_CMD8_INT_SET_W<5> {
        SPI_SLV_CMD8_INT_SET_W::new(self)
    }
    #[doc = "Bit 6 - The software set bit for SPI slave CMD9 interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd9_int_set(&mut self) -> SPI_SLV_CMD9_INT_SET_W<6> {
        SPI_SLV_CMD9_INT_SET_W::new(self)
    }
    #[doc = "Bit 7 - The software set bit for SPI slave CMDA interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmda_int_set(&mut self) -> SPI_SLV_CMDA_INT_SET_W<7> {
        SPI_SLV_CMDA_INT_SET_W::new(self)
    }
    #[doc = "Bit 8 - The software set bit for SPI_SLV_RD_DMA_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_rd_dma_done_int_set(&mut self) -> SPI_SLV_RD_DMA_DONE_INT_SET_W<8> {
        SPI_SLV_RD_DMA_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 9 - The software set bit for SPI_SLV_WR_DMA_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_wr_dma_done_int_set(&mut self) -> SPI_SLV_WR_DMA_DONE_INT_SET_W<9> {
        SPI_SLV_WR_DMA_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 10 - The software set bit for SPI_SLV_RD_BUF_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_rd_buf_done_int_set(&mut self) -> SPI_SLV_RD_BUF_DONE_INT_SET_W<10> {
        SPI_SLV_RD_BUF_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 11 - The software set bit for SPI_SLV_WR_BUF_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_wr_buf_done_int_set(&mut self) -> SPI_SLV_WR_BUF_DONE_INT_SET_W<11> {
        SPI_SLV_WR_BUF_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 12 - The software set bit for SPI_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_trans_done_int_set(&mut self) -> SPI_TRANS_DONE_INT_SET_W<12> {
        SPI_TRANS_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 13 - The software set bit for SPI_DMA_SEG_TRANS_DONE_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_dma_seg_trans_done_int_set(&mut self) -> SPI_DMA_SEG_TRANS_DONE_INT_SET_W<13> {
        SPI_DMA_SEG_TRANS_DONE_INT_SET_W::new(self)
    }
    #[doc = "Bit 14 - The software set bit for SPI_SEG_MAGIC_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_seg_magic_err_int_set(&mut self) -> SPI_SEG_MAGIC_ERR_INT_SET_W<14> {
        SPI_SEG_MAGIC_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 15 - The software set bit for SPI_SLV_BUF_ADDR_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_buf_addr_err_int_set(&mut self) -> SPI_SLV_BUF_ADDR_ERR_INT_SET_W<15> {
        SPI_SLV_BUF_ADDR_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 16 - The software set bit for SPI_SLV_CMD_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_slv_cmd_err_int_set(&mut self) -> SPI_SLV_CMD_ERR_INT_SET_W<16> {
        SPI_SLV_CMD_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 17 - The software set bit for SPI_MST_RX_AFIFO_WFULL_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mst_rx_afifo_wfull_err_int_set(
        &mut self,
    ) -> SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_W<17> {
        SPI_MST_RX_AFIFO_WFULL_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 18 - The software set bit for SPI_MST_TX_AFIFO_REMPTY_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mst_tx_afifo_rempty_err_int_set(
        &mut self,
    ) -> SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_W<18> {
        SPI_MST_TX_AFIFO_REMPTY_ERR_INT_SET_W::new(self)
    }
    #[doc = "Bit 19 - The software set bit for SPI_APP2_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_app2_int_set(&mut self) -> SPI_APP2_INT_SET_W<19> {
        SPI_APP2_INT_SET_W::new(self)
    }
    #[doc = "Bit 20 - The software set bit for SPI_APP1_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn spi_app1_int_set(&mut self) -> SPI_APP1_INT_SET_W<20> {
        SPI_APP1_INT_SET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI interrupt software set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_dma_int_set](index.html) module"]
pub struct SPI_DMA_INT_SET_SPEC;
impl crate::RegisterSpec for SPI_DMA_INT_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [spi_dma_int_set::W](W) writer structure"]
impl crate::Writable for SPI_DMA_INT_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_DMA_INT_SET to value 0"]
impl crate::Resettable for SPI_DMA_INT_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
