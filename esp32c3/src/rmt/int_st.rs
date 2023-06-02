#[doc = "Register `INT_ST` reader"]
pub struct R(crate::R<INT_ST_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ST_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ST_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ST_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH_TX_END_INT_ST[0-1]` reader - reg_ch%s_tx_end_int_st."]
pub type CH_TX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH_RX_END_INT_ST[2-3]` reader - reg_ch2_rx_end_int_st."]
pub type CH_RX_END_INT_ST_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR_INT_ST[0-1]` reader - reg_ch%s_err_int_st."]
pub type CH_TX_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR_INT_ST[2-3]` reader - reg_ch2_err_int_st."]
pub type CH_RX_ERR_INT_ST_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT_INT_ST[0-1]` reader - reg_ch%s_tx_thr_event_int_st."]
pub type CH_TX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH2_RX_THR_EVENT_INT_ST` reader - reg_ch2_rx_thr_event_int_st."]
pub type CH2_RX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH3_RX_THR_EVENT_INT_ST` reader - reg_ch3_rx_thr_event_int_st."]
pub type CH3_RX_THR_EVENT_INT_ST_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP_INT_ST[0-1]` reader - reg_ch%s_tx_loop_int_st."]
pub type CH_TX_LOOP_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "reg_ch[0-1]_tx_end_int_st."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_st(&self, n: u8) -> CH_TX_END_INT_ST_R {
        CH_TX_END_INT_ST_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_st."]
    #[inline(always)]
    pub fn ch0_tx_end_int_st(&self) -> CH_TX_END_INT_ST_R {
        CH_TX_END_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_st."]
    #[inline(always)]
    pub fn ch1_tx_end_int_st(&self) -> CH_TX_END_INT_ST_R {
        CH_TX_END_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "reg_ch2_rx_end_int_st."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_st(&self, n: u8) -> CH_RX_END_INT_ST_R {
        CH_RX_END_INT_ST_R::new(((self.bits >> (n - 2 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_st."]
    #[inline(always)]
    pub fn ch2_rx_end_int_st(&self) -> CH_RX_END_INT_ST_R {
        CH_RX_END_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_st."]
    #[inline(always)]
    pub fn ch3_rx_end_int_st(&self) -> CH_RX_END_INT_ST_R {
        CH_RX_END_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_err_int_st."]
    #[inline(always)]
    pub unsafe fn ch_tx_err_int_st(&self, n: u8) -> CH_TX_ERR_INT_ST_R {
        CH_TX_ERR_INT_ST_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - reg_ch0_err_int_st."]
    #[inline(always)]
    pub fn ch0_tx_err_int_st(&self) -> CH_TX_ERR_INT_ST_R {
        CH_TX_ERR_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_st."]
    #[inline(always)]
    pub fn ch1_tx_err_int_st(&self) -> CH_TX_ERR_INT_ST_R {
        CH_TX_ERR_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "reg_ch2_err_int_st."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_st(&self, n: u8) -> CH_RX_ERR_INT_ST_R {
        CH_RX_ERR_INT_ST_R::new(((self.bits >> (n - 2 + 6)) & 1) != 0)
    }
    #[doc = "Bit 6 - reg_ch2_err_int_st."]
    #[inline(always)]
    pub fn ch2_rx_err_int_st(&self) -> CH_RX_ERR_INT_ST_R {
        CH_RX_ERR_INT_ST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_st."]
    #[inline(always)]
    pub fn ch3_rx_err_int_st(&self) -> CH_RX_ERR_INT_ST_R {
        CH_RX_ERR_INT_ST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_tx_thr_event_int_st."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_st(&self, n: u8) -> CH_TX_THR_EVENT_INT_ST_R {
        CH_TX_THR_EVENT_INT_ST_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_st(&self) -> CH_TX_THR_EVENT_INT_ST_R {
        CH_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_st(&self) -> CH_TX_THR_EVENT_INT_ST_R {
        CH_TX_THR_EVENT_INT_ST_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_st(&self) -> CH2_RX_THR_EVENT_INT_ST_R {
        CH2_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ch3_rx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_st(&self) -> CH3_RX_THR_EVENT_INT_ST_R {
        CH3_RX_THR_EVENT_INT_ST_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "reg_ch[0-1]_tx_loop_int_st."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_st(&self, n: u8) -> CH_TX_LOOP_INT_ST_R {
        CH_TX_LOOP_INT_ST_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_st."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_st(&self) -> CH_TX_LOOP_INT_ST_R {
        CH_TX_LOOP_INT_ST_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_st."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_st(&self) -> CH_TX_LOOP_INT_ST_R {
        CH_TX_LOOP_INT_ST_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "ch0_tx_end_int_st",
                &format_args!("{}", self.ch0_tx_end_int_st().bit()),
            )
            .field(
                "ch1_tx_end_int_st",
                &format_args!("{}", self.ch1_tx_end_int_st().bit()),
            )
            .field(
                "ch2_rx_end_int_st",
                &format_args!("{}", self.ch2_rx_end_int_st().bit()),
            )
            .field(
                "ch3_rx_end_int_st",
                &format_args!("{}", self.ch3_rx_end_int_st().bit()),
            )
            .field(
                "ch0_tx_err_int_st",
                &format_args!("{}", self.ch0_tx_err_int_st().bit()),
            )
            .field(
                "ch1_tx_err_int_st",
                &format_args!("{}", self.ch1_tx_err_int_st().bit()),
            )
            .field(
                "ch2_rx_err_int_st",
                &format_args!("{}", self.ch2_rx_err_int_st().bit()),
            )
            .field(
                "ch3_rx_err_int_st",
                &format_args!("{}", self.ch3_rx_err_int_st().bit()),
            )
            .field(
                "ch0_tx_thr_event_int_st",
                &format_args!("{}", self.ch0_tx_thr_event_int_st().bit()),
            )
            .field(
                "ch1_tx_thr_event_int_st",
                &format_args!("{}", self.ch1_tx_thr_event_int_st().bit()),
            )
            .field(
                "ch2_rx_thr_event_int_st",
                &format_args!("{}", self.ch2_rx_thr_event_int_st().bit()),
            )
            .field(
                "ch3_rx_thr_event_int_st",
                &format_args!("{}", self.ch3_rx_thr_event_int_st().bit()),
            )
            .field(
                "ch0_tx_loop_int_st",
                &format_args!("{}", self.ch0_tx_loop_int_st().bit()),
            )
            .field(
                "ch1_tx_loop_int_st",
                &format_args!("{}", self.ch1_tx_loop_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "RMT_INT_ST_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_st](index.html) module"]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_st::R](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
