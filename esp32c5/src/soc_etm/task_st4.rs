#[doc = "Register `TASK_ST4` reader"]
pub type R = crate::R<TASK_ST4_SPEC>;
#[doc = "Register `TASK_ST4` writer"]
pub type W = crate::W<TASK_ST4_SPEC>;
#[doc = "Field `REGDMA_TASK_START2_ST` reader - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_TASK_START2_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_TASK_START2_ST` writer - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_TASK_START2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_TASK_START3_ST` reader - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_TASK_START3_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_TASK_START3_ST` writer - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_TASK_START3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_START_SAMPLE_ST` reader - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TMPSNSR_TASK_START_SAMPLE_ST_R = crate::BitReader;
#[doc = "Field `TMPSNSR_TASK_START_SAMPLE_ST` writer - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TMPSNSR_TASK_START_SAMPLE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_TASK_STOP_SAMPLE_ST` reader - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TMPSNSR_TASK_STOP_SAMPLE_ST_R = crate::BitReader;
#[doc = "Field `TMPSNSR_TASK_STOP_SAMPLE_ST` writer - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TMPSNSR_TASK_STOP_SAMPLE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_RX_ST` reader - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_START_RX_ST_R = crate::BitReader;
#[doc = "Field `I2S0_TASK_START_RX_ST` writer - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_START_RX_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_START_TX_ST` reader - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_START_TX_ST_R = crate::BitReader;
#[doc = "Field `I2S0_TASK_START_TX_ST` writer - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_START_TX_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_RX_ST` reader - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_STOP_RX_ST_R = crate::BitReader;
#[doc = "Field `I2S0_TASK_STOP_RX_ST` writer - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_STOP_RX_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_TASK_STOP_TX_ST` reader - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_STOP_TX_ST_R = crate::BitReader;
#[doc = "Field `I2S0_TASK_STOP_TX_ST` writer - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_STOP_TX_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_WAKEUP_CPU_ST` reader - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_TASK_WAKEUP_CPU_ST_R = crate::BitReader;
#[doc = "Field `ULP_TASK_WAKEUP_CPU_ST` writer - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_TASK_WAKEUP_CPU_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_TASK_INT_CPU_ST` reader - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_TASK_INT_CPU_ST_R = crate::BitReader;
#[doc = "Field `ULP_TASK_INT_CPU_ST` writer - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_TASK_INT_CPU_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_START_ST` reader - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_START_ST_R = crate::BitReader;
#[doc = "Field `RTC_TASK_START_ST` writer - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_START_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_STOP_ST` reader - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_STOP_ST_R = crate::BitReader;
#[doc = "Field `RTC_TASK_STOP_ST` writer - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_STOP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_CLR_ST` reader - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_CLR_ST_R = crate::BitReader;
#[doc = "Field `RTC_TASK_CLR_ST` writer - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_CLR_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_TASK_TRIGGERFLW_ST` reader - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_TRIGGERFLW_ST_R = crate::BitReader;
#[doc = "Field `RTC_TASK_TRIGGERFLW_ST` writer - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_TASK_TRIGGERFLW_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_IN_START_CH0_ST` reader - Represents GDMA_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_IN_START_CH0_ST` writer - Represents GDMA_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_IN_START_CH1_ST` reader - Represents GDMA_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_IN_START_CH1_ST` writer - Represents GDMA_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_IN_START_CH2_ST` reader - Represents GDMA_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_IN_START_CH2_ST` writer - Represents GDMA_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_OUT_START_CH0_ST` reader - Represents GDMA_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_OUT_START_CH0_ST` writer - Represents GDMA_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_OUT_START_CH1_ST` reader - Represents GDMA_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_OUT_START_CH1_ST` writer - Represents GDMA_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_OUT_START_CH2_ST` reader - Represents GDMA_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_OUT_START_CH2_ST` writer - Represents GDMA_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` reader - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_R = crate::BitReader;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` writer - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start2_st(&self) -> REGDMA_TASK_START2_ST_R {
        REGDMA_TASK_START2_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start3_st(&self) -> REGDMA_TASK_START3_ST_R {
        REGDMA_TASK_START3_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_start_sample_st(&self) -> TMPSNSR_TASK_START_SAMPLE_ST_R {
        TMPSNSR_TASK_START_SAMPLE_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_stop_sample_st(&self) -> TMPSNSR_TASK_STOP_SAMPLE_ST_R {
        TMPSNSR_TASK_STOP_SAMPLE_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_rx_st(&self) -> I2S0_TASK_START_RX_ST_R {
        I2S0_TASK_START_RX_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_tx_st(&self) -> I2S0_TASK_START_TX_ST_R {
        I2S0_TASK_START_TX_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_rx_st(&self) -> I2S0_TASK_STOP_RX_ST_R {
        I2S0_TASK_STOP_RX_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_tx_st(&self) -> I2S0_TASK_STOP_TX_ST_R {
        I2S0_TASK_STOP_TX_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st(&self) -> ULP_TASK_WAKEUP_CPU_ST_R {
        ULP_TASK_WAKEUP_CPU_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st(&self) -> ULP_TASK_INT_CPU_ST_R {
        ULP_TASK_INT_CPU_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_start_st(&self) -> RTC_TASK_START_ST_R {
        RTC_TASK_START_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_stop_st(&self) -> RTC_TASK_STOP_ST_R {
        RTC_TASK_STOP_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_clr_st(&self) -> RTC_TASK_CLR_ST_R {
        RTC_TASK_CLR_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st(&self) -> RTC_TASK_TRIGGERFLW_ST_R {
        RTC_TASK_TRIGGERFLW_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents GDMA_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch0_st(&self) -> GDMA_TASK_IN_START_CH0_ST_R {
        GDMA_TASK_IN_START_CH0_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents GDMA_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch1_st(&self) -> GDMA_TASK_IN_START_CH1_ST_R {
        GDMA_TASK_IN_START_CH1_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents GDMA_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch2_st(&self) -> GDMA_TASK_IN_START_CH2_ST_R {
        GDMA_TASK_IN_START_CH2_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents GDMA_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch0_st(&self) -> GDMA_TASK_OUT_START_CH0_ST_R {
        GDMA_TASK_OUT_START_CH0_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents GDMA_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch1_st(&self) -> GDMA_TASK_OUT_START_CH1_ST_R {
        GDMA_TASK_OUT_START_CH1_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents GDMA_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch2_st(&self) -> GDMA_TASK_OUT_START_CH2_ST_R {
        GDMA_TASK_OUT_START_CH2_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&self) -> PMU_TASK_SLEEP_REQ_ST_R {
        PMU_TASK_SLEEP_REQ_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_ST4")
            .field("regdma_task_start2_st", &self.regdma_task_start2_st())
            .field("regdma_task_start3_st", &self.regdma_task_start3_st())
            .field(
                "tmpsnsr_task_start_sample_st",
                &self.tmpsnsr_task_start_sample_st(),
            )
            .field(
                "tmpsnsr_task_stop_sample_st",
                &self.tmpsnsr_task_stop_sample_st(),
            )
            .field("i2s0_task_start_rx_st", &self.i2s0_task_start_rx_st())
            .field("i2s0_task_start_tx_st", &self.i2s0_task_start_tx_st())
            .field("i2s0_task_stop_rx_st", &self.i2s0_task_stop_rx_st())
            .field("i2s0_task_stop_tx_st", &self.i2s0_task_stop_tx_st())
            .field("ulp_task_wakeup_cpu_st", &self.ulp_task_wakeup_cpu_st())
            .field("ulp_task_int_cpu_st", &self.ulp_task_int_cpu_st())
            .field("rtc_task_start_st", &self.rtc_task_start_st())
            .field("rtc_task_stop_st", &self.rtc_task_stop_st())
            .field("rtc_task_clr_st", &self.rtc_task_clr_st())
            .field("rtc_task_triggerflw_st", &self.rtc_task_triggerflw_st())
            .field(
                "gdma_task_in_start_ch0_st",
                &self.gdma_task_in_start_ch0_st(),
            )
            .field(
                "gdma_task_in_start_ch1_st",
                &self.gdma_task_in_start_ch1_st(),
            )
            .field(
                "gdma_task_in_start_ch2_st",
                &self.gdma_task_in_start_ch2_st(),
            )
            .field(
                "gdma_task_out_start_ch0_st",
                &self.gdma_task_out_start_ch0_st(),
            )
            .field(
                "gdma_task_out_start_ch1_st",
                &self.gdma_task_out_start_ch1_st(),
            )
            .field(
                "gdma_task_out_start_ch2_st",
                &self.gdma_task_out_start_ch2_st(),
            )
            .field("pmu_task_sleep_req_st", &self.pmu_task_sleep_req_st())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents REGDMA_task_start2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start2_st(&mut self) -> REGDMA_TASK_START2_ST_W<TASK_ST4_SPEC> {
        REGDMA_TASK_START2_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_task_start3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_task_start3_st(&mut self) -> REGDMA_TASK_START3_ST_W<TASK_ST4_SPEC> {
        REGDMA_TASK_START3_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents TMPSNSR_task_start_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_start_sample_st(
        &mut self,
    ) -> TMPSNSR_TASK_START_SAMPLE_ST_W<TASK_ST4_SPEC> {
        TMPSNSR_TASK_START_SAMPLE_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents TMPSNSR_task_stop_sample trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_task_stop_sample_st(&mut self) -> TMPSNSR_TASK_STOP_SAMPLE_ST_W<TASK_ST4_SPEC> {
        TMPSNSR_TASK_STOP_SAMPLE_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents I2S0_task_start_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_rx_st(&mut self) -> I2S0_TASK_START_RX_ST_W<TASK_ST4_SPEC> {
        I2S0_TASK_START_RX_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents I2S0_task_start_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_start_tx_st(&mut self) -> I2S0_TASK_START_TX_ST_W<TASK_ST4_SPEC> {
        I2S0_TASK_START_TX_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents I2S0_task_stop_rx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_rx_st(&mut self) -> I2S0_TASK_STOP_RX_ST_W<TASK_ST4_SPEC> {
        I2S0_TASK_STOP_RX_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents I2S0_task_stop_tx trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_stop_tx_st(&mut self) -> I2S0_TASK_STOP_TX_ST_W<TASK_ST4_SPEC> {
        I2S0_TASK_STOP_TX_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st(&mut self) -> ULP_TASK_WAKEUP_CPU_ST_W<TASK_ST4_SPEC> {
        ULP_TASK_WAKEUP_CPU_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st(&mut self) -> ULP_TASK_INT_CPU_ST_W<TASK_ST4_SPEC> {
        ULP_TASK_INT_CPU_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_start_st(&mut self) -> RTC_TASK_START_ST_W<TASK_ST4_SPEC> {
        RTC_TASK_START_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_stop_st(&mut self) -> RTC_TASK_STOP_ST_W<TASK_ST4_SPEC> {
        RTC_TASK_STOP_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_clr_st(&mut self) -> RTC_TASK_CLR_ST_W<TASK_ST4_SPEC> {
        RTC_TASK_CLR_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st(&mut self) -> RTC_TASK_TRIGGERFLW_ST_W<TASK_ST4_SPEC> {
        RTC_TASK_TRIGGERFLW_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents GDMA_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch0_st(&mut self) -> GDMA_TASK_IN_START_CH0_ST_W<TASK_ST4_SPEC> {
        GDMA_TASK_IN_START_CH0_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents GDMA_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch1_st(&mut self) -> GDMA_TASK_IN_START_CH1_ST_W<TASK_ST4_SPEC> {
        GDMA_TASK_IN_START_CH1_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents GDMA_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch2_st(&mut self) -> GDMA_TASK_IN_START_CH2_ST_W<TASK_ST4_SPEC> {
        GDMA_TASK_IN_START_CH2_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents GDMA_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch0_st(&mut self) -> GDMA_TASK_OUT_START_CH0_ST_W<TASK_ST4_SPEC> {
        GDMA_TASK_OUT_START_CH0_ST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents GDMA_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch1_st(&mut self) -> GDMA_TASK_OUT_START_CH1_ST_W<TASK_ST4_SPEC> {
        GDMA_TASK_OUT_START_CH1_ST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents GDMA_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch2_st(&mut self) -> GDMA_TASK_OUT_START_CH2_ST_W<TASK_ST4_SPEC> {
        GDMA_TASK_OUT_START_CH2_ST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&mut self) -> PMU_TASK_SLEEP_REQ_ST_W<TASK_ST4_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_W::new(self, 20)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST4_SPEC;
impl crate::RegisterSpec for TASK_ST4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st4::R`](R) reader structure"]
impl crate::Readable for TASK_ST4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_st4::W`](W) writer structure"]
impl crate::Writable for TASK_ST4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST4 to value 0"]
impl crate::Resettable for TASK_ST4_SPEC {}
