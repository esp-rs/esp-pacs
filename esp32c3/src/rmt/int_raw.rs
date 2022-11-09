#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH_TX_END_INT_RAW[0-1]` reader - reg_ch%s_tx_end_int_raw."]
pub type CH_TX_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_RX_END_INT_RAW[2-3]` reader - reg_ch2_rx_end_int_raw."]
pub type CH_RX_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_ERR_INT_RAW[0-1]` reader - reg_ch%s_err_int_raw."]
pub type CH_TX_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_RX_ERR_INT_RAW[2-3]` reader - reg_ch2_err_int_raw."]
pub type CH_RX_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_THR_EVENT_INT_RAW[0-1]` reader - reg_ch%s_tx_thr_event_int_raw."]
pub type CH_TX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH2_RX_THR_EVENT_INT_RAW` reader - reg_ch2_rx_thr_event_int_raw."]
pub type CH2_RX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH3_RX_THR_EVENT_INT_RAW` reader - reg_ch3_rx_thr_event_int_raw."]
pub type CH3_RX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_LOOP_INT_RAW[0-1]` reader - reg_ch%s_tx_loop_int_raw."]
pub type CH_TX_LOOP_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "reg_ch[0-1]_tx_end_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_raw(&self, n: u8) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_raw(&self, n: u8) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> (n - 2 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    pub fn ch2_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_raw."]
    #[inline(always)]
    pub fn ch3_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_err_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_err_int_raw(&self, n: u8) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_ch0_err_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "reg_ch2_err_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_raw(&self, n: u8) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> (n - 2 + 6)) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_ch2_err_int_raw."]
    #[inline(always)]
    pub fn ch2_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_raw."]
    #[inline(always)]
    pub fn ch3_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_tx_thr_event_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_raw(&self, n: u8) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_raw(&self) -> CH2_RX_THR_EVENT_INT_RAW_R {
        CH2_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ch3_rx_thr_event_int_raw."]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_raw(&self) -> CH3_RX_THR_EVENT_INT_RAW_R {
        CH3_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_tx_loop_int_raw."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_raw(&self, n: u8) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_raw."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_raw."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[doc = "RMT_INT_RAW_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
