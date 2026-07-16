#[doc = "Register `EVT_ST12_CLR` writer"]
pub type W = crate::W<EVT_ST12_CLR_SPEC>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA2D_EVT_OUT_TOTAL_EOF_CH3_ST_CLR` writer - Configures whether or not to clear DMA2D_evt_out_total_eof_ch3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type DMA2D_EVT_OUT_TOTAL_EOF_CH3_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G0_ST_CLR` writer - Configures whether or not to clear MODEM_evt_g0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type MODEM_EVT_G0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G1_ST_CLR` writer - Configures whether or not to clear MODEM_evt_g1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type MODEM_EVT_G1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G2_ST_CLR` writer - Configures whether or not to clear MODEM_evt_g2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type MODEM_EVT_G2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_EVT_G3_ST_CLR` writer - Configures whether or not to clear MODEM_evt_g3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type MODEM_EVT_G3_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_DELAY_CHANNEL_1_POS_ST_CLR` writer - Configures whether or not to clear ZERO_DET_evt_delay_channel_1_pos trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ZERO_DET_EVT_DELAY_CHANNEL_1_POS_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_DELAY_CHANNEL_2_POS_ST_CLR` writer - Configures whether or not to clear ZERO_DET_evt_delay_channel_2_pos trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ZERO_DET_EVT_DELAY_CHANNEL_2_POS_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_DELAY_CHANNEL_3_POS_ST_CLR` writer - Configures whether or not to clear ZERO_DET_evt_delay_channel_3_pos trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ZERO_DET_EVT_DELAY_CHANNEL_3_POS_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_DELAY_CHANNEL_1_NEG_ST_CLR` writer - Configures whether or not to clear ZERO_DET_evt_delay_channel_1_neg trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ZERO_DET_EVT_DELAY_CHANNEL_1_NEG_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_DELAY_CHANNEL_2_NEG_ST_CLR` writer - Configures whether or not to clear ZERO_DET_evt_delay_channel_2_neg trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ZERO_DET_EVT_DELAY_CHANNEL_2_NEG_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_EVT_DELAY_CHANNEL_3_NEG_ST_CLR` writer - Configures whether or not to clear ZERO_DET_evt_delay_channel_3_neg trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ZERO_DET_EVT_DELAY_CHANNEL_3_NEG_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CORDIC_EVT_RES_RDY_ST_CLR` writer - Configures whether or not to clear CORDIC_evt_res_rdy trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CORDIC_EVT_RES_RDY_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_ST12_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear DMA2D_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch1_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear DMA2D_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch2_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear DMA2D_evt_out_total_eof_ch3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn dma2d_evt_out_total_eof_ch3_st_clr(
        &mut self,
    ) -> DMA2D_EVT_OUT_TOTAL_EOF_CH3_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        DMA2D_EVT_OUT_TOTAL_EOF_CH3_ST_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear MODEM_evt_g0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn modem_evt_g0_st_clr(&mut self) -> MODEM_EVT_G0_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        MODEM_EVT_G0_ST_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear MODEM_evt_g1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn modem_evt_g1_st_clr(&mut self) -> MODEM_EVT_G1_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        MODEM_EVT_G1_ST_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear MODEM_evt_g2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn modem_evt_g2_st_clr(&mut self) -> MODEM_EVT_G2_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        MODEM_EVT_G2_ST_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear MODEM_evt_g3 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn modem_evt_g3_st_clr(&mut self) -> MODEM_EVT_G3_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        MODEM_EVT_G3_ST_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear ZERO_DET_evt_delay_channel_1_pos trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn zero_det_evt_delay_channel_1_pos_st_clr(
        &mut self,
    ) -> ZERO_DET_EVT_DELAY_CHANNEL_1_POS_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        ZERO_DET_EVT_DELAY_CHANNEL_1_POS_ST_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear ZERO_DET_evt_delay_channel_2_pos trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn zero_det_evt_delay_channel_2_pos_st_clr(
        &mut self,
    ) -> ZERO_DET_EVT_DELAY_CHANNEL_2_POS_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        ZERO_DET_EVT_DELAY_CHANNEL_2_POS_ST_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear ZERO_DET_evt_delay_channel_3_pos trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn zero_det_evt_delay_channel_3_pos_st_clr(
        &mut self,
    ) -> ZERO_DET_EVT_DELAY_CHANNEL_3_POS_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        ZERO_DET_EVT_DELAY_CHANNEL_3_POS_ST_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear ZERO_DET_evt_delay_channel_1_neg trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn zero_det_evt_delay_channel_1_neg_st_clr(
        &mut self,
    ) -> ZERO_DET_EVT_DELAY_CHANNEL_1_NEG_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        ZERO_DET_EVT_DELAY_CHANNEL_1_NEG_ST_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear ZERO_DET_evt_delay_channel_2_neg trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn zero_det_evt_delay_channel_2_neg_st_clr(
        &mut self,
    ) -> ZERO_DET_EVT_DELAY_CHANNEL_2_NEG_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        ZERO_DET_EVT_DELAY_CHANNEL_2_NEG_ST_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear ZERO_DET_evt_delay_channel_3_neg trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn zero_det_evt_delay_channel_3_neg_st_clr(
        &mut self,
    ) -> ZERO_DET_EVT_DELAY_CHANNEL_3_NEG_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        ZERO_DET_EVT_DELAY_CHANNEL_3_NEG_ST_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear CORDIC_evt_res_rdy trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn cordic_evt_res_rdy_st_clr(
        &mut self,
    ) -> CORDIC_EVT_RES_RDY_ST_CLR_W<'_, EVT_ST12_CLR_SPEC> {
        CORDIC_EVT_RES_RDY_ST_CLR_W::new(self, 13)
    }
}
#[doc = "Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st12_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST12_CLR_SPEC;
impl crate::RegisterSpec for EVT_ST12_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evt_st12_clr::W`](W) writer structure"]
impl crate::Writable for EVT_ST12_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST12_CLR to value 0"]
impl crate::Resettable for EVT_ST12_CLR_SPEC {}
