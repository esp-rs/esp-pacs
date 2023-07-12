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
#[doc = "Register `INT_RAW` writer"]
pub struct W(crate::W<INT_RAW_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_RAW_SPEC>;
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
impl From<crate::W<INT_RAW_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_RAW_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
pub type CH0_TX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH0_TX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
pub type CH0_TX_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH1_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
pub type CH1_TX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH1_TX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
pub type CH1_TX_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH2_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
pub type CH2_RX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH2_RX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
pub type CH2_RX_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH3_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when reception done."]
pub type CH3_RX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH3_RX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when reception done."]
pub type CH3_RX_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH0_TX_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH0_TX_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH0_TX_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH0_TX_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH1_TX_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
pub type CH1_TX_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH1_TX_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
pub type CH1_TX_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH2_TX_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type CH2_TX_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH2_TX_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type CH2_TX_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH3_TX_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
pub type CH3_TX_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH3_TX_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
pub type CH3_TX_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
pub type CH0_TX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
pub type CH0_TX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
pub type CH1_TX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
pub type CH1_TX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH2_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
pub type CH2_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH2_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
pub type CH2_RX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH3_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when receiver receive more data than configured value."]
pub type CH3_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH3_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when receiver receive more data than configured value."]
pub type CH3_RX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH0_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
pub type CH0_TX_LOOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH0_TX_LOOP_INT_RAW` writer - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
pub type CH0_TX_LOOP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH1_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
pub type CH1_TX_LOOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH1_TX_LOOP_INT_RAW` writer - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
pub type CH1_TX_LOOP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH0_TX_END_INT_RAW_R {
        CH0_TX_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH1_TX_END_INT_RAW_R {
        CH1_TX_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    pub fn ch2_rx_end_int_raw(&self) -> CH2_RX_END_INT_RAW_R {
        CH2_RX_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when reception done."]
    #[inline(always)]
    pub fn ch3_rx_end_int_raw(&self) -> CH3_RX_END_INT_RAW_R {
        CH3_RX_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_tx_err_int_raw(&self) -> CH0_TX_ERR_INT_RAW_R {
        CH0_TX_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_tx_err_int_raw(&self) -> CH1_TX_ERR_INT_RAW_R {
        CH1_TX_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_tx_err_int_raw(&self) -> CH2_TX_ERR_INT_RAW_R {
        CH2_TX_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_tx_err_int_raw(&self) -> CH3_TX_ERR_INT_RAW_R {
        CH3_TX_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH0_TX_THR_EVENT_INT_RAW_R {
        CH0_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH1_TX_THR_EVENT_INT_RAW_R {
        CH1_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_raw(&self) -> CH2_RX_THR_EVENT_INT_RAW_R {
        CH2_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_raw(&self) -> CH3_RX_THR_EVENT_INT_RAW_R {
        CH3_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> CH0_TX_LOOP_INT_RAW_R {
        CH0_TX_LOOP_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> CH1_TX_LOOP_INT_RAW_R {
        CH1_TX_LOOP_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "ch0_tx_end_int_raw",
                &format_args!("{}", self.ch0_tx_end_int_raw().bit()),
            )
            .field(
                "ch1_tx_end_int_raw",
                &format_args!("{}", self.ch1_tx_end_int_raw().bit()),
            )
            .field(
                "ch2_rx_end_int_raw",
                &format_args!("{}", self.ch2_rx_end_int_raw().bit()),
            )
            .field(
                "ch3_rx_end_int_raw",
                &format_args!("{}", self.ch3_rx_end_int_raw().bit()),
            )
            .field(
                "ch0_tx_err_int_raw",
                &format_args!("{}", self.ch0_tx_err_int_raw().bit()),
            )
            .field(
                "ch1_tx_err_int_raw",
                &format_args!("{}", self.ch1_tx_err_int_raw().bit()),
            )
            .field(
                "ch2_tx_err_int_raw",
                &format_args!("{}", self.ch2_tx_err_int_raw().bit()),
            )
            .field(
                "ch3_tx_err_int_raw",
                &format_args!("{}", self.ch3_tx_err_int_raw().bit()),
            )
            .field(
                "ch0_tx_thr_event_int_raw",
                &format_args!("{}", self.ch0_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch1_tx_thr_event_int_raw",
                &format_args!("{}", self.ch1_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch2_rx_thr_event_int_raw",
                &format_args!("{}", self.ch2_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch3_rx_thr_event_int_raw",
                &format_args!("{}", self.ch3_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch0_tx_loop_int_raw",
                &format_args!("{}", self.ch0_tx_loop_int_raw().bit()),
            )
            .field(
                "ch1_tx_loop_int_raw",
                &format_args!("{}", self.ch1_tx_loop_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end_int_raw(&mut self) -> CH0_TX_END_INT_RAW_W<0> {
        CH0_TX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end_int_raw(&mut self) -> CH1_TX_END_INT_RAW_W<1> {
        CH1_TX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end_int_raw(&mut self) -> CH2_RX_END_INT_RAW_W<2> {
        CH2_RX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end_int_raw(&mut self) -> CH3_RX_END_INT_RAW_W<3> {
        CH3_RX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err_int_raw(&mut self) -> CH0_TX_ERR_INT_RAW_W<4> {
        CH0_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err_int_raw(&mut self) -> CH1_TX_ERR_INT_RAW_W<5> {
        CH1_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err_int_raw(&mut self) -> CH2_TX_ERR_INT_RAW_W<6> {
        CH2_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err_int_raw(&mut self) -> CH3_TX_ERR_INT_RAW_W<7> {
        CH3_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event_int_raw(&mut self) -> CH0_TX_THR_EVENT_INT_RAW_W<8> {
        CH0_TX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event_int_raw(&mut self) -> CH1_TX_THR_EVENT_INT_RAW_W<9> {
        CH1_TX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_thr_event_int_raw(&mut self) -> CH2_RX_THR_EVENT_INT_RAW_W<10> {
        CH2_RX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_thr_event_int_raw(&mut self) -> CH3_RX_THR_EVENT_INT_RAW_W<11> {
        CH3_RX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop_int_raw(&mut self) -> CH0_TX_LOOP_INT_RAW_W<12> {
        CH0_TX_LOOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop_int_raw(&mut self) -> CH1_TX_LOOP_INT_RAW_W<13> {
        CH1_TX_LOOP_INT_RAW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_raw::W](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
