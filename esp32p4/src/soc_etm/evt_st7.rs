#[doc = "Register `EVT_ST7` reader"]
pub type R = crate::R<EVT_ST7_SPEC>;
#[doc = "Register `EVT_ST7` writer"]
pub type W = crate::W<EVT_ST7_SPEC>;
#[doc = "Field `DMA2D_EVT_IN_SUC_EOF_CH1_ST` reader - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_IN_SUC_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_IN_SUC_EOF_CH1_ST` writer - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_IN_SUC_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH0_ST` reader - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_DONE_CH0_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH0_ST` writer - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH1_ST` reader - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_DONE_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH1_ST` writer - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH2_ST` reader - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_DONE_CH2_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH2_ST` writer - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_DONE_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH0_ST` reader - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH0_ST` writer - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH1_ST` reader - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH1_ST` writer - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH2_ST` reader - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH2_ST` writer - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST` reader - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST` writer - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST` reader - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST` writer - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST` reader - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST` writer - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_in_suc_eof_ch1_st(&self) -> DMA2D_EVT_IN_SUC_EOF_CH1_ST_R {
        DMA2D_EVT_IN_SUC_EOF_CH1_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch0_st(&self) -> DMA2D_EVT_OUT_DONE_CH0_ST_R {
        DMA2D_EVT_OUT_DONE_CH0_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch1_st(&self) -> DMA2D_EVT_OUT_DONE_CH1_ST_R {
        DMA2D_EVT_OUT_DONE_CH1_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch2_st(&self) -> DMA2D_EVT_OUT_DONE_CH2_ST_R {
        DMA2D_EVT_OUT_DONE_CH2_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch0_st(&self) -> DMA2D_EVT_OUT_EOF_CH0_ST_R {
        DMA2D_EVT_OUT_EOF_CH0_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch1_st(&self) -> DMA2D_EVT_OUT_EOF_CH1_ST_R {
        DMA2D_EVT_OUT_EOF_CH1_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch2_st(&self) -> DMA2D_EVT_OUT_EOF_CH2_ST_R {
        DMA2D_EVT_OUT_EOF_CH2_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch0_st(&self) -> DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_R {
        DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch1_st(&self) -> DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_R {
        DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch2_st(&self) -> DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_R {
        DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST7")
            .field(
                "dma2d_evt_in_suc_eof_ch1_st",
                &self.dma2d_evt_in_suc_eof_ch1_st(),
            )
            .field(
                "dma2d_evt_out_done_ch0_st",
                &self.dma2d_evt_out_done_ch0_st(),
            )
            .field(
                "dma2d_evt_out_done_ch1_st",
                &self.dma2d_evt_out_done_ch1_st(),
            )
            .field(
                "dma2d_evt_out_done_ch2_st",
                &self.dma2d_evt_out_done_ch2_st(),
            )
            .field("dma2d_evt_out_eof_ch0_st", &self.dma2d_evt_out_eof_ch0_st())
            .field("dma2d_evt_out_eof_ch1_st", &self.dma2d_evt_out_eof_ch1_st())
            .field("dma2d_evt_out_eof_ch2_st", &self.dma2d_evt_out_eof_ch2_st())
            .field(
                "dma2d_evt_out_total_eof_ch0_st",
                &self.dma2d_evt_out_total_eof_ch0_st(),
            )
            .field(
                "dma2d_evt_out_total_eof_ch1_st",
                &self.dma2d_evt_out_total_eof_ch1_st(),
            )
            .field(
                "dma2d_evt_out_total_eof_ch2_st",
                &self.dma2d_evt_out_total_eof_ch2_st(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_in_suc_eof_ch1_st(&mut self) -> DMA2D_EVT_IN_SUC_EOF_CH1_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_IN_SUC_EOF_CH1_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents DMA2D_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch0_st(&mut self) -> DMA2D_EVT_OUT_DONE_CH0_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_DONE_CH0_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents DMA2D_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch1_st(&mut self) -> DMA2D_EVT_OUT_DONE_CH1_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_DONE_CH1_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents DMA2D_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch2_st(&mut self) -> DMA2D_EVT_OUT_DONE_CH2_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_DONE_CH2_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch0_st(&mut self) -> DMA2D_EVT_OUT_EOF_CH0_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_EOF_CH0_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch1_st(&mut self) -> DMA2D_EVT_OUT_EOF_CH1_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_EOF_CH1_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch2_st(&mut self) -> DMA2D_EVT_OUT_EOF_CH2_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_EOF_CH2_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch0_st(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch1_st(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch2_st(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_W<EVT_ST7_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_W::new(self, 9)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST7_SPEC;
impl crate::RegisterSpec for EVT_ST7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st7::R`](R) reader structure"]
impl crate::Readable for EVT_ST7_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st7::W`](W) writer structure"]
impl crate::Writable for EVT_ST7_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST7 to value 0"]
impl crate::Resettable for EVT_ST7_SPEC {}
