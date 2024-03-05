#[doc = "Register `EVT_ST5` reader"]
pub type R = crate::R<EVT_ST5_SPEC>;
#[doc = "Register `EVT_ST5` writer"]
pub type W = crate::W<EVT_ST5_SPEC>;
#[doc = "Field `ULP_EVT_ERR_INTR_ST` reader - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_EVT_ERR_INTR_ST_R = crate::BitReader;
#[doc = "Field `ULP_EVT_ERR_INTR_ST` writer - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_EVT_ERR_INTR_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_HALT_ST` reader - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_EVT_HALT_ST_R = crate::BitReader;
#[doc = "Field `ULP_EVT_HALT_ST` writer - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_EVT_HALT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ULP_EVT_START_INTR_ST` reader - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_EVT_START_INTR_ST_R = crate::BitReader;
#[doc = "Field `ULP_EVT_START_INTR_ST` writer - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ULP_EVT_START_INTR_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_TICK_ST` reader - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_EVT_TICK_ST_R = crate::BitReader;
#[doc = "Field `RTC_EVT_TICK_ST` writer - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_EVT_TICK_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_OVF_ST` reader - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_EVT_OVF_ST_R = crate::BitReader;
#[doc = "Field `RTC_EVT_OVF_ST` writer - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_EVT_OVF_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RTC_EVT_CMP_ST` reader - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_EVT_CMP_ST_R = crate::BitReader;
#[doc = "Field `RTC_EVT_CMP_ST` writer - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type RTC_EVT_CMP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH0_ST` reader - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH0_ST` writer - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH1_ST` reader - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH1_ST` writer - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH2_ST` reader - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH2_ST` writer - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST` reader - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST` writer - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST` reader - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST` writer - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST` reader - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST` writer - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST` reader - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST` writer - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST` reader - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST` writer - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST` reader - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST` writer - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST` reader - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST` writer - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST` reader - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST` writer - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST` reader - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST` writer - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH0_ST` reader - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_DONE_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH0_ST` writer - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH1_ST` reader - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_DONE_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH1_ST` writer - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH2_ST` reader - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_DONE_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_DONE_CH2_ST` writer - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_DONE_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH0_ST` reader - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH0_ST` writer - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH1_ST` reader - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH1_ST` writer - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH2_ST` reader - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_EOF_CH2_ST` writer - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST` reader - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST` writer - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST` reader - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST` writer - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST` reader - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST` writer - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST` reader - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST` writer - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST` reader - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST` writer - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST` reader - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST` writer - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST` reader - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST` writer - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST` reader - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST` writer - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st(&self) -> ULP_EVT_ERR_INTR_ST_R {
        ULP_EVT_ERR_INTR_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_halt_st(&self) -> ULP_EVT_HALT_ST_R {
        ULP_EVT_HALT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st(&self) -> ULP_EVT_START_INTR_ST_R {
        ULP_EVT_START_INTR_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_tick_st(&self) -> RTC_EVT_TICK_ST_R {
        RTC_EVT_TICK_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st(&self) -> RTC_EVT_OVF_ST_R {
        RTC_EVT_OVF_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st(&self) -> RTC_EVT_CMP_ST_R {
        RTC_EVT_CMP_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch0_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH0_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH0_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch1_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH1_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH1_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch2_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH2_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH2_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st(&self) -> PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_R {
        PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st(&self) -> PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_R {
        PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st(&self) -> PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_R {
        PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch0_st(&self) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_R {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch1_st(&self) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_R {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_empty_ch2_st(&self) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_R {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch0_st(&self) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_R {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch1_st(&self) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_R {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_fifo_full_ch2_st(&self) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_R {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch0_st(&self) -> PDMA_AHB_EVT_OUT_DONE_CH0_ST_R {
        PDMA_AHB_EVT_OUT_DONE_CH0_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch1_st(&self) -> PDMA_AHB_EVT_OUT_DONE_CH1_ST_R {
        PDMA_AHB_EVT_OUT_DONE_CH1_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_done_ch2_st(&self) -> PDMA_AHB_EVT_OUT_DONE_CH2_ST_R {
        PDMA_AHB_EVT_OUT_DONE_CH2_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch0_st(&self) -> PDMA_AHB_EVT_OUT_EOF_CH0_ST_R {
        PDMA_AHB_EVT_OUT_EOF_CH0_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch1_st(&self) -> PDMA_AHB_EVT_OUT_EOF_CH1_ST_R {
        PDMA_AHB_EVT_OUT_EOF_CH1_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_eof_ch2_st(&self) -> PDMA_AHB_EVT_OUT_EOF_CH2_ST_R {
        PDMA_AHB_EVT_OUT_EOF_CH2_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch0_st(&self) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_R {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch1_st(&self) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_R {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_total_eof_ch2_st(&self) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_R {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch0_st(&self) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_R {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch1_st(&self) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_R {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_empty_ch2_st(&self) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_R {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch0_st(&self) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_R {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_out_fifo_full_ch1_st(&self) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_R {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST5")
            .field(
                "ulp_evt_err_intr_st",
                &format_args!("{}", self.ulp_evt_err_intr_st().bit()),
            )
            .field(
                "ulp_evt_halt_st",
                &format_args!("{}", self.ulp_evt_halt_st().bit()),
            )
            .field(
                "ulp_evt_start_intr_st",
                &format_args!("{}", self.ulp_evt_start_intr_st().bit()),
            )
            .field(
                "rtc_evt_tick_st",
                &format_args!("{}", self.rtc_evt_tick_st().bit()),
            )
            .field(
                "rtc_evt_ovf_st",
                &format_args!("{}", self.rtc_evt_ovf_st().bit()),
            )
            .field(
                "rtc_evt_cmp_st",
                &format_args!("{}", self.rtc_evt_cmp_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_done_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_in_done_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_done_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_in_done_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_done_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_in_done_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_suc_eof_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_in_suc_eof_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_suc_eof_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_in_suc_eof_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_suc_eof_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_in_suc_eof_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_fifo_empty_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_in_fifo_empty_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_fifo_empty_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_in_fifo_empty_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_fifo_empty_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_in_fifo_empty_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_fifo_full_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_in_fifo_full_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_fifo_full_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_in_fifo_full_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_in_fifo_full_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_in_fifo_full_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_done_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_out_done_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_done_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_out_done_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_done_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_out_done_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_eof_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_out_eof_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_eof_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_out_eof_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_eof_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_out_eof_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_total_eof_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_out_total_eof_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_total_eof_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_out_total_eof_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_total_eof_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_out_total_eof_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_fifo_empty_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_out_fifo_empty_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_fifo_empty_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_out_fifo_empty_ch1_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_fifo_empty_ch2_st",
                &format_args!("{}", self.pdma_ahb_evt_out_fifo_empty_ch2_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_fifo_full_ch0_st",
                &format_args!("{}", self.pdma_ahb_evt_out_fifo_full_ch0_st().bit()),
            )
            .field(
                "pdma_ahb_evt_out_fifo_full_ch1_st",
                &format_args!("{}", self.pdma_ahb_evt_out_fifo_full_ch1_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_ST5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_evt_err_intr_st(&mut self) -> ULP_EVT_ERR_INTR_ST_W<EVT_ST5_SPEC> {
        ULP_EVT_ERR_INTR_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_evt_halt_st(&mut self) -> ULP_EVT_HALT_ST_W<EVT_ST5_SPEC> {
        ULP_EVT_HALT_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn ulp_evt_start_intr_st(&mut self) -> ULP_EVT_START_INTR_ST_W<EVT_ST5_SPEC> {
        ULP_EVT_START_INTR_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_evt_tick_st(&mut self) -> RTC_EVT_TICK_ST_W<EVT_ST5_SPEC> {
        RTC_EVT_TICK_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_evt_ovf_st(&mut self) -> RTC_EVT_OVF_ST_W<EVT_ST5_SPEC> {
        RTC_EVT_OVF_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn rtc_evt_cmp_st(&mut self) -> RTC_EVT_CMP_ST_W<EVT_ST5_SPEC> {
        RTC_EVT_CMP_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_done_ch0_st(&mut self) -> PDMA_AHB_EVT_IN_DONE_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH0_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_done_ch1_st(&mut self) -> PDMA_AHB_EVT_IN_DONE_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH1_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_done_ch2_st(&mut self) -> PDMA_AHB_EVT_IN_DONE_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH2_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents PDMA_AHB_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_empty_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH0_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents PDMA_AHB_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_empty_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH1_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents PDMA_AHB_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_empty_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_EMPTY_CH2_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents PDMA_AHB_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_full_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH0_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents PDMA_AHB_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_full_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH1_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents PDMA_AHB_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_in_fifo_full_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_IN_FIFO_FULL_CH2_ST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents PDMA_AHB_evt_out_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_done_ch0_st(&mut self) -> PDMA_AHB_EVT_OUT_DONE_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH0_ST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents PDMA_AHB_evt_out_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_done_ch1_st(&mut self) -> PDMA_AHB_EVT_OUT_DONE_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH1_ST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents PDMA_AHB_evt_out_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_done_ch2_st(&mut self) -> PDMA_AHB_EVT_OUT_DONE_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_DONE_CH2_ST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Represents PDMA_AHB_evt_out_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_eof_ch0_st(&mut self) -> PDMA_AHB_EVT_OUT_EOF_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH0_ST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Represents PDMA_AHB_evt_out_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_eof_ch1_st(&mut self) -> PDMA_AHB_EVT_OUT_EOF_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH1_ST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Represents PDMA_AHB_evt_out_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_eof_ch2_st(&mut self) -> PDMA_AHB_EVT_OUT_EOF_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_EOF_CH2_ST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_evt_out_total_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_total_eof_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH0_ST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_evt_out_total_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_total_eof_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH1_ST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_evt_out_total_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_total_eof_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_TOTAL_EOF_CH2_ST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_evt_out_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_empty_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH0_ST_W::new(self, 27)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_evt_out_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_empty_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH1_ST_W::new(self, 28)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_evt_out_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_empty_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_EMPTY_CH2_ST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Represents PDMA_AHB_evt_out_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_full_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH0_ST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Represents PDMA_AHB_evt_out_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn pdma_ahb_evt_out_fifo_full_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_W<EVT_ST5_SPEC> {
        PDMA_AHB_EVT_OUT_FIFO_FULL_CH1_ST_W::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st5::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st5::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST5_SPEC;
impl crate::RegisterSpec for EVT_ST5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st5::R`](R) reader structure"]
impl crate::Readable for EVT_ST5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st5::W`](W) writer structure"]
impl crate::Writable for EVT_ST5_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVT_ST5 to value 0"]
impl crate::Resettable for EVT_ST5_SPEC {
    const RESET_VALUE: u32 = 0;
}
