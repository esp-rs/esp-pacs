#[doc = "Register `SLAVE` reader"]
pub struct R(crate::R<SLAVE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLAVE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLAVE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLAVE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLAVE` writer"]
pub struct W(crate::W<SLAVE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLAVE_SPEC>;
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
impl From<crate::W<SLAVE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLAVE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TRANS_DONE` reader - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
pub type TRANS_DONE_R = crate::BitReader;
#[doc = "Field `TRANS_DONE` writer - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
pub type TRANS_DONE_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `INT_RD_BUF_DONE_EN` reader - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_RD_BUF_DONE_EN_R = crate::BitReader;
#[doc = "Field `INT_RD_BUF_DONE_EN` writer - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_RD_BUF_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `INT_WR_BUF_DONE_EN` reader - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_WR_BUF_DONE_EN_R = crate::BitReader;
#[doc = "Field `INT_WR_BUF_DONE_EN` writer - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_WR_BUF_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `INT_RD_DMA_DONE_EN` reader - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_RD_DMA_DONE_EN_R = crate::BitReader;
#[doc = "Field `INT_RD_DMA_DONE_EN` writer - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_RD_DMA_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `INT_WR_DMA_DONE_EN` reader - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_WR_DMA_DONE_EN_R = crate::BitReader;
#[doc = "Field `INT_WR_DMA_DONE_EN` writer - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_WR_DMA_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `INT_TRANS_DONE_EN` reader - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_TRANS_DONE_EN_R = crate::BitReader;
#[doc = "Field `INT_TRANS_DONE_EN` writer - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_TRANS_DONE_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `INT_DMA_SEG_TRANS_EN` reader - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_DMA_SEG_TRANS_EN_R = crate::BitReader;
#[doc = "Field `INT_DMA_SEG_TRANS_EN` writer - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
pub type INT_DMA_SEG_TRANS_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `SEG_MAGIC_ERR_INT_EN` reader - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
pub type SEG_MAGIC_ERR_INT_EN_R = crate::BitReader;
#[doc = "Field `SEG_MAGIC_ERR_INT_EN` writer - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
pub type SEG_MAGIC_ERR_INT_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `TRANS_CNT` reader - The operations counter in both the master mode and the slave mode."]
pub type TRANS_CNT_R = crate::FieldReader;
#[doc = "Field `TRANS_DONE_AUTO_CLR_EN` reader - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
pub type TRANS_DONE_AUTO_CLR_EN_R = crate::BitReader;
#[doc = "Field `TRANS_DONE_AUTO_CLR_EN` writer - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
pub type TRANS_DONE_AUTO_CLR_EN_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `MODE` reader - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_R = crate::BitReader;
#[doc = "Field `MODE` writer - Set SPI work mode. 1: slave mode 0: master mode."]
pub type MODE_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
#[doc = "Field `SOFT_RESET` reader - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub type SOFT_RESET_R = crate::BitReader;
#[doc = "Field `SOFT_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
pub type SOFT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, SLAVE_SPEC, O>;
impl R {
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn trans_done(&self) -> TRANS_DONE_R {
        TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_rd_buf_done_en(&self) -> INT_RD_BUF_DONE_EN_R {
        INT_RD_BUF_DONE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_wr_buf_done_en(&self) -> INT_WR_BUF_DONE_EN_R {
        INT_WR_BUF_DONE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_rd_dma_done_en(&self) -> INT_RD_DMA_DONE_EN_R {
        INT_RD_DMA_DONE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_wr_dma_done_en(&self) -> INT_WR_DMA_DONE_EN_R {
        INT_WR_DMA_DONE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_trans_done_en(&self) -> INT_TRANS_DONE_EN_R {
        INT_TRANS_DONE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_dma_seg_trans_en(&self) -> INT_DMA_SEG_TRANS_EN_R {
        INT_DMA_SEG_TRANS_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
    #[inline(always)]
    pub fn seg_magic_err_int_en(&self) -> SEG_MAGIC_ERR_INT_EN_R {
        SEG_MAGIC_ERR_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 23:26 - The operations counter in both the master mode and the slave mode."]
    #[inline(always)]
    pub fn trans_cnt(&self) -> TRANS_CNT_R {
        TRANS_CNT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    #[doc = "Bit 29 - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn trans_done_auto_clr_en(&self) -> TRANS_DONE_AUTO_CLR_EN_R {
        TRANS_DONE_AUTO_CLR_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE")
            .field("trans_done", &format_args!("{}", self.trans_done().bit()))
            .field(
                "int_rd_buf_done_en",
                &format_args!("{}", self.int_rd_buf_done_en().bit()),
            )
            .field(
                "int_wr_buf_done_en",
                &format_args!("{}", self.int_wr_buf_done_en().bit()),
            )
            .field(
                "int_rd_dma_done_en",
                &format_args!("{}", self.int_rd_dma_done_en().bit()),
            )
            .field(
                "int_wr_dma_done_en",
                &format_args!("{}", self.int_wr_dma_done_en().bit()),
            )
            .field(
                "int_trans_done_en",
                &format_args!("{}", self.int_trans_done_en().bit()),
            )
            .field(
                "int_dma_seg_trans_en",
                &format_args!("{}", self.int_dma_seg_trans_en().bit()),
            )
            .field(
                "seg_magic_err_int_en",
                &format_args!("{}", self.seg_magic_err_int_en().bit()),
            )
            .field("trans_cnt", &format_args!("{}", self.trans_cnt().bits()))
            .field(
                "trans_done_auto_clr_en",
                &format_args!("{}", self.trans_done_auto_clr_en().bit()),
            )
            .field("mode", &format_args!("{}", self.mode().bit()))
            .field("soft_reset", &format_args!("{}", self.soft_reset().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLAVE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn trans_done(&mut self) -> TRANS_DONE_W<4> {
        TRANS_DONE_W::new(self)
    }
    #[doc = "Bit 5 - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn int_rd_buf_done_en(&mut self) -> INT_RD_BUF_DONE_EN_W<5> {
        INT_RD_BUF_DONE_EN_W::new(self)
    }
    #[doc = "Bit 6 - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn int_wr_buf_done_en(&mut self) -> INT_WR_BUF_DONE_EN_W<6> {
        INT_WR_BUF_DONE_EN_W::new(self)
    }
    #[doc = "Bit 7 - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn int_rd_dma_done_en(&mut self) -> INT_RD_DMA_DONE_EN_W<7> {
        INT_RD_DMA_DONE_EN_W::new(self)
    }
    #[doc = "Bit 8 - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn int_wr_dma_done_en(&mut self) -> INT_WR_DMA_DONE_EN_W<8> {
        INT_WR_DMA_DONE_EN_W::new(self)
    }
    #[doc = "Bit 9 - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn int_trans_done_en(&mut self) -> INT_TRANS_DONE_EN_W<9> {
        INT_TRANS_DONE_EN_W::new(self)
    }
    #[doc = "Bit 10 - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn int_dma_seg_trans_en(&mut self) -> INT_DMA_SEG_TRANS_EN_W<10> {
        INT_DMA_SEG_TRANS_EN_W::new(self)
    }
    #[doc = "Bit 11 - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn seg_magic_err_int_en(&mut self) -> SEG_MAGIC_ERR_INT_EN_W<11> {
        SEG_MAGIC_ERR_INT_EN_W::new(self)
    }
    #[doc = "Bit 29 - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn trans_done_auto_clr_en(&mut self) -> TRANS_DONE_AUTO_CLR_EN_W<29> {
        TRANS_DONE_AUTO_CLR_EN_W::new(self)
    }
    #[doc = "Bit 30 - Set SPI work mode. 1: slave mode 0: master mode."]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<30> {
        MODE_W::new(self)
    }
    #[doc = "Bit 31 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<31> {
        SOFT_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI slave control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slave](index.html) module"]
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slave::R](R) reader structure"]
impl crate::Readable for SLAVE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slave::W](W) writer structure"]
impl crate::Writable for SLAVE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLAVE to value 0x0200"]
impl crate::Resettable for SLAVE_SPEC {
    const RESET_VALUE: Self::Ux = 0x0200;
}
