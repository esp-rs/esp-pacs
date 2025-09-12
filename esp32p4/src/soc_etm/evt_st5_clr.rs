#[doc = "Register `EVT_ST5_CLR` writer"]
pub type W = crate::W<EVT_ST5_CLR_SPEC>;
#[doc = "Field `ULP_EVT_ERR_INTR_ST_CLR` writer - Configures whether or not to clear ULP_evt_err_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ULP_EVT_ERR_INTR_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_HALT_ST_CLR` writer - Configures whether or not to clear ULP_evt_halt trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ULP_EVT_HALT_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_START_INTR_ST_CLR` writer - Configures whether or not to clear ULP_evt_start_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type ULP_EVT_START_INTR_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_TICK_ST_CLR` writer - Configures whether or not to clear RTC_evt_tick trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RTC_EVT_TICK_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_OVF_ST_CLR` writer - Configures whether or not to clear RTC_evt_ovf trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RTC_EVT_OVF_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_CMP_ST_CLR` writer - Configures whether or not to clear RTC_evt_cmp trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type RTC_EVT_CMP_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_ST5_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear ULP_evt_err_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st_clr(&mut self) -> ULP_EVT_ERR_INTR_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        ULP_EVT_ERR_INTR_ST_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear ULP_evt_halt trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_evt_halt_st_clr(&mut self) -> ULP_EVT_HALT_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        ULP_EVT_HALT_ST_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear ULP_evt_start_intr trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st_clr(
        &mut self,
    ) -> ULP_EVT_START_INTR_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        ULP_EVT_START_INTR_ST_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear RTC_evt_tick trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_evt_tick_st_clr(&mut self) -> RTC_EVT_TICK_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        RTC_EVT_TICK_ST_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear RTC_evt_ovf trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st_clr(&mut self) -> RTC_EVT_OVF_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        RTC_EVT_OVF_ST_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear RTC_evt_cmp trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st_clr(&mut self) -> RTC_EVT_CMP_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        RTC_EVT_CMP_ST_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR_W<'_, EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR_W::new(self, 31)
    }
}
#[doc = "Events trigger status clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st5_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST5_CLR_SPEC;
impl crate::RegisterSpec for EVT_ST5_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`evt_st5_clr::W`](W) writer structure"]
impl crate::Writable for EVT_ST5_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST5_CLR to value 0"]
impl crate::Resettable for EVT_ST5_CLR_SPEC {}
