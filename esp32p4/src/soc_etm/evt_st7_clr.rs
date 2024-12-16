#[doc = "Register `EVT_ST7_CLR` writer"]
pub type W = crate::W<EVT_ST7_CLR_SPEC>;
#[doc = "Field `DMA2D_EVT_IN_SUC_EOF_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_IN_SUC_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_DONE_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_DONE_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_DONE_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_DONE_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_EOF_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_ST7_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear DMA2D_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_in_suc_eof_ch1_st_clr(
        &mut self,
    ) -> DMA2D_EVT_IN_SUC_EOF_CH1_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_IN_SUC_EOF_CH1_ST_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear DMA2D_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch0_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_DONE_CH0_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_DONE_CH0_ST_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear DMA2D_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch1_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_DONE_CH1_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_DONE_CH1_ST_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear DMA2D_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_done_ch2_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_DONE_CH2_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_DONE_CH2_ST_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear DMA2D_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch0_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_EOF_CH0_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_EOF_CH0_ST_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear DMA2D_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch1_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_EOF_CH1_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_EOF_CH1_ST_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear DMA2D_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_eof_ch2_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_EOF_CH2_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_EOF_CH2_ST_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear DMA2D_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch0_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch1_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch2_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<EVT_ST7_CLR_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W::new(self, 9)
    }
}
#[doc = "Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st7_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST7_CLR_SPEC;
impl crate::RegisterSpec for EVT_ST7_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evt_st7_clr::W`](W) writer structure"]
impl crate::Writable for EVT_ST7_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVT_ST7_CLR to value 0"]
impl crate::Resettable for EVT_ST7_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
