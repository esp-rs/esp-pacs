#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_TX_END_INT_CLR[0-7]` writer - Set this bit to clear the rmt_ch%s_rx_end_int_raw.."]
pub type CH_TX_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_RX_END_INT_CLR[0-7]` writer - Set this bit to clear the rmt_ch%s_tx_end_int_raw."]
pub type CH_RX_END_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_ERR_INT_CLR[0-7]` writer - Set this bit to clear the rmt_ch%s_err_int_raw."]
pub type CH_ERR_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_TX_THR_EVENT_INT_CLR[0-7]` writer - Set this bit to clear the rmt_ch%s_tx_thr_event_int_raw interrupt."]
pub type CH_TX_THR_EVENT_INT_CLR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set this bit to clear the rmt_ch[0-7]_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end_int_clr<const O: u8>(&mut self) -> CH_TX_END_INT_CLR_W<O> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 0 - Set this bit to clear the rmt_ch0_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<0> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear the rmt_ch1_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<3> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear the rmt_ch2_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<6> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear the rmt_ch3_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<9> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear the rmt_ch4_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<12> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to clear the rmt_ch5_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<15> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to clear the rmt_ch6_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<18> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to clear the rmt_ch7_rx_end_int_raw.."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_tx_end_int_clr(&mut self) -> CH_TX_END_INT_CLR_W<21> {
        CH_TX_END_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear the rmt_ch[0-7]_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end_int_clr<const O: u8>(&mut self) -> CH_RX_END_INT_CLR_W<O> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear the rmt_ch0_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<1> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear the rmt_ch1_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<4> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear the rmt_ch2_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<7> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear the rmt_ch3_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<10> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear the rmt_ch4_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<13> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to clear the rmt_ch5_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<16> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to clear the rmt_ch6_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<19> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to clear the rmt_ch7_tx_end_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end_int_clr(&mut self) -> CH_RX_END_INT_CLR_W<22> {
        CH_RX_END_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear the rmt_ch[0-7]_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_err_int_clr<const O: u8>(&mut self) -> CH_ERR_INT_CLR_W<O> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear the rmt_ch0_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<2> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear the rmt_ch1_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<5> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear the rmt_ch2_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<8> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear the rmt_ch3_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<11> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to clear the rmt_ch4_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<14> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to clear the rmt_ch5_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<17> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to clear the rmt_ch6_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<20> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to clear the rmt_ch7_err_int_raw."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_err_int_clr(&mut self) -> CH_ERR_INT_CLR_W<23> {
        CH_ERR_INT_CLR_W::new(self)
    }
    #[doc = "Set this bit to clear the rmt_ch[0-7]_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event_int_clr<const O: u8>(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<O> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to clear the rmt_ch0_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<24> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 25 - Set this bit to clear the rmt_ch1_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<25> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 26 - Set this bit to clear the rmt_ch2_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<26> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 27 - Set this bit to clear the rmt_ch3_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<27> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 28 - Set this bit to clear the rmt_ch4_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<28> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 29 - Set this bit to clear the rmt_ch5_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<29> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 30 - Set this bit to clear the rmt_ch6_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<30> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Bit 31 - Set this bit to clear the rmt_ch7_tx_thr_event_int_raw interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_tx_thr_event_int_clr(&mut self) -> CH_TX_THR_EVENT_INT_CLR_W<31> {
        CH_TX_THR_EVENT_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
