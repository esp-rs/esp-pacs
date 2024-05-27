///Register `EVT_ST5_CLR` writer
pub type W = crate::W<EVT_ST5_CLR_SPEC>;
///Field `ULP_EVT_ERR_INTR_ST_CLR` writer - Configures whether or not to clear ULP_evt_err_intr trigger status.\\0: Invalid, No effect\\1: Clear
pub type ULP_EVT_ERR_INTR_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULP_EVT_HALT_ST_CLR` writer - Configures whether or not to clear ULP_evt_halt trigger status.\\0: Invalid, No effect\\1: Clear
pub type ULP_EVT_HALT_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ULP_EVT_START_INTR_ST_CLR` writer - Configures whether or not to clear ULP_evt_start_intr trigger status.\\0: Invalid, No effect\\1: Clear
pub type ULP_EVT_START_INTR_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_EVT_TICK_ST_CLR` writer - Configures whether or not to clear RTC_evt_tick trigger status.\\0: Invalid, No effect\\1: Clear
pub type RTC_EVT_TICK_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_EVT_OVF_ST_CLR` writer - Configures whether or not to clear RTC_evt_ovf trigger status.\\0: Invalid, No effect\\1: Clear
pub type RTC_EVT_OVF_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RTC_EVT_CMP_ST_CLR` writer - Configures whether or not to clear RTC_evt_cmp trigger status.\\0: Invalid, No effect\\1: Clear
pub type RTC_EVT_CMP_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_done_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_done_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR` writer - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_ST5_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Configures whether or not to clear ULP_evt_err_intr trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn ulp_evt_err_intr_st_clr(&mut self) -> ULP_EVT_ERR_INTR_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        ULP_EVT_ERR_INTR_ST_CLR_W::new(self, 0)
    }
    ///Bit 1 - Configures whether or not to clear ULP_evt_halt trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn ulp_evt_halt_st_clr(&mut self) -> ULP_EVT_HALT_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        ULP_EVT_HALT_ST_CLR_W::new(self, 1)
    }
    ///Bit 2 - Configures whether or not to clear ULP_evt_start_intr trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn ulp_evt_start_intr_st_clr(&mut self) -> ULP_EVT_START_INTR_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        ULP_EVT_START_INTR_ST_CLR_W::new(self, 2)
    }
    ///Bit 3 - Configures whether or not to clear RTC_evt_tick trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn rtc_evt_tick_st_clr(&mut self) -> RTC_EVT_TICK_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        RTC_EVT_TICK_ST_CLR_W::new(self, 3)
    }
    ///Bit 4 - Configures whether or not to clear RTC_evt_ovf trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn rtc_evt_ovf_st_clr(&mut self) -> RTC_EVT_OVF_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        RTC_EVT_OVF_ST_CLR_W::new(self, 4)
    }
    ///Bit 5 - Configures whether or not to clear RTC_evt_cmp trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn rtc_evt_cmp_st_clr(&mut self) -> RTC_EVT_CMP_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        RTC_EVT_CMP_ST_CLR_W::new(self, 5)
    }
    ///Bit 6 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_done_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH0_ST_CLR_W::new(self, 6)
    }
    ///Bit 7 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_done_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH1_ST_CLR_W::new(self, 7)
    }
    ///Bit 8 - Configures whether or not to clear PDMA_AHB_evt_in_done_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_done_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH2_ST_CLR_W::new(self, 8)
    }
    ///Bit 9 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_CLR_W::new(self, 9)
    }
    ///Bit 10 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_CLR_W::new(self, 10)
    }
    ///Bit 11 - Configures whether or not to clear PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_CLR_W::new(self, 11)
    }
    ///Bit 12 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_CLR_W::new(self, 12)
    }
    ///Bit 13 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_CLR_W::new(self, 13)
    }
    ///Bit 14 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_CLR_W::new(self, 14)
    }
    ///Bit 15 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_CLR_W::new(self, 15)
    }
    ///Bit 16 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_CLR_W::new(self, 16)
    }
    ///Bit 17 - Configures whether or not to clear PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_full_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_CLR_W::new(self, 17)
    }
    ///Bit 18 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_done_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH0_ST_CLR_W::new(self, 18)
    }
    ///Bit 19 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_done_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH1_ST_CLR_W::new(self, 19)
    }
    ///Bit 20 - Configures whether or not to clear PDMA_AHB_evt_out_done_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_done_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH2_ST_CLR_W::new(self, 20)
    }
    ///Bit 21 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_eof_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH0_ST_CLR_W::new(self, 21)
    }
    ///Bit 22 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_eof_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH1_ST_CLR_W::new(self, 22)
    }
    ///Bit 23 - Configures whether or not to clear PDMA_AHB_evt_out_eof_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_eof_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH2_ST_CLR_W::new(self, 23)
    }
    ///Bit 24 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_total_eof_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_CLR_W::new(self, 24)
    }
    ///Bit 25 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_total_eof_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_CLR_W::new(self, 25)
    }
    ///Bit 26 - Configures whether or not to clear PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_total_eof_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_CLR_W::new(self, 26)
    }
    ///Bit 27 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_empty_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_CLR_W::new(self, 27)
    }
    ///Bit 28 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_empty_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_CLR_W::new(self, 28)
    }
    ///Bit 29 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_empty_ch2_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_CLR_W::new(self, 29)
    }
    ///Bit 30 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_full_ch0_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_CLR_W::new(self, 30)
    }
    ///Bit 31 - Configures whether or not to clear PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\0: Invalid, No effect\\1: Clear
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_full_ch1_st_clr(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR_W<EVT_ST5_CLR_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_CLR_W::new(self, 31)
    }
}
/**Events trigger status clear register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st5_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EVT_ST5_CLR_SPEC;
impl crate::RegisterSpec for EVT_ST5_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`evt_st5_clr::W`](W) writer structure
impl crate::Writable for EVT_ST5_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EVT_ST5_CLR to value 0
impl crate::Resettable for EVT_ST5_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
