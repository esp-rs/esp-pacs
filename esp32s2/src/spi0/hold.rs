#[doc = "Register `HOLD` reader"]
pub type R = crate::R<HOLD_SPEC>;
#[doc = "Register `HOLD` writer"]
pub type W = crate::W<HOLD_SPEC>;
#[doc = "Field `INT_HOLD_ENA` reader - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
pub type INT_HOLD_ENA_R = crate::FieldReader;
#[doc = "Field `INT_HOLD_ENA` writer - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
pub type INT_HOLD_ENA_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `VAL` reader - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
pub type VAL_R = crate::BitReader;
#[doc = "Field `VAL` writer - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
pub type VAL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_EN` reader - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
pub type OUT_EN_R = crate::BitReader;
#[doc = "Field `OUT_EN` writer - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
pub type OUT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_TIME` reader - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
pub type OUT_TIME_R = crate::FieldReader;
#[doc = "Field `OUT_TIME` writer - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
pub type OUT_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DMA_SEG_TRANS_DONE` reader - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
pub type DMA_SEG_TRANS_DONE_R = crate::BitReader;
#[doc = "Field `DMA_SEG_TRANS_DONE` writer - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
pub type DMA_SEG_TRANS_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
    #[inline(always)]
    pub fn int_hold_ena(&self) -> INT_HOLD_ENA_R {
        INT_HOLD_ENA_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
    #[inline(always)]
    pub fn val(&self) -> VAL_R {
        VAL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_en(&self) -> OUT_EN_R {
        OUT_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
    #[inline(always)]
    pub fn out_time(&self) -> OUT_TIME_R {
        OUT_TIME_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
    #[inline(always)]
    pub fn dma_seg_trans_done(&self) -> DMA_SEG_TRANS_DONE_R {
        DMA_SEG_TRANS_DONE_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HOLD")
            .field("int_hold_ena", &self.int_hold_ena())
            .field("val", &self.val())
            .field("out_en", &self.out_en())
            .field("out_time", &self.out_time())
            .field("dma_seg_trans_done", &self.dma_seg_trans_done())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - This register is for two SPI masters to share the same cs clock and data signals. The bits of one SPI are set, if the other SPI is busy, the SPI will be hold. 1(3): hold at idle phase 2: hold at prepare phase. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn int_hold_ena(&mut self) -> INT_HOLD_ENA_W<HOLD_SPEC> {
        INT_HOLD_ENA_W::new(self, 0)
    }
    #[doc = "Bit 2 - spi hold output value, which should be used with SPI_HOLD_OUT_EN. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn val(&mut self) -> VAL_W<HOLD_SPEC> {
        VAL_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable set spi output hold value to spi_hold_reg. It can be used to hold spi state machine with SPI_EXT_HOLD_EN and other usr hold signals. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn out_en(&mut self) -> OUT_EN_W<HOLD_SPEC> {
        OUT_EN_W::new(self, 3)
    }
    #[doc = "Bits 4:6 - set the hold cycles of output spi_hold signal when SPI_HOLD_OUT_EN is enable. Can be configured in CONF state."]
    #[inline(always)]
    #[must_use]
    pub fn out_time(&mut self) -> OUT_TIME_W<HOLD_SPEC> {
        OUT_TIME_W::new(self, 4)
    }
    #[doc = "Bit 7 - 1: spi master DMA full-duplex/half-duplex seg-trans ends or slave half-duplex seg-trans ends. And data has been pushed to corresponding memory. 0: seg-trans is not ended or not occurred. Can not be changed by CONF_buf."]
    #[inline(always)]
    #[must_use]
    pub fn dma_seg_trans_done(&mut self) -> DMA_SEG_TRANS_DONE_W<HOLD_SPEC> {
        DMA_SEG_TRANS_DONE_W::new(self, 7)
    }
}
#[doc = "SPI hold register\n\nYou can [`read`](crate::Reg::read) this register and get [`hold::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hold::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HOLD_SPEC;
impl crate::RegisterSpec for HOLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hold::R`](R) reader structure"]
impl crate::Readable for HOLD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hold::W`](W) writer structure"]
impl crate::Writable for HOLD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HOLD to value 0"]
impl crate::Resettable for HOLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
