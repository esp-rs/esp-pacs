#[doc = "Register `EVT_ST3` reader"]
pub type R = crate::R<EVT_ST3_SPEC>;
#[doc = "Register `EVT_ST3` writer"]
pub type W = crate::W<EVT_ST3_SPEC>;
#[doc = "Field `ADC_EVT_STARTED0_ST` reader - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_STARTED0_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_STARTED0_ST` writer - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_STARTED0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_DONE0_ST` reader - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE0_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_DONE0_ST` writer - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_DONE0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `REGDMA_EVT_ERR0_ST` reader - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR0_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR0_ST` writer - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_ERR1_ST` reader - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR1_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR1_ST` writer - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_ERR2_ST` reader - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR2_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR2_ST` writer - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REGDMA_EVT_ERR3_ST` reader - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR3_ST_R = crate::BitReader;
#[doc = "Field `REGDMA_EVT_ERR3_ST` writer - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type REGDMA_EVT_ERR3_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `GDMA_EVT_IN_DONE_CH0_ST` reader - Represents GDMA_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_DONE_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_DONE_CH0_ST` writer - Represents GDMA_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_DONE_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_DONE_CH1_ST` reader - Represents GDMA_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_DONE_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_DONE_CH1_ST` writer - Represents GDMA_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_DONE_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_DONE_CH2_ST` reader - Represents GDMA_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_DONE_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_DONE_CH2_ST` writer - Represents GDMA_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_DONE_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_SUC_EOF_CH0_ST` reader - Represents GDMA_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_SUC_EOF_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_SUC_EOF_CH0_ST` writer - Represents GDMA_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_SUC_EOF_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_SUC_EOF_CH1_ST` reader - Represents GDMA_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_SUC_EOF_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_SUC_EOF_CH1_ST` writer - Represents GDMA_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_SUC_EOF_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_SUC_EOF_CH2_ST` reader - Represents GDMA_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_SUC_EOF_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_SUC_EOF_CH2_ST` writer - Represents GDMA_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_SUC_EOF_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_FIFO_EMPTY_CH0_ST` reader - Represents GDMA_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_EMPTY_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_FIFO_EMPTY_CH0_ST` writer - Represents GDMA_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_EMPTY_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_FIFO_EMPTY_CH1_ST` reader - Represents GDMA_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_EMPTY_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_FIFO_EMPTY_CH1_ST` writer - Represents GDMA_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_EMPTY_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_FIFO_EMPTY_CH2_ST` reader - Represents GDMA_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_EMPTY_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_FIFO_EMPTY_CH2_ST` writer - Represents GDMA_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_EMPTY_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_FIFO_FULL_CH0_ST` reader - Represents GDMA_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_FULL_CH0_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_FIFO_FULL_CH0_ST` writer - Represents GDMA_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_FULL_CH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_FIFO_FULL_CH1_ST` reader - Represents GDMA_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_FULL_CH1_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_FIFO_FULL_CH1_ST` writer - Represents GDMA_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_FULL_CH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GDMA_EVT_IN_FIFO_FULL_CH2_ST` reader - Represents GDMA_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_FULL_CH2_ST_R = crate::BitReader;
#[doc = "Field `GDMA_EVT_IN_FIFO_FULL_CH2_ST` writer - Represents GDMA_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type GDMA_EVT_IN_FIFO_FULL_CH2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_started0_st(&self) -> ADC_EVT_STARTED0_ST_R {
        ADC_EVT_STARTED0_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done0_st(&self) -> REGDMA_EVT_DONE0_ST_R {
        REGDMA_EVT_DONE0_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done1_st(&self) -> REGDMA_EVT_DONE1_ST_R {
        REGDMA_EVT_DONE1_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done2_st(&self) -> REGDMA_EVT_DONE2_ST_R {
        REGDMA_EVT_DONE2_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done3_st(&self) -> REGDMA_EVT_DONE3_ST_R {
        REGDMA_EVT_DONE3_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err0_st(&self) -> REGDMA_EVT_ERR0_ST_R {
        REGDMA_EVT_ERR0_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err1_st(&self) -> REGDMA_EVT_ERR1_ST_R {
        REGDMA_EVT_ERR1_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err2_st(&self) -> REGDMA_EVT_ERR2_ST_R {
        REGDMA_EVT_ERR2_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err3_st(&self) -> REGDMA_EVT_ERR3_ST_R {
        REGDMA_EVT_ERR3_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_evt_over_limit_st(&self) -> TMPSNSR_EVT_OVER_LIMIT_ST_R {
        TMPSNSR_EVT_OVER_LIMIT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_rx_done_st(&self) -> I2S0_EVT_RX_DONE_ST_R {
        I2S0_EVT_RX_DONE_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_tx_done_st(&self) -> I2S0_EVT_TX_DONE_ST_R {
        I2S0_EVT_TX_DONE_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_received_st(&self) -> I2S0_EVT_X_WORDS_RECEIVED_ST_R {
        I2S0_EVT_X_WORDS_RECEIVED_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_sent_st(&self) -> I2S0_EVT_X_WORDS_SENT_ST_R {
        I2S0_EVT_X_WORDS_SENT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st(&self) -> ULP_EVT_ERR_INTR_ST_R {
        ULP_EVT_ERR_INTR_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_halt_st(&self) -> ULP_EVT_HALT_ST_R {
        ULP_EVT_HALT_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st(&self) -> ULP_EVT_START_INTR_ST_R {
        ULP_EVT_START_INTR_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_tick_st(&self) -> RTC_EVT_TICK_ST_R {
        RTC_EVT_TICK_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st(&self) -> RTC_EVT_OVF_ST_R {
        RTC_EVT_OVF_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st(&self) -> RTC_EVT_CMP_ST_R {
        RTC_EVT_CMP_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents GDMA_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_done_ch0_st(&self) -> GDMA_EVT_IN_DONE_CH0_ST_R {
        GDMA_EVT_IN_DONE_CH0_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents GDMA_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_done_ch1_st(&self) -> GDMA_EVT_IN_DONE_CH1_ST_R {
        GDMA_EVT_IN_DONE_CH1_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents GDMA_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_done_ch2_st(&self) -> GDMA_EVT_IN_DONE_CH2_ST_R {
        GDMA_EVT_IN_DONE_CH2_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents GDMA_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_suc_eof_ch0_st(&self) -> GDMA_EVT_IN_SUC_EOF_CH0_ST_R {
        GDMA_EVT_IN_SUC_EOF_CH0_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents GDMA_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_suc_eof_ch1_st(&self) -> GDMA_EVT_IN_SUC_EOF_CH1_ST_R {
        GDMA_EVT_IN_SUC_EOF_CH1_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents GDMA_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_suc_eof_ch2_st(&self) -> GDMA_EVT_IN_SUC_EOF_CH2_ST_R {
        GDMA_EVT_IN_SUC_EOF_CH2_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents GDMA_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_empty_ch0_st(&self) -> GDMA_EVT_IN_FIFO_EMPTY_CH0_ST_R {
        GDMA_EVT_IN_FIFO_EMPTY_CH0_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents GDMA_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_empty_ch1_st(&self) -> GDMA_EVT_IN_FIFO_EMPTY_CH1_ST_R {
        GDMA_EVT_IN_FIFO_EMPTY_CH1_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents GDMA_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_empty_ch2_st(&self) -> GDMA_EVT_IN_FIFO_EMPTY_CH2_ST_R {
        GDMA_EVT_IN_FIFO_EMPTY_CH2_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents GDMA_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_full_ch0_st(&self) -> GDMA_EVT_IN_FIFO_FULL_CH0_ST_R {
        GDMA_EVT_IN_FIFO_FULL_CH0_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents GDMA_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_full_ch1_st(&self) -> GDMA_EVT_IN_FIFO_FULL_CH1_ST_R {
        GDMA_EVT_IN_FIFO_FULL_CH1_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents GDMA_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_full_ch2_st(&self) -> GDMA_EVT_IN_FIFO_FULL_CH2_ST_R {
        GDMA_EVT_IN_FIFO_FULL_CH2_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST3")
            .field("adc_evt_started0_st", &self.adc_evt_started0_st())
            .field("regdma_evt_done0_st", &self.regdma_evt_done0_st())
            .field("regdma_evt_done1_st", &self.regdma_evt_done1_st())
            .field("regdma_evt_done2_st", &self.regdma_evt_done2_st())
            .field("regdma_evt_done3_st", &self.regdma_evt_done3_st())
            .field("regdma_evt_err0_st", &self.regdma_evt_err0_st())
            .field("regdma_evt_err1_st", &self.regdma_evt_err1_st())
            .field("regdma_evt_err2_st", &self.regdma_evt_err2_st())
            .field("regdma_evt_err3_st", &self.regdma_evt_err3_st())
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
            .field("ulp_evt_err_intr_st", &self.ulp_evt_err_intr_st())
            .field("ulp_evt_halt_st", &self.ulp_evt_halt_st())
            .field("ulp_evt_start_intr_st", &self.ulp_evt_start_intr_st())
            .field("rtc_evt_tick_st", &self.rtc_evt_tick_st())
            .field("rtc_evt_ovf_st", &self.rtc_evt_ovf_st())
            .field("rtc_evt_cmp_st", &self.rtc_evt_cmp_st())
            .field("gdma_evt_in_done_ch0_st", &self.gdma_evt_in_done_ch0_st())
            .field("gdma_evt_in_done_ch1_st", &self.gdma_evt_in_done_ch1_st())
            .field("gdma_evt_in_done_ch2_st", &self.gdma_evt_in_done_ch2_st())
            .field(
                "gdma_evt_in_suc_eof_ch0_st",
                &self.gdma_evt_in_suc_eof_ch0_st(),
            )
            .field(
                "gdma_evt_in_suc_eof_ch1_st",
                &self.gdma_evt_in_suc_eof_ch1_st(),
            )
            .field(
                "gdma_evt_in_suc_eof_ch2_st",
                &self.gdma_evt_in_suc_eof_ch2_st(),
            )
            .field(
                "gdma_evt_in_fifo_empty_ch0_st",
                &self.gdma_evt_in_fifo_empty_ch0_st(),
            )
            .field(
                "gdma_evt_in_fifo_empty_ch1_st",
                &self.gdma_evt_in_fifo_empty_ch1_st(),
            )
            .field(
                "gdma_evt_in_fifo_empty_ch2_st",
                &self.gdma_evt_in_fifo_empty_ch2_st(),
            )
            .field(
                "gdma_evt_in_fifo_full_ch0_st",
                &self.gdma_evt_in_fifo_full_ch0_st(),
            )
            .field(
                "gdma_evt_in_fifo_full_ch1_st",
                &self.gdma_evt_in_fifo_full_ch1_st(),
            )
            .field(
                "gdma_evt_in_fifo_full_ch2_st",
                &self.gdma_evt_in_fifo_full_ch2_st(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_started0_st(&mut self) -> ADC_EVT_STARTED0_ST_W<EVT_ST3_SPEC> {
        ADC_EVT_STARTED0_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done0_st(&mut self) -> REGDMA_EVT_DONE0_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_DONE0_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done1_st(&mut self) -> REGDMA_EVT_DONE1_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_DONE1_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done2_st(&mut self) -> REGDMA_EVT_DONE2_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_DONE2_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done3_st(&mut self) -> REGDMA_EVT_DONE3_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_DONE3_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err0_st(&mut self) -> REGDMA_EVT_ERR0_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_ERR0_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err1_st(&mut self) -> REGDMA_EVT_ERR1_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_ERR1_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err2_st(&mut self) -> REGDMA_EVT_ERR2_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_ERR2_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err3_st(&mut self) -> REGDMA_EVT_ERR3_ST_W<EVT_ST3_SPEC> {
        REGDMA_EVT_ERR3_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_evt_over_limit_st(&mut self) -> TMPSNSR_EVT_OVER_LIMIT_ST_W<EVT_ST3_SPEC> {
        TMPSNSR_EVT_OVER_LIMIT_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_rx_done_st(&mut self) -> I2S0_EVT_RX_DONE_ST_W<EVT_ST3_SPEC> {
        I2S0_EVT_RX_DONE_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_tx_done_st(&mut self) -> I2S0_EVT_TX_DONE_ST_W<EVT_ST3_SPEC> {
        I2S0_EVT_TX_DONE_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_received_st(&mut self) -> I2S0_EVT_X_WORDS_RECEIVED_ST_W<EVT_ST3_SPEC> {
        I2S0_EVT_X_WORDS_RECEIVED_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_sent_st(&mut self) -> I2S0_EVT_X_WORDS_SENT_ST_W<EVT_ST3_SPEC> {
        I2S0_EVT_X_WORDS_SENT_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents ULP_evt_err_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_err_intr_st(&mut self) -> ULP_EVT_ERR_INTR_ST_W<EVT_ST3_SPEC> {
        ULP_EVT_ERR_INTR_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents ULP_evt_halt trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_halt_st(&mut self) -> ULP_EVT_HALT_ST_W<EVT_ST3_SPEC> {
        ULP_EVT_HALT_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents ULP_evt_start_intr trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn ulp_evt_start_intr_st(&mut self) -> ULP_EVT_START_INTR_ST_W<EVT_ST3_SPEC> {
        ULP_EVT_START_INTR_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents RTC_evt_tick trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_tick_st(&mut self) -> RTC_EVT_TICK_ST_W<EVT_ST3_SPEC> {
        RTC_EVT_TICK_ST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents RTC_evt_ovf trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_ovf_st(&mut self) -> RTC_EVT_OVF_ST_W<EVT_ST3_SPEC> {
        RTC_EVT_OVF_ST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents RTC_evt_cmp trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn rtc_evt_cmp_st(&mut self) -> RTC_EVT_CMP_ST_W<EVT_ST3_SPEC> {
        RTC_EVT_CMP_ST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents GDMA_evt_in_done_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_done_ch0_st(&mut self) -> GDMA_EVT_IN_DONE_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_DONE_CH0_ST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Represents GDMA_evt_in_done_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_done_ch1_st(&mut self) -> GDMA_EVT_IN_DONE_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_DONE_CH1_ST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Represents GDMA_evt_in_done_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_done_ch2_st(&mut self) -> GDMA_EVT_IN_DONE_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_DONE_CH2_ST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Represents GDMA_evt_in_suc_eof_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_suc_eof_ch0_st(&mut self) -> GDMA_EVT_IN_SUC_EOF_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_SUC_EOF_CH0_ST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Represents GDMA_evt_in_suc_eof_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_suc_eof_ch1_st(&mut self) -> GDMA_EVT_IN_SUC_EOF_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_SUC_EOF_CH1_ST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Represents GDMA_evt_in_suc_eof_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_suc_eof_ch2_st(&mut self) -> GDMA_EVT_IN_SUC_EOF_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_SUC_EOF_CH2_ST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Represents GDMA_evt_in_fifo_empty_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_empty_ch0_st(
        &mut self,
    ) -> GDMA_EVT_IN_FIFO_EMPTY_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_FIFO_EMPTY_CH0_ST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Represents GDMA_evt_in_fifo_empty_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_empty_ch1_st(
        &mut self,
    ) -> GDMA_EVT_IN_FIFO_EMPTY_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_FIFO_EMPTY_CH1_ST_W::new(self, 27)
    }
    #[doc = "Bit 28 - Represents GDMA_evt_in_fifo_empty_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_empty_ch2_st(
        &mut self,
    ) -> GDMA_EVT_IN_FIFO_EMPTY_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_FIFO_EMPTY_CH2_ST_W::new(self, 28)
    }
    #[doc = "Bit 29 - Represents GDMA_evt_in_fifo_full_ch0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_full_ch0_st(&mut self) -> GDMA_EVT_IN_FIFO_FULL_CH0_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_FIFO_FULL_CH0_ST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Represents GDMA_evt_in_fifo_full_ch1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_full_ch1_st(&mut self) -> GDMA_EVT_IN_FIFO_FULL_CH1_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_FIFO_FULL_CH1_ST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Represents GDMA_evt_in_fifo_full_ch2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn gdma_evt_in_fifo_full_ch2_st(&mut self) -> GDMA_EVT_IN_FIFO_FULL_CH2_ST_W<EVT_ST3_SPEC> {
        GDMA_EVT_IN_FIFO_FULL_CH2_ST_W::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::Reg::read) this register and get [`evt_st3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`evt_st3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST3_SPEC;
impl crate::RegisterSpec for EVT_ST3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st3::R`](R) reader structure"]
impl crate::Readable for EVT_ST3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st3::W`](W) writer structure"]
impl crate::Writable for EVT_ST3_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVT_ST3 to value 0"]
impl crate::Resettable for EVT_ST3_SPEC {
    const RESET_VALUE: u32 = 0;
}
