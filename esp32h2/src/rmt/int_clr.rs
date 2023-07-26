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
#[doc = "Field `CH_TX_END[0-1]` writer - Set this bit to clear theCH%s_TX_END_INT interrupt."]
pub type CH_TX_END_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_RX_END[2-3]` writer - Set this bit to clear theCH2_RX_END_INT interrupt."]
pub type CH_RX_END_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_TX_ERR[0-1]` writer - Set this bit to clear theCH4_ERR_INT interrupt."]
pub type CH_TX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_RX_ERR[2-3]` writer - Set this bit to clear theCH6_ERR_INT interrupt."]
pub type CH_RX_ERR_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_TX_THR_EVENT[0-1]` writer - Set this bit to clear theCH%s_TX_THR_EVENT_INT interrupt."]
pub type CH_TX_THR_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_RX_THR_EVENT[2-3]` writer - Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt."]
pub type CH_RX_THR_EVENT_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[doc = "Field `CH_TX_LOOP[0-1]` writer - Set this bit to clear theCH%s_TX_LOOP_INT interrupt."]
pub type CH_TX_LOOP_W<'a, const O: u8> = crate::BitWriter<'a, INT_CLR_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set this bit to clear theCH[0-1]_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end<const O: u8>(&mut self) -> CH_TX_END_W<O> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 0 - Set this bit to clear theCH0_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<0> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear theCH1_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<1> {
        CH_TX_END_W::new(self)
    }
    #[doc = "Set this bit to clear theCH2_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end<const O: u8>(&mut self) -> CH_RX_END_W<O> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear theCH2_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<2> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear theCH2_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<3> {
        CH_RX_END_W::new(self)
    }
    #[doc = "Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err<const O: u8>(&mut self) -> CH_TX_ERR_W<O> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<4> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<5> {
        CH_TX_ERR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH6_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err<const O: u8>(&mut self) -> CH_RX_ERR_W<O> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear theCH6_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_err(&mut self) -> CH_RX_ERR_W<6> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear theCH6_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_err(&mut self) -> CH_RX_ERR_W<7> {
        CH_RX_ERR_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-1]_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event<const O: u8>(&mut self) -> CH_TX_THR_EVENT_W<O> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<8> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<9> {
        CH_TX_THR_EVENT_W::new(self)
    }
    #[doc = "Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_thr_event<const O: u8>(&mut self) -> CH_RX_THR_EVENT_W<O> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 10 - Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<10> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Bit 11 - Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<11> {
        CH_RX_THR_EVENT_W::new(self)
    }
    #[doc = "Set this bit to clear theCH[0-1]_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop<const O: u8>(&mut self) -> CH_TX_LOOP_W<O> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to clear theCH0_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<12> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to clear theCH1_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<13> {
        CH_TX_LOOP_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt clear bits\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
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
