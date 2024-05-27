///Register `SLAVE` reader
pub type R = crate::R<SLAVE_SPEC>;
///Register `SLAVE` writer
pub type W = crate::W<SLAVE_SPEC>;
///Field `TRANS_DONE` reader - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf.
pub type TRANS_DONE_R = crate::BitReader;
///Field `TRANS_DONE` writer - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf.
pub type TRANS_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_RD_BUF_DONE_EN` reader - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_RD_BUF_DONE_EN_R = crate::BitReader;
///Field `INT_RD_BUF_DONE_EN` writer - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_RD_BUF_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_WR_BUF_DONE_EN` reader - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_WR_BUF_DONE_EN_R = crate::BitReader;
///Field `INT_WR_BUF_DONE_EN` writer - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_WR_BUF_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_RD_DMA_DONE_EN` reader - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_RD_DMA_DONE_EN_R = crate::BitReader;
///Field `INT_RD_DMA_DONE_EN` writer - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_RD_DMA_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_WR_DMA_DONE_EN` reader - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_WR_DMA_DONE_EN_R = crate::BitReader;
///Field `INT_WR_DMA_DONE_EN` writer - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_WR_DMA_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_TRANS_DONE_EN` reader - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_TRANS_DONE_EN_R = crate::BitReader;
///Field `INT_TRANS_DONE_EN` writer - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_TRANS_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INT_DMA_SEG_TRANS_EN` reader - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_DMA_SEG_TRANS_EN_R = crate::BitReader;
///Field `INT_DMA_SEG_TRANS_EN` writer - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
pub type INT_DMA_SEG_TRANS_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SEG_MAGIC_ERR_INT_EN` reader - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state.
pub type SEG_MAGIC_ERR_INT_EN_R = crate::BitReader;
///Field `SEG_MAGIC_ERR_INT_EN` writer - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state.
pub type SEG_MAGIC_ERR_INT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TRANS_CNT` reader - The operations counter in both the master mode and the slave mode.
pub type TRANS_CNT_R = crate::FieldReader;
///Field `TRANS_DONE_AUTO_CLR_EN` reader - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state.
pub type TRANS_DONE_AUTO_CLR_EN_R = crate::BitReader;
///Field `TRANS_DONE_AUTO_CLR_EN` writer - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state.
pub type TRANS_DONE_AUTO_CLR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `MODE` reader - Set SPI work mode. 1: slave mode 0: master mode.
pub type MODE_R = crate::BitReader;
///Field `MODE` writer - Set SPI work mode. 1: slave mode 0: master mode.
pub type MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SOFT_RESET` reader - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
pub type SOFT_RESET_R = crate::BitReader;
///Field `SOFT_RESET` writer - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
pub type SOFT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf.
    #[inline(always)]
    pub fn trans_done(&self) -> TRANS_DONE_R {
        TRANS_DONE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn int_rd_buf_done_en(&self) -> INT_RD_BUF_DONE_EN_R {
        INT_RD_BUF_DONE_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn int_wr_buf_done_en(&self) -> INT_WR_BUF_DONE_EN_R {
        INT_WR_BUF_DONE_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn int_rd_dma_done_en(&self) -> INT_RD_DMA_DONE_EN_R {
        INT_RD_DMA_DONE_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn int_wr_dma_done_en(&self) -> INT_WR_DMA_DONE_EN_R {
        INT_WR_DMA_DONE_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn int_trans_done_en(&self) -> INT_TRANS_DONE_EN_R {
        INT_TRANS_DONE_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    pub fn int_dma_seg_trans_en(&self) -> INT_DMA_SEG_TRANS_EN_R {
        INT_DMA_SEG_TRANS_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state.
    #[inline(always)]
    pub fn seg_magic_err_int_en(&self) -> SEG_MAGIC_ERR_INT_EN_R {
        SEG_MAGIC_ERR_INT_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bits 23:26 - The operations counter in both the master mode and the slave mode.
    #[inline(always)]
    pub fn trans_cnt(&self) -> TRANS_CNT_R {
        TRANS_CNT_R::new(((self.bits >> 23) & 0x0f) as u8)
    }
    ///Bit 29 - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state.
    #[inline(always)]
    pub fn trans_done_auto_clr_en(&self) -> TRANS_DONE_AUTO_CLR_EN_R {
        TRANS_DONE_AUTO_CLR_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    ///Bit 30 - Set SPI work mode. 1: slave mode 0: master mode.
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 30) & 1) != 0)
    }
    ///Bit 31 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
    #[inline(always)]
    pub fn soft_reset(&self) -> SOFT_RESET_R {
        SOFT_RESET_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLAVE")
            .field("trans_done", &self.trans_done())
            .field("int_rd_buf_done_en", &self.int_rd_buf_done_en())
            .field("int_wr_buf_done_en", &self.int_wr_buf_done_en())
            .field("int_rd_dma_done_en", &self.int_rd_dma_done_en())
            .field("int_wr_dma_done_en", &self.int_wr_dma_done_en())
            .field("int_trans_done_en", &self.int_trans_done_en())
            .field("int_dma_seg_trans_en", &self.int_dma_seg_trans_en())
            .field("seg_magic_err_int_en", &self.seg_magic_err_int_en())
            .field("trans_cnt", &self.trans_cnt())
            .field("trans_done_auto_clr_en", &self.trans_done_auto_clr_en())
            .field("mode", &self.mode())
            .field("soft_reset", &self.soft_reset())
            .finish()
    }
}
impl W {
    ///Bit 4 - The interrupt raw bit for the completion of any operation in both the master mode and the slave mode. Can not be changed by CONF_buf.
    #[inline(always)]
    #[must_use]
    pub fn trans_done(&mut self) -> TRANS_DONE_W<SLAVE_SPEC> {
        TRANS_DONE_W::new(self, 4)
    }
    ///Bit 5 - SPI_SLV_RD_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn int_rd_buf_done_en(&mut self) -> INT_RD_BUF_DONE_EN_W<SLAVE_SPEC> {
        INT_RD_BUF_DONE_EN_W::new(self, 5)
    }
    ///Bit 6 - SPI_SLV_WR_BUF_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn int_wr_buf_done_en(&mut self) -> INT_WR_BUF_DONE_EN_W<SLAVE_SPEC> {
        INT_WR_BUF_DONE_EN_W::new(self, 6)
    }
    ///Bit 7 - SPI_SLV_RD_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn int_rd_dma_done_en(&mut self) -> INT_RD_DMA_DONE_EN_W<SLAVE_SPEC> {
        INT_RD_DMA_DONE_EN_W::new(self, 7)
    }
    ///Bit 8 - SPI_SLV_WR_DMA_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn int_wr_dma_done_en(&mut self) -> INT_WR_DMA_DONE_EN_W<SLAVE_SPEC> {
        INT_WR_DMA_DONE_EN_W::new(self, 8)
    }
    ///Bit 9 - SPI_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn int_trans_done_en(&mut self) -> INT_TRANS_DONE_EN_W<SLAVE_SPEC> {
        INT_TRANS_DONE_EN_W::new(self, 9)
    }
    ///Bit 10 - SPI_DMA_SEG_TRANS_DONE Interrupt enable. 1: enable 0: disable. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn int_dma_seg_trans_en(&mut self) -> INT_DMA_SEG_TRANS_EN_W<SLAVE_SPEC> {
        INT_DMA_SEG_TRANS_EN_W::new(self, 10)
    }
    ///Bit 11 - 1: Enable seg magic value error interrupt. 0: Others. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn seg_magic_err_int_en(&mut self) -> SEG_MAGIC_ERR_INT_EN_W<SLAVE_SPEC> {
        SEG_MAGIC_ERR_INT_EN_W::new(self, 11)
    }
    ///Bit 29 - SPI_TRANS_DONE auto clear enable, clear it 3 apb cycles after the pos edge of SPI_TRANS_DONE. 0:disable. 1: enable. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn trans_done_auto_clr_en(&mut self) -> TRANS_DONE_AUTO_CLR_EN_W<SLAVE_SPEC> {
        TRANS_DONE_AUTO_CLR_EN_W::new(self, 29)
    }
    ///Bit 30 - Set SPI work mode. 1: slave mode 0: master mode.
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<SLAVE_SPEC> {
        MODE_W::new(self, 30)
    }
    ///Bit 31 - Software reset enable, reset the spi clock line cs line and data lines. Can be configured in CONF state.
    #[inline(always)]
    #[must_use]
    pub fn soft_reset(&mut self) -> SOFT_RESET_W<SLAVE_SPEC> {
        SOFT_RESET_W::new(self, 31)
    }
}
/**SPI slave control register

You can [`read`](crate::generic::Reg::read) this register and get [`slave::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`slave::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct SLAVE_SPEC;
impl crate::RegisterSpec for SLAVE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`slave::R`](R) reader structure
impl crate::Readable for SLAVE_SPEC {}
///`write(|w| ..)` method takes [`slave::W`](W) writer structure
impl crate::Writable for SLAVE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets SLAVE to value 0x0200
impl crate::Resettable for SLAVE_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
