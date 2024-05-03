#[doc = "Register `EVT_ST4` reader"]
pub type R = crate::R<EVT_ST4_SPEC>;
#[doc = "Register `EVT_ST4` writer"]
pub type W = crate::W<EVT_ST4_SPEC>;
#[doc = "Field `MCPWM1_EVT_OP0_TEE2_ST` reader - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM1_EVT_OP0_TEE2_ST_R = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP0_TEE2_ST` writer - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM1_EVT_OP0_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP1_TEE2_ST` reader - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM1_EVT_OP1_TEE2_ST_R = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP1_TEE2_ST` writer - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM1_EVT_OP1_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCPWM1_EVT_OP2_TEE2_ST` reader - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM1_EVT_OP2_TEE2_ST_R = crate::BitReader;
#[doc = "Field `MCPWM1_EVT_OP2_TEE2_ST` writer - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type MCPWM1_EVT_OP2_TEE2_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_CONV_CMPLT0_ST` reader - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_CONV_CMPLT0_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_CONV_CMPLT0_ST` writer - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_CONV_CMPLT0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH0_ST` reader - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_ABOVE_THRESH0_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH0_ST` writer - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_ABOVE_THRESH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH1_ST` reader - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_ABOVE_THRESH1_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_ABOVE_THRESH1_ST` writer - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_ABOVE_THRESH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH0_ST` reader - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_BELOW_THRESH0_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH0_ST` writer - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_BELOW_THRESH0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH1_ST` reader - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_BELOW_THRESH1_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_EQ_BELOW_THRESH1_ST` writer - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_EQ_BELOW_THRESH1_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_RESULT_DONE0_ST` reader - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_RESULT_DONE0_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_RESULT_DONE0_ST` writer - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_RESULT_DONE0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ADC_EVT_STOPPED0_ST` reader - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_STOPPED0_ST_R = crate::BitReader;
#[doc = "Field `ADC_EVT_STOPPED0_ST` writer - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type ADC_EVT_STOPPED0_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `I2S2_EVT_RX_DONE_ST` reader - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_RX_DONE_ST_R = crate::BitReader;
#[doc = "Field `I2S2_EVT_RX_DONE_ST` writer - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_RX_DONE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_EVT_TX_DONE_ST` reader - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_TX_DONE_ST_R = crate::BitReader;
#[doc = "Field `I2S2_EVT_TX_DONE_ST` writer - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_TX_DONE_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_EVT_X_WORDS_RECEIVED_ST` reader - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_X_WORDS_RECEIVED_ST_R = crate::BitReader;
#[doc = "Field `I2S2_EVT_X_WORDS_RECEIVED_ST` writer - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_X_WORDS_RECEIVED_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `I2S2_EVT_X_WORDS_SENT_ST` reader - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_X_WORDS_SENT_ST_R = crate::BitReader;
#[doc = "Field `I2S2_EVT_X_WORDS_SENT_ST` writer - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
pub type I2S2_EVT_X_WORDS_SENT_ST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op0_tee2_st(&self) -> MCPWM1_EVT_OP0_TEE2_ST_R {
        MCPWM1_EVT_OP0_TEE2_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op1_tee2_st(&self) -> MCPWM1_EVT_OP1_TEE2_ST_R {
        MCPWM1_EVT_OP1_TEE2_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn mcpwm1_evt_op2_tee2_st(&self) -> MCPWM1_EVT_OP2_TEE2_ST_R {
        MCPWM1_EVT_OP2_TEE2_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_conv_cmplt0_st(&self) -> ADC_EVT_CONV_CMPLT0_ST_R {
        ADC_EVT_CONV_CMPLT0_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_above_thresh0_st(&self) -> ADC_EVT_EQ_ABOVE_THRESH0_ST_R {
        ADC_EVT_EQ_ABOVE_THRESH0_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_above_thresh1_st(&self) -> ADC_EVT_EQ_ABOVE_THRESH1_ST_R {
        ADC_EVT_EQ_ABOVE_THRESH1_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_below_thresh0_st(&self) -> ADC_EVT_EQ_BELOW_THRESH0_ST_R {
        ADC_EVT_EQ_BELOW_THRESH0_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_eq_below_thresh1_st(&self) -> ADC_EVT_EQ_BELOW_THRESH1_ST_R {
        ADC_EVT_EQ_BELOW_THRESH1_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_result_done0_st(&self) -> ADC_EVT_RESULT_DONE0_ST_R {
        ADC_EVT_RESULT_DONE0_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_stopped0_st(&self) -> ADC_EVT_STOPPED0_ST_R {
        ADC_EVT_STOPPED0_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn adc_evt_started0_st(&self) -> ADC_EVT_STARTED0_ST_R {
        ADC_EVT_STARTED0_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done0_st(&self) -> REGDMA_EVT_DONE0_ST_R {
        REGDMA_EVT_DONE0_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done1_st(&self) -> REGDMA_EVT_DONE1_ST_R {
        REGDMA_EVT_DONE1_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done2_st(&self) -> REGDMA_EVT_DONE2_ST_R {
        REGDMA_EVT_DONE2_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_done3_st(&self) -> REGDMA_EVT_DONE3_ST_R {
        REGDMA_EVT_DONE3_ST_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err0_st(&self) -> REGDMA_EVT_ERR0_ST_R {
        REGDMA_EVT_ERR0_ST_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err1_st(&self) -> REGDMA_EVT_ERR1_ST_R {
        REGDMA_EVT_ERR1_ST_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err2_st(&self) -> REGDMA_EVT_ERR2_ST_R {
        REGDMA_EVT_ERR2_ST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn regdma_evt_err3_st(&self) -> REGDMA_EVT_ERR3_ST_R {
        REGDMA_EVT_ERR3_ST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn tmpsnsr_evt_over_limit_st(&self) -> TMPSNSR_EVT_OVER_LIMIT_ST_R {
        TMPSNSR_EVT_OVER_LIMIT_ST_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_rx_done_st(&self) -> I2S0_EVT_RX_DONE_ST_R {
        I2S0_EVT_RX_DONE_ST_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_tx_done_st(&self) -> I2S0_EVT_TX_DONE_ST_R {
        I2S0_EVT_TX_DONE_ST_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_received_st(&self) -> I2S0_EVT_X_WORDS_RECEIVED_ST_R {
        I2S0_EVT_X_WORDS_RECEIVED_ST_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s0_evt_x_words_sent_st(&self) -> I2S0_EVT_X_WORDS_SENT_ST_R {
        I2S0_EVT_X_WORDS_SENT_ST_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_rx_done_st(&self) -> I2S1_EVT_RX_DONE_ST_R {
        I2S1_EVT_RX_DONE_ST_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_tx_done_st(&self) -> I2S1_EVT_TX_DONE_ST_R {
        I2S1_EVT_TX_DONE_ST_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_received_st(&self) -> I2S1_EVT_X_WORDS_RECEIVED_ST_R {
        I2S1_EVT_X_WORDS_RECEIVED_ST_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s1_evt_x_words_sent_st(&self) -> I2S1_EVT_X_WORDS_SENT_ST_R {
        I2S1_EVT_X_WORDS_SENT_ST_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_rx_done_st(&self) -> I2S2_EVT_RX_DONE_ST_R {
        I2S2_EVT_RX_DONE_ST_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_tx_done_st(&self) -> I2S2_EVT_TX_DONE_ST_R {
        I2S2_EVT_TX_DONE_ST_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_x_words_received_st(&self) -> I2S2_EVT_X_WORDS_RECEIVED_ST_R {
        I2S2_EVT_X_WORDS_RECEIVED_ST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    pub fn i2s2_evt_x_words_sent_st(&self) -> I2S2_EVT_X_WORDS_SENT_ST_R {
        I2S2_EVT_X_WORDS_SENT_ST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EVT_ST4")
            .field(
                "mcpwm1_evt_op0_tee2_st",
                &self.mcpwm1_evt_op0_tee2_st().bit(),
            )
            .field(
                "mcpwm1_evt_op1_tee2_st",
                &self.mcpwm1_evt_op1_tee2_st().bit(),
            )
            .field(
                "mcpwm1_evt_op2_tee2_st",
                &self.mcpwm1_evt_op2_tee2_st().bit(),
            )
            .field(
                "adc_evt_conv_cmplt0_st",
                &self.adc_evt_conv_cmplt0_st().bit(),
            )
            .field(
                "adc_evt_eq_above_thresh0_st",
                &self.adc_evt_eq_above_thresh0_st().bit(),
            )
            .field(
                "adc_evt_eq_above_thresh1_st",
                &self.adc_evt_eq_above_thresh1_st().bit(),
            )
            .field(
                "adc_evt_eq_below_thresh0_st",
                &self.adc_evt_eq_below_thresh0_st().bit(),
            )
            .field(
                "adc_evt_eq_below_thresh1_st",
                &self.adc_evt_eq_below_thresh1_st().bit(),
            )
            .field(
                "adc_evt_result_done0_st",
                &self.adc_evt_result_done0_st().bit(),
            )
            .field("adc_evt_stopped0_st", &self.adc_evt_stopped0_st().bit())
            .field("adc_evt_started0_st", &self.adc_evt_started0_st().bit())
            .field("regdma_evt_done0_st", &self.regdma_evt_done0_st().bit())
            .field("regdma_evt_done1_st", &self.regdma_evt_done1_st().bit())
            .field("regdma_evt_done2_st", &self.regdma_evt_done2_st().bit())
            .field("regdma_evt_done3_st", &self.regdma_evt_done3_st().bit())
            .field("regdma_evt_err0_st", &self.regdma_evt_err0_st().bit())
            .field("regdma_evt_err1_st", &self.regdma_evt_err1_st().bit())
            .field("regdma_evt_err2_st", &self.regdma_evt_err2_st().bit())
            .field("regdma_evt_err3_st", &self.regdma_evt_err3_st().bit())
            .field(
                "tmpsnsr_evt_over_limit_st",
                &self.tmpsnsr_evt_over_limit_st().bit(),
            )
            .field("i2s0_evt_rx_done_st", &self.i2s0_evt_rx_done_st().bit())
            .field("i2s0_evt_tx_done_st", &self.i2s0_evt_tx_done_st().bit())
            .field(
                "i2s0_evt_x_words_received_st",
                &self.i2s0_evt_x_words_received_st().bit(),
            )
            .field(
                "i2s0_evt_x_words_sent_st",
                &self.i2s0_evt_x_words_sent_st().bit(),
            )
            .field("i2s1_evt_rx_done_st", &self.i2s1_evt_rx_done_st().bit())
            .field("i2s1_evt_tx_done_st", &self.i2s1_evt_tx_done_st().bit())
            .field(
                "i2s1_evt_x_words_received_st",
                &self.i2s1_evt_x_words_received_st().bit(),
            )
            .field(
                "i2s1_evt_x_words_sent_st",
                &self.i2s1_evt_x_words_sent_st().bit(),
            )
            .field("i2s2_evt_rx_done_st", &self.i2s2_evt_rx_done_st().bit())
            .field("i2s2_evt_tx_done_st", &self.i2s2_evt_tx_done_st().bit())
            .field(
                "i2s2_evt_x_words_received_st",
                &self.i2s2_evt_x_words_received_st().bit(),
            )
            .field(
                "i2s2_evt_x_words_sent_st",
                &self.i2s2_evt_x_words_sent_st().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EVT_ST4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Represents MCPWM1_evt_op0_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn mcpwm1_evt_op0_tee2_st(&mut self) -> MCPWM1_EVT_OP0_TEE2_ST_W<EVT_ST4_SPEC> {
        MCPWM1_EVT_OP0_TEE2_ST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Represents MCPWM1_evt_op1_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn mcpwm1_evt_op1_tee2_st(&mut self) -> MCPWM1_EVT_OP1_TEE2_ST_W<EVT_ST4_SPEC> {
        MCPWM1_EVT_OP1_TEE2_ST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Represents MCPWM1_evt_op2_tee2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn mcpwm1_evt_op2_tee2_st(&mut self) -> MCPWM1_EVT_OP2_TEE2_ST_W<EVT_ST4_SPEC> {
        MCPWM1_EVT_OP2_TEE2_ST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Represents ADC_evt_conv_cmplt0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_conv_cmplt0_st(&mut self) -> ADC_EVT_CONV_CMPLT0_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_CONV_CMPLT0_ST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Represents ADC_evt_eq_above_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_eq_above_thresh0_st(&mut self) -> ADC_EVT_EQ_ABOVE_THRESH0_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_EQ_ABOVE_THRESH0_ST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Represents ADC_evt_eq_above_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_eq_above_thresh1_st(&mut self) -> ADC_EVT_EQ_ABOVE_THRESH1_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_EQ_ABOVE_THRESH1_ST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Represents ADC_evt_eq_below_thresh0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_eq_below_thresh0_st(&mut self) -> ADC_EVT_EQ_BELOW_THRESH0_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_EQ_BELOW_THRESH0_ST_W::new(self, 6)
    }
    #[doc = "Bit 7 - Represents ADC_evt_eq_below_thresh1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_eq_below_thresh1_st(&mut self) -> ADC_EVT_EQ_BELOW_THRESH1_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_EQ_BELOW_THRESH1_ST_W::new(self, 7)
    }
    #[doc = "Bit 8 - Represents ADC_evt_result_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_result_done0_st(&mut self) -> ADC_EVT_RESULT_DONE0_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_RESULT_DONE0_ST_W::new(self, 8)
    }
    #[doc = "Bit 9 - Represents ADC_evt_stopped0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_stopped0_st(&mut self) -> ADC_EVT_STOPPED0_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_STOPPED0_ST_W::new(self, 9)
    }
    #[doc = "Bit 10 - Represents ADC_evt_started0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn adc_evt_started0_st(&mut self) -> ADC_EVT_STARTED0_ST_W<EVT_ST4_SPEC> {
        ADC_EVT_STARTED0_ST_W::new(self, 10)
    }
    #[doc = "Bit 11 - Represents REGDMA_evt_done0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_done0_st(&mut self) -> REGDMA_EVT_DONE0_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_DONE0_ST_W::new(self, 11)
    }
    #[doc = "Bit 12 - Represents REGDMA_evt_done1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_done1_st(&mut self) -> REGDMA_EVT_DONE1_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_DONE1_ST_W::new(self, 12)
    }
    #[doc = "Bit 13 - Represents REGDMA_evt_done2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_done2_st(&mut self) -> REGDMA_EVT_DONE2_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_DONE2_ST_W::new(self, 13)
    }
    #[doc = "Bit 14 - Represents REGDMA_evt_done3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_done3_st(&mut self) -> REGDMA_EVT_DONE3_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_DONE3_ST_W::new(self, 14)
    }
    #[doc = "Bit 15 - Represents REGDMA_evt_err0 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_err0_st(&mut self) -> REGDMA_EVT_ERR0_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_ERR0_ST_W::new(self, 15)
    }
    #[doc = "Bit 16 - Represents REGDMA_evt_err1 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_err1_st(&mut self) -> REGDMA_EVT_ERR1_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_ERR1_ST_W::new(self, 16)
    }
    #[doc = "Bit 17 - Represents REGDMA_evt_err2 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_err2_st(&mut self) -> REGDMA_EVT_ERR2_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_ERR2_ST_W::new(self, 17)
    }
    #[doc = "Bit 18 - Represents REGDMA_evt_err3 trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn regdma_evt_err3_st(&mut self) -> REGDMA_EVT_ERR3_ST_W<EVT_ST4_SPEC> {
        REGDMA_EVT_ERR3_ST_W::new(self, 18)
    }
    #[doc = "Bit 19 - Represents TMPSNSR_evt_over_limit trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn tmpsnsr_evt_over_limit_st(&mut self) -> TMPSNSR_EVT_OVER_LIMIT_ST_W<EVT_ST4_SPEC> {
        TMPSNSR_EVT_OVER_LIMIT_ST_W::new(self, 19)
    }
    #[doc = "Bit 20 - Represents I2S0_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_evt_rx_done_st(&mut self) -> I2S0_EVT_RX_DONE_ST_W<EVT_ST4_SPEC> {
        I2S0_EVT_RX_DONE_ST_W::new(self, 20)
    }
    #[doc = "Bit 21 - Represents I2S0_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_evt_tx_done_st(&mut self) -> I2S0_EVT_TX_DONE_ST_W<EVT_ST4_SPEC> {
        I2S0_EVT_TX_DONE_ST_W::new(self, 21)
    }
    #[doc = "Bit 22 - Represents I2S0_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_evt_x_words_received_st(&mut self) -> I2S0_EVT_X_WORDS_RECEIVED_ST_W<EVT_ST4_SPEC> {
        I2S0_EVT_X_WORDS_RECEIVED_ST_W::new(self, 22)
    }
    #[doc = "Bit 23 - Represents I2S0_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s0_evt_x_words_sent_st(&mut self) -> I2S0_EVT_X_WORDS_SENT_ST_W<EVT_ST4_SPEC> {
        I2S0_EVT_X_WORDS_SENT_ST_W::new(self, 23)
    }
    #[doc = "Bit 24 - Represents I2S1_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_evt_rx_done_st(&mut self) -> I2S1_EVT_RX_DONE_ST_W<EVT_ST4_SPEC> {
        I2S1_EVT_RX_DONE_ST_W::new(self, 24)
    }
    #[doc = "Bit 25 - Represents I2S1_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_evt_tx_done_st(&mut self) -> I2S1_EVT_TX_DONE_ST_W<EVT_ST4_SPEC> {
        I2S1_EVT_TX_DONE_ST_W::new(self, 25)
    }
    #[doc = "Bit 26 - Represents I2S1_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_evt_x_words_received_st(&mut self) -> I2S1_EVT_X_WORDS_RECEIVED_ST_W<EVT_ST4_SPEC> {
        I2S1_EVT_X_WORDS_RECEIVED_ST_W::new(self, 26)
    }
    #[doc = "Bit 27 - Represents I2S1_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s1_evt_x_words_sent_st(&mut self) -> I2S1_EVT_X_WORDS_SENT_ST_W<EVT_ST4_SPEC> {
        I2S1_EVT_X_WORDS_SENT_ST_W::new(self, 27)
    }
    #[doc = "Bit 28 - Represents I2S2_evt_rx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_evt_rx_done_st(&mut self) -> I2S2_EVT_RX_DONE_ST_W<EVT_ST4_SPEC> {
        I2S2_EVT_RX_DONE_ST_W::new(self, 28)
    }
    #[doc = "Bit 29 - Represents I2S2_evt_tx_done trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_evt_tx_done_st(&mut self) -> I2S2_EVT_TX_DONE_ST_W<EVT_ST4_SPEC> {
        I2S2_EVT_TX_DONE_ST_W::new(self, 29)
    }
    #[doc = "Bit 30 - Represents I2S2_evt_x_words_received trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_evt_x_words_received_st(&mut self) -> I2S2_EVT_X_WORDS_RECEIVED_ST_W<EVT_ST4_SPEC> {
        I2S2_EVT_X_WORDS_RECEIVED_ST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Represents I2S2_evt_x_words_sent trigger status.\\\\0: Not triggered\\\\1: Triggered"]
    #[inline(always)]
    #[must_use]
    pub fn i2s2_evt_x_words_sent_st(&mut self) -> I2S2_EVT_X_WORDS_SENT_ST_W<EVT_ST4_SPEC> {
        I2S2_EVT_X_WORDS_SENT_ST_W::new(self, 31)
    }
}
#[doc = "Events trigger status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`evt_st4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`evt_st4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EVT_ST4_SPEC;
impl crate::RegisterSpec for EVT_ST4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`evt_st4::R`](R) reader structure"]
impl crate::Readable for EVT_ST4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`evt_st4::W`](W) writer structure"]
impl crate::Writable for EVT_ST4_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EVT_ST4 to value 0"]
impl crate::Resettable for EVT_ST4_SPEC {
    const RESET_VALUE: u32 = 0;
}
