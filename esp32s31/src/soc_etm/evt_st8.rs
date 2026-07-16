#[doc = "Register `EVT_ST8` reader"]
pub type R = crate::R<EVT_ST8_SPEC>;
#[doc = "Register `EVT_ST8` writer"]
pub type W = crate::W<EVT_ST8_SPEC>;
#[doc = "Field `REGDMA_EVT_DONE1_ST` reader - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE1_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE1_ST` writer - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_DONE2_ST` reader - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE2_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE2_ST` writer - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_DONE3_ST` reader - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE3_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE3_ST` writer - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_FAIL0_ST` reader - Represents REGDMA_evt_fail0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL0_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_FAIL0_ST` writer - Represents REGDMA_evt_fail0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_FAIL1_ST` reader - Represents REGDMA_evt_fail1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL1_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_FAIL1_ST` writer - Represents REGDMA_evt_fail1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_FAIL2_ST` reader - Represents REGDMA_evt_fail2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL2_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_FAIL2_ST` writer - Represents REGDMA_evt_fail2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_FAIL3_ST` reader - Represents REGDMA_evt_fail3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL3_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_FAIL3_ST` writer - Represents REGDMA_evt_fail3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_FAIL3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TMPSNSR_EVT_OVER_LIMIT_ST` reader - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TMPSNSR_EVT_OVER_LIMIT_ST_R = crate::BitReader;
#[doc = "Field `TMPSNSR_EVT_OVER_LIMIT_ST` writer - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type TMPSNSR_EVT_OVER_LIMIT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_RX_DONE_ST` reader - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_RX_DONE_ST_R = crate::BitReader;
#[doc = "Field `I2S0_EVT_RX_DONE_ST` writer - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_RX_DONE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_TX_DONE_ST` reader - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_TX_DONE_ST_R = crate::BitReader;
#[doc = "Field `I2S0_EVT_TX_DONE_ST` writer - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_TX_DONE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_X_WORDS_RECEIVED_ST` reader - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_X_WORDS_RECEIVED_ST_R = crate::BitReader;
#[doc = "Field `I2S0_EVT_X_WORDS_RECEIVED_ST` writer - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_X_WORDS_RECEIVED_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S0_EVT_X_WORDS_SENT_ST` reader - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_X_WORDS_SENT_ST_R = crate::BitReader;
#[doc = "Field `I2S0_EVT_X_WORDS_SENT_ST` writer - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S0_EVT_X_WORDS_SENT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_RX_DONE_ST` reader - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_RX_DONE_ST_R = crate::BitReader;
#[doc = "Field `I2S1_EVT_RX_DONE_ST` writer - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_RX_DONE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_TX_DONE_ST` reader - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_TX_DONE_ST_R = crate::BitReader;
#[doc = "Field `I2S1_EVT_TX_DONE_ST` writer - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_TX_DONE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_X_WORDS_RECEIVED_ST` reader - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_X_WORDS_RECEIVED_ST_R = crate::BitReader;
#[doc = "Field `I2S1_EVT_X_WORDS_RECEIVED_ST` writer - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_X_WORDS_RECEIVED_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S1_EVT_X_WORDS_SENT_ST` reader - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_X_WORDS_SENT_ST_R = crate::BitReader;
#[doc = "Field `I2S1_EVT_X_WORDS_SENT_ST` writer - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S1_EVT_X_WORDS_SENT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PMU_EVT_SLEEP_WEEKUP_ST` reader - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_EVT_SLEEP_WEEKUP_ST_R = crate::BitReader;
#[doc = "Field `PMU_EVT_SLEEP_WEEKUP_ST` writer - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PMU_EVT_SLEEP_WEEKUP_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH3_ST` reader - Represents PDMA_AHB_evt_in_done_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH3_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH3_ST` writer - Represents PDMA_AHB_evt_in_done_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH4_ST` reader - Represents PDMA_AHB_evt_in_done_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH4_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_DONE_CH4_ST` writer - Represents PDMA_AHB_evt_in_done_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_DONE_CH4_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST` reader - Represents PDMA_AHB_evt_in_suc_eof_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST_R = crate::BitReader;
#[doc = "Field `PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST` writer - Represents PDMA_AHB_evt_in_suc_eof_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done1_st(&self) -> REGDMA_EVT_DONE1_ST_R {
        REGDMA_EVT_DONE1_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done2_st(&self) -> REGDMA_EVT_DONE2_ST_R {
        REGDMA_EVT_DONE2_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done3_st(&self) -> REGDMA_EVT_DONE3_ST_R {
        REGDMA_EVT_DONE3_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents REGDMA_evt_fail0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail0_st(&self) -> REGDMA_EVT_FAIL0_ST_R {
        REGDMA_EVT_FAIL0_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents REGDMA_evt_fail1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail1_st(&self) -> REGDMA_EVT_FAIL1_ST_R {
        REGDMA_EVT_FAIL1_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents REGDMA_evt_fail2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail2_st(&self) -> REGDMA_EVT_FAIL2_ST_R {
        REGDMA_EVT_FAIL2_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents REGDMA_evt_fail3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail3_st(&self) -> REGDMA_EVT_FAIL3_ST_R {
        REGDMA_EVT_FAIL3_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_evt_over_limit_st(&self) -> TMPSNSR_EVT_OVER_LIMIT_ST_R {
        TMPSNSR_EVT_OVER_LIMIT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_rx_done_st(&self) -> I2S0_EVT_RX_DONE_ST_R {
        I2S0_EVT_RX_DONE_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_tx_done_st(&self) -> I2S0_EVT_TX_DONE_ST_R {
        I2S0_EVT_TX_DONE_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_received_st(&self) -> I2S0_EVT_X_WORDS_RECEIVED_ST_R {
        I2S0_EVT_X_WORDS_RECEIVED_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_sent_st(&self) -> I2S0_EVT_X_WORDS_SENT_ST_R {
        I2S0_EVT_X_WORDS_SENT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_rx_done_st(&self) -> I2S1_EVT_RX_DONE_ST_R {
        I2S1_EVT_RX_DONE_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_tx_done_st(&self) -> I2S1_EVT_TX_DONE_ST_R {
        I2S1_EVT_TX_DONE_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_received_st(&self) -> I2S1_EVT_X_WORDS_RECEIVED_ST_R {
        I2S1_EVT_X_WORDS_RECEIVED_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_sent_st(&self) -> I2S1_EVT_X_WORDS_SENT_ST_R {
        I2S1_EVT_X_WORDS_SENT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st(&self) -> ULP_EVT_ERR_INTR_ST_R {
        ULP_EVT_ERR_INTR_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_halt_st(&self) -> ULP_EVT_HALT_ST_R {
        ULP_EVT_HALT_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st(&self) -> ULP_EVT_START_INTR_ST_R {
        ULP_EVT_START_INTR_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_tick_st(&self) -> RTC_EVT_TICK_ST_R {
        RTC_EVT_TICK_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st(&self) -> RTC_EVT_OVF_ST_R {
        RTC_EVT_OVF_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st(&self) -> RTC_EVT_CMP_ST_R {
        RTC_EVT_CMP_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st(&self) -> PMU_EVT_SLEEP_WEEKUP_ST_R {
        PMU_EVT_SLEEP_WEEKUP_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch0_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH0_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH0_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch1_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH1_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH1_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch2_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH2_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH2_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_evt_in_done_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch3_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH3_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH3_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_evt_in_done_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch4_st(&self) -> PDMA_AHB_EVT_IN_DONE_CH4_ST_R {
        PDMA_AHB_EVT_IN_DONE_CH4_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st(&self) -> PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_R {
        PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st(&self) -> PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_R {
        PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st(&self) -> PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_R {
        PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents PDMA_AHB_evt_in_suc_eof_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch3_st(&self) -> PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST_R {
        PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST8")
            .field("regdma_evt_done1_st", &self.regdma_evt_done1_st())
            .field("regdma_evt_done2_st", &self.regdma_evt_done2_st())
            .field("regdma_evt_done3_st", &self.regdma_evt_done3_st())
            .field("regdma_evt_fail0_st", &self.regdma_evt_fail0_st())
            .field("regdma_evt_fail1_st", &self.regdma_evt_fail1_st())
            .field("regdma_evt_fail2_st", &self.regdma_evt_fail2_st())
            .field("regdma_evt_fail3_st", &self.regdma_evt_fail3_st())
            .field(
                "tmpsnsr_evt_over_limit_st",
                &self.tmpsnsr_evt_over_limit_st(),
            )
            .field("i2s0_evt_rx_done_st", &self.i2s0_evt_rx_done_st())
            .field("i2s0_evt_tx_done_st", &self.i2s0_evt_tx_done_st())
            .field(
                "i2s0_evt_x_words_received_st",
                &self.i2s0_evt_x_words_received_st(),
            )
            .field("i2s0_evt_x_words_sent_st", &self.i2s0_evt_x_words_sent_st())
            .field("i2s1_evt_rx_done_st", &self.i2s1_evt_rx_done_st())
            .field("i2s1_evt_tx_done_st", &self.i2s1_evt_tx_done_st())
            .field(
                "i2s1_evt_x_words_received_st",
                &self.i2s1_evt_x_words_received_st(),
            )
            .field("i2s1_evt_x_words_sent_st", &self.i2s1_evt_x_words_sent_st())
            .field("ulp_evt_err_intr_st", &self.ulp_evt_err_intr_st())
            .field("ulp_evt_halt_st", &self.ulp_evt_halt_st())
            .field("ulp_evt_start_intr_st", &self.ulp_evt_start_intr_st())
            .field("rtc_evt_tick_st", &self.rtc_evt_tick_st())
            .field("rtc_evt_ovf_st", &self.rtc_evt_ovf_st())
            .field("rtc_evt_cmp_st", &self.rtc_evt_cmp_st())
            .field("pmu_evt_sleep_weekup_st", &self.pmu_evt_sleep_weekup_st())
            .field(
                "pdma_ahb_evt_in_done_ch0_st",
                &self.pdma_ahb_evt_in_done_ch0_st(),
            )
            .field(
                "pdma_ahb_evt_in_done_ch1_st",
                &self.pdma_ahb_evt_in_done_ch1_st(),
            )
            .field(
                "pdma_ahb_evt_in_done_ch2_st",
                &self.pdma_ahb_evt_in_done_ch2_st(),
            )
            .field(
                "pdma_ahb_evt_in_done_ch3_st",
                &self.pdma_ahb_evt_in_done_ch3_st(),
            )
            .field(
                "pdma_ahb_evt_in_done_ch4_st",
                &self.pdma_ahb_evt_in_done_ch4_st(),
            )
            .field(
                "pdma_ahb_evt_in_suc_eof_ch0_st",
                &self.pdma_ahb_evt_in_suc_eof_ch0_st(),
            )
            .field(
                "pdma_ahb_evt_in_suc_eof_ch1_st",
                &self.pdma_ahb_evt_in_suc_eof_ch1_st(),
            )
            .field(
                "pdma_ahb_evt_in_suc_eof_ch2_st",
                &self.pdma_ahb_evt_in_suc_eof_ch2_st(),
            )
            .field(
                "pdma_ahb_evt_in_suc_eof_ch3_st",
                &self.pdma_ahb_evt_in_suc_eof_ch3_st(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done1_st(&mut self) -> REGDMA_EVT_DONE1_ST_W<'_, EVT_ST8_SPEC> {
        REGDMA_EVT_DONE1_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done2_st(&mut self) -> REGDMA_EVT_DONE2_ST_W<'_, EVT_ST8_SPEC> {
        REGDMA_EVT_DONE2_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done3_st(&mut self) -> REGDMA_EVT_DONE3_ST_W<'_, EVT_ST8_SPEC> {
        REGDMA_EVT_DONE3_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents REGDMA_evt_fail0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail0_st(&mut self) -> REGDMA_EVT_FAIL0_ST_W<'_, EVT_ST8_SPEC> {
        REGDMA_EVT_FAIL0_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents REGDMA_evt_fail1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail1_st(&mut self) -> REGDMA_EVT_FAIL1_ST_W<'_, EVT_ST8_SPEC> {
        REGDMA_EVT_FAIL1_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents REGDMA_evt_fail2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail2_st(&mut self) -> REGDMA_EVT_FAIL2_ST_W<'_, EVT_ST8_SPEC> {
        REGDMA_EVT_FAIL2_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents REGDMA_evt_fail3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_fail3_st(&mut self) -> REGDMA_EVT_FAIL3_ST_W<'_, EVT_ST8_SPEC> {
        REGDMA_EVT_FAIL3_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_evt_over_limit_st(&mut self) -> TMPSNSR_EVT_OVER_LIMIT_ST_W<'_, EVT_ST8_SPEC> {
        TMPSNSR_EVT_OVER_LIMIT_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_rx_done_st(&mut self) -> I2S0_EVT_RX_DONE_ST_W<'_, EVT_ST8_SPEC> {
        I2S0_EVT_RX_DONE_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_tx_done_st(&mut self) -> I2S0_EVT_TX_DONE_ST_W<'_, EVT_ST8_SPEC> {
        I2S0_EVT_TX_DONE_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_received_st(
        &mut self,
    ) -> I2S0_EVT_X_WORDS_RECEIVED_ST_W<'_, EVT_ST8_SPEC> {
        I2S0_EVT_X_WORDS_RECEIVED_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_sent_st(&mut self) -> I2S0_EVT_X_WORDS_SENT_ST_W<'_, EVT_ST8_SPEC> {
        I2S0_EVT_X_WORDS_SENT_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_rx_done_st(&mut self) -> I2S1_EVT_RX_DONE_ST_W<'_, EVT_ST8_SPEC> {
        I2S1_EVT_RX_DONE_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_tx_done_st(&mut self) -> I2S1_EVT_TX_DONE_ST_W<'_, EVT_ST8_SPEC> {
        I2S1_EVT_TX_DONE_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_received_st(
        &mut self,
    ) -> I2S1_EVT_X_WORDS_RECEIVED_ST_W<'_, EVT_ST8_SPEC> {
        I2S1_EVT_X_WORDS_RECEIVED_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_sent_st(&mut self) -> I2S1_EVT_X_WORDS_SENT_ST_W<'_, EVT_ST8_SPEC> {
        I2S1_EVT_X_WORDS_SENT_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st(&mut self) -> ULP_EVT_ERR_INTR_ST_W<'_, EVT_ST8_SPEC> {
        ULP_EVT_ERR_INTR_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_halt_st(&mut self) -> ULP_EVT_HALT_ST_W<'_, EVT_ST8_SPEC> {
        ULP_EVT_HALT_ST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st(&mut self) -> ULP_EVT_START_INTR_ST_W<'_, EVT_ST8_SPEC> {
        ULP_EVT_START_INTR_ST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_tick_st(&mut self) -> RTC_EVT_TICK_ST_W<'_, EVT_ST8_SPEC> {
        RTC_EVT_TICK_ST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st(&mut self) -> RTC_EVT_OVF_ST_W<'_, EVT_ST8_SPEC> {
        RTC_EVT_OVF_ST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st(&mut self) -> RTC_EVT_CMP_ST_W<'_, EVT_ST8_SPEC> {
        RTC_EVT_CMP_ST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Represents PMU_evt_sleep_weekup trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pmu_evt_sleep_weekup_st(&mut self) -> PMU_EVT_SLEEP_WEEKUP_ST_W<'_, EVT_ST8_SPEC> {
        PMU_EVT_SLEEP_WEEKUP_ST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Represents PDMA_AHB_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH0_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH0_ST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Represents PDMA_AHB_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH1_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH1_ST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Represents PDMA_AHB_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH2_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH2_ST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Represents PDMA_AHB_evt_in_done_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch3_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH3_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH3_ST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Represents PDMA_AHB_evt_in_done_ch4 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_done_ch4_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_DONE_CH4_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_DONE_CH4_ST_W::new(self, 27)
    }
    #[doc = "Bit 28 - Represents PDMA_AHB_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch0_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH0_ST_W::new(self, 28)
    }
    #[doc = "Bit 29 - Represents PDMA_AHB_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch1_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH1_ST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Represents PDMA_AHB_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch2_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH2_ST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Represents PDMA_AHB_evt_in_suc_eof_ch3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn pdma_ahb_evt_in_suc_eof_ch3_st(
        &mut self,
    ) -> PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST_W<'_, EVT_ST8_SPEC> {
        PDMA_AHB_EVT_IN_SUC_EOF_CH3_ST_W::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST8_SPEC;
impl crate::RegisterSpec for EVT_ST8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st8::R`](R) reader structure"]
impl crate::Readable for EVT_ST8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st8::W`](W) writer structure"]
impl crate::Writable for EVT_ST8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EVT_ST8 to value 0"]
impl crate::Resettable for EVT_ST8_SPEC {}
