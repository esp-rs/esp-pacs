#[doc = "Register `TASK_ST5` reader"]
pub type R = crate::R<TASK_ST5_SPEC>;
#[doc = "Register `TASK_ST5` writer"]
pub type W = crate::W<TASK_ST5_SPEC>;
#[doc = "Field `I2S0_TASK_SYNC_CHECK_ST` reader - Represents I2S0_task_sync_check trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_SYNC_CHECK_ST_R = crate::BitReader;
#[doc = "Field `I2S0_TASK_SYNC_CHECK_ST` writer - Represents I2S0_task_sync_check trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_TASK_SYNC_CHECK_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `GDMA_TASK_IN_START_CH3_ST` reader - Represents GDMA_task_in_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH3_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_IN_START_CH3_ST` writer - Represents GDMA_task_in_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_IN_START_CH4_ST` reader - Represents GDMA_task_in_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH4_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_IN_START_CH4_ST` writer - Represents GDMA_task_in_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_IN_START_CH4_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `GDMA_TASK_OUT_START_CH3_ST` reader - Represents GDMA_task_out_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH3_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_OUT_START_CH3_ST` writer - Represents GDMA_task_out_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_TASK_OUT_START_CH4_ST` reader - Represents GDMA_task_out_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH4_ST_R = crate::BitReader;
#[doc = "Field `GDMA_TASK_OUT_START_CH4_ST` writer - Represents GDMA_task_out_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_TASK_OUT_START_CH4_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` reader - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_R = crate::BitReader;
#[doc = "Field `PMU_TASK_SLEEP_REQ_ST` writer - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_TASK_SLEEP_REQ_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_TASK_G0_ST` reader - Represents MODEM_task_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G0_ST_R = crate::BitReader;
#[doc = "Field `MODEM_TASK_G0_ST` writer - Represents MODEM_task_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_TASK_G1_ST` reader - Represents MODEM_task_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G1_ST_R = crate::BitReader;
#[doc = "Field `MODEM_TASK_G1_ST` writer - Represents MODEM_task_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_TASK_G2_ST` reader - Represents MODEM_task_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G2_ST_R = crate::BitReader;
#[doc = "Field `MODEM_TASK_G2_ST` writer - Represents MODEM_task_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_TASK_G3_ST` reader - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G3_ST_R = crate::BitReader;
#[doc = "Field `MODEM_TASK_G3_ST` writer - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MODEM_TASK_G3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ZERO_DET_TASK_START_ST` reader - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_TASK_START_ST_R = crate::BitReader;
#[doc = "Field `ZERO_DET_TASK_START_ST` writer - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ZERO_DET_TASK_START_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents I2S0_task_sync_check trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_sync_check_st(&self) -> I2S0_TASK_SYNC_CHECK_ST_R {
        I2S0_TASK_SYNC_CHECK_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st(&self) -> ULP_TASK_WAKEUP_CPU_ST_R {
        ULP_TASK_WAKEUP_CPU_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st(&self) -> ULP_TASK_INT_CPU_ST_R {
        ULP_TASK_INT_CPU_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_start_st(&self) -> RTC_TASK_START_ST_R {
        RTC_TASK_START_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_stop_st(&self) -> RTC_TASK_STOP_ST_R {
        RTC_TASK_STOP_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_clr_st(&self) -> RTC_TASK_CLR_ST_R {
        RTC_TASK_CLR_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st(&self) -> RTC_TASK_TRIGGERFLW_ST_R {
        RTC_TASK_TRIGGERFLW_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents GDMA_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch0_st(&self) -> GDMA_TASK_IN_START_CH0_ST_R {
        GDMA_TASK_IN_START_CH0_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents GDMA_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch1_st(&self) -> GDMA_TASK_IN_START_CH1_ST_R {
        GDMA_TASK_IN_START_CH1_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents GDMA_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch2_st(&self) -> GDMA_TASK_IN_START_CH2_ST_R {
        GDMA_TASK_IN_START_CH2_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents GDMA_task_in_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch3_st(&self) -> GDMA_TASK_IN_START_CH3_ST_R {
        GDMA_TASK_IN_START_CH3_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents GDMA_task_in_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch4_st(&self) -> GDMA_TASK_IN_START_CH4_ST_R {
        GDMA_TASK_IN_START_CH4_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents GDMA_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch0_st(&self) -> GDMA_TASK_OUT_START_CH0_ST_R {
        GDMA_TASK_OUT_START_CH0_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents GDMA_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch1_st(&self) -> GDMA_TASK_OUT_START_CH1_ST_R {
        GDMA_TASK_OUT_START_CH1_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents GDMA_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch2_st(&self) -> GDMA_TASK_OUT_START_CH2_ST_R {
        GDMA_TASK_OUT_START_CH2_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents GDMA_task_out_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch3_st(&self) -> GDMA_TASK_OUT_START_CH3_ST_R {
        GDMA_TASK_OUT_START_CH3_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents GDMA_task_out_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch4_st(&self) -> GDMA_TASK_OUT_START_CH4_ST_R {
        GDMA_TASK_OUT_START_CH4_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&self) -> PMU_TASK_SLEEP_REQ_ST_R {
        PMU_TASK_SLEEP_REQ_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents MODEM_task_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g0_st(&self) -> MODEM_TASK_G0_ST_R {
        MODEM_TASK_G0_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents MODEM_task_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g1_st(&self) -> MODEM_TASK_G1_ST_R {
        MODEM_TASK_G1_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents MODEM_task_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g2_st(&self) -> MODEM_TASK_G2_ST_R {
        MODEM_TASK_G2_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g3_st(&self) -> MODEM_TASK_G3_ST_R {
        MODEM_TASK_G3_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_task_start_st(&self) -> ZERO_DET_TASK_START_ST_R {
        ZERO_DET_TASK_START_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TASK_ST5")
            .field("i2s0_task_sync_check_st", &self.i2s0_task_sync_check_st())
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
                "gdma_task_in_start_ch3_st",
                &self.gdma_task_in_start_ch3_st(),
            )
            .field(
                "gdma_task_in_start_ch4_st",
                &self.gdma_task_in_start_ch4_st(),
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
            .field(
                "gdma_task_out_start_ch3_st",
                &self.gdma_task_out_start_ch3_st(),
            )
            .field(
                "gdma_task_out_start_ch4_st",
                &self.gdma_task_out_start_ch4_st(),
            )
            .field("pmu_task_sleep_req_st", &self.pmu_task_sleep_req_st())
            .field("modem_task_g0_st", &self.modem_task_g0_st())
            .field("modem_task_g1_st", &self.modem_task_g1_st())
            .field("modem_task_g2_st", &self.modem_task_g2_st())
            .field("modem_task_g3_st", &self.modem_task_g3_st())
            .field("zero_det_task_start_st", &self.zero_det_task_start_st())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents I2S0_task_sync_check trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_task_sync_check_st(&mut self) -> I2S0_TASK_SYNC_CHECK_ST_W<'_, TASK_ST5_SPEC> {
        I2S0_TASK_SYNC_CHECK_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents ULP_task_wakeup_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_wakeup_cpu_st(&mut self) -> ULP_TASK_WAKEUP_CPU_ST_W<'_, TASK_ST5_SPEC> {
        ULP_TASK_WAKEUP_CPU_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents ULP_task_int_cpu trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_task_int_cpu_st(&mut self) -> ULP_TASK_INT_CPU_ST_W<'_, TASK_ST5_SPEC> {
        ULP_TASK_INT_CPU_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents RTC_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_start_st(&mut self) -> RTC_TASK_START_ST_W<'_, TASK_ST5_SPEC> {
        RTC_TASK_START_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents RTC_task_stop trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_stop_st(&mut self) -> RTC_TASK_STOP_ST_W<'_, TASK_ST5_SPEC> {
        RTC_TASK_STOP_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents RTC_task_clr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_clr_st(&mut self) -> RTC_TASK_CLR_ST_W<'_, TASK_ST5_SPEC> {
        RTC_TASK_CLR_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents RTC_task_triggerflw trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_task_triggerflw_st(&mut self) -> RTC_TASK_TRIGGERFLW_ST_W<'_, TASK_ST5_SPEC> {
        RTC_TASK_TRIGGERFLW_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents GDMA_task_in_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch0_st(&mut self) -> GDMA_TASK_IN_START_CH0_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_IN_START_CH0_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents GDMA_task_in_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch1_st(&mut self) -> GDMA_TASK_IN_START_CH1_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_IN_START_CH1_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents GDMA_task_in_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch2_st(&mut self) -> GDMA_TASK_IN_START_CH2_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_IN_START_CH2_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents GDMA_task_in_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch3_st(&mut self) -> GDMA_TASK_IN_START_CH3_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_IN_START_CH3_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents GDMA_task_in_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_in_start_ch4_st(&mut self) -> GDMA_TASK_IN_START_CH4_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_IN_START_CH4_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents GDMA_task_out_start_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch0_st(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH0_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_OUT_START_CH0_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents GDMA_task_out_start_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch1_st(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH1_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_OUT_START_CH1_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents GDMA_task_out_start_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch2_st(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH2_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_OUT_START_CH2_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents GDMA_task_out_start_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch3_st(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH3_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_OUT_START_CH3_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents GDMA_task_out_start_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_task_out_start_ch4_st(
        &mut self,
    ) -> GDMA_TASK_OUT_START_CH4_ST_W<'_, TASK_ST5_SPEC> {
        GDMA_TASK_OUT_START_CH4_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents PMU_task_sleep_req trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_task_sleep_req_st(&mut self) -> PMU_TASK_SLEEP_REQ_ST_W<'_, TASK_ST5_SPEC> {
        PMU_TASK_SLEEP_REQ_ST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents MODEM_task_g0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g0_st(&mut self) -> MODEM_TASK_G0_ST_W<'_, TASK_ST5_SPEC> {
        MODEM_TASK_G0_ST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents MODEM_task_g1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g1_st(&mut self) -> MODEM_TASK_G1_ST_W<'_, TASK_ST5_SPEC> {
        MODEM_TASK_G1_ST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents MODEM_task_g2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g2_st(&mut self) -> MODEM_TASK_G2_ST_W<'_, TASK_ST5_SPEC> {
        MODEM_TASK_G2_ST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Represents MODEM_task_g3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn modem_task_g3_st(&mut self) -> MODEM_TASK_G3_ST_W<'_, TASK_ST5_SPEC> {
        MODEM_TASK_G3_ST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Represents ZERO_DET_task_start trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn zero_det_task_start_st(&mut self) -> ZERO_DET_TASK_START_ST_W<'_, TASK_ST5_SPEC> {
        ZERO_DET_TASK_START_ST_W::new(self, 22)
    }
}
#[doc = "Tasks trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`task_st5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`task_st5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TASK_ST5_SPEC;
impl crate::RegisterSpec for TASK_ST5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`task_st5::R`](R) reader structure"]
impl crate::Readable for TASK_ST5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`task_st5::W`](W) writer structure"]
impl crate::Writable for TASK_ST5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TASK_ST5 to value 0"]
impl crate::Resettable for TASK_ST5_SPEC {}
