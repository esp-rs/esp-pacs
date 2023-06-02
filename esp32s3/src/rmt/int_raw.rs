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
#[doc = "Field `CH_TX_END_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmission done."]
pub type CH_TX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH_TX_END_INT_RAW[0-3]` writer - The interrupt raw bit for CHANNEL%s. Triggered when transmission done."]
pub type CH_TX_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH_TX_ERR_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when error occurs."]
pub type CH_TX_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR_INT_RAW[0-3]` writer - The interrupt raw bit for CHANNEL%s. Triggered when error occurs."]
pub type CH_TX_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH_TX_THR_EVENT_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value."]
pub type CH_TX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT_INT_RAW[0-3]` writer - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value."]
pub type CH_TX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH_TX_LOOP_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value."]
pub type CH_TX_LOOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP_INT_RAW[0-3]` writer - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value."]
pub type CH_TX_LOOP_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH_RX_END_INT_RAW[4-7]` reader - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type CH_RX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH_RX_END_INT_RAW[4-7]` writer - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type CH_RX_END_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH_RX_ERR_INT_RAW[4-7]` reader - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH_RX_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR_INT_RAW[4-7]` writer - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH_RX_ERR_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH4_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type CH4_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type CH4_RX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH5_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
pub type CH5_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
pub type CH5_RX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH6_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
pub type CH6_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
pub type CH6_RX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `CH7_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
pub type CH7_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
pub type CH7_RX_THR_EVENT_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_RAW` writer - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_RAW_W<'a, const O: u8> = crate::BitWriter<'a, INT_RAW_SPEC, O>;
impl R {
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when transmission done."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_raw(&self, n: u8) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch2_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch3_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when error occurs."]
    #[inline(always)]
    pub unsafe fn ch_tx_err_int_raw(&self, n: u8) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_raw(&self, n: u8) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_raw(&self, n: u8) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_raw(&self, n: u8) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> (n - 4 + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch4_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch5_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch6_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch7_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_raw(&self, n: u8) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> (n - 4 + 20)) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch4_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch5_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch6_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch7_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_raw(&self) -> CH4_RX_THR_EVENT_INT_RAW_R {
        CH4_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_raw(&self) -> CH5_RX_THR_EVENT_INT_RAW_R {
        CH5_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_raw(&self) -> CH6_RX_THR_EVENT_INT_RAW_R {
        CH6_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_raw(&self) -> CH7_RX_THR_EVENT_INT_RAW_R {
        CH7_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_raw(&self) -> TX_CH3_DMA_ACCESS_FAIL_INT_RAW_R {
        TX_CH3_DMA_ACCESS_FAIL_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_raw(&self) -> RX_CH7_DMA_ACCESS_FAIL_INT_RAW_R {
        RX_CH7_DMA_ACCESS_FAIL_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
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
                "ch2_tx_end_int_raw",
                &format_args!("{}", self.ch2_tx_end_int_raw().bit()),
            )
            .field(
                "ch3_tx_end_int_raw",
                &format_args!("{}", self.ch3_tx_end_int_raw().bit()),
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
                "ch2_tx_thr_event_int_raw",
                &format_args!("{}", self.ch2_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch3_tx_thr_event_int_raw",
                &format_args!("{}", self.ch3_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch0_tx_loop_int_raw",
                &format_args!("{}", self.ch0_tx_loop_int_raw().bit()),
            )
            .field(
                "ch1_tx_loop_int_raw",
                &format_args!("{}", self.ch1_tx_loop_int_raw().bit()),
            )
            .field(
                "ch2_tx_loop_int_raw",
                &format_args!("{}", self.ch2_tx_loop_int_raw().bit()),
            )
            .field(
                "ch3_tx_loop_int_raw",
                &format_args!("{}", self.ch3_tx_loop_int_raw().bit()),
            )
            .field(
                "ch4_rx_end_int_raw",
                &format_args!("{}", self.ch4_rx_end_int_raw().bit()),
            )
            .field(
                "ch5_rx_end_int_raw",
                &format_args!("{}", self.ch5_rx_end_int_raw().bit()),
            )
            .field(
                "ch6_rx_end_int_raw",
                &format_args!("{}", self.ch6_rx_end_int_raw().bit()),
            )
            .field(
                "ch7_rx_end_int_raw",
                &format_args!("{}", self.ch7_rx_end_int_raw().bit()),
            )
            .field(
                "ch4_rx_err_int_raw",
                &format_args!("{}", self.ch4_rx_err_int_raw().bit()),
            )
            .field(
                "ch5_rx_err_int_raw",
                &format_args!("{}", self.ch5_rx_err_int_raw().bit()),
            )
            .field(
                "ch6_rx_err_int_raw",
                &format_args!("{}", self.ch6_rx_err_int_raw().bit()),
            )
            .field(
                "ch7_rx_err_int_raw",
                &format_args!("{}", self.ch7_rx_err_int_raw().bit()),
            )
            .field(
                "ch4_rx_thr_event_int_raw",
                &format_args!("{}", self.ch4_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch5_rx_thr_event_int_raw",
                &format_args!("{}", self.ch5_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch6_rx_thr_event_int_raw",
                &format_args!("{}", self.ch6_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch7_rx_thr_event_int_raw",
                &format_args!("{}", self.ch7_rx_thr_event_int_raw().bit()),
            )
            .field(
                "tx_ch3_dma_access_fail_int_raw",
                &format_args!("{}", self.tx_ch3_dma_access_fail_int_raw().bit()),
            )
            .field(
                "rx_ch7_dma_access_fail_int_raw",
                &format_args!("{}", self.rx_ch7_dma_access_fail_int_raw().bit()),
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
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end_int_raw<const O: u8>(&mut self) -> CH_TX_END_INT_RAW_W<O> {
        CH_TX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end_int_raw(&mut self) -> CH_TX_END_INT_RAW_W<0> {
        CH_TX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end_int_raw(&mut self) -> CH_TX_END_INT_RAW_W<1> {
        CH_TX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end_int_raw(&mut self) -> CH_TX_END_INT_RAW_W<2> {
        CH_TX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end_int_raw(&mut self) -> CH_TX_END_INT_RAW_W<3> {
        CH_TX_END_INT_RAW_W::new(self)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err_int_raw<const O: u8>(&mut self) -> CH_TX_ERR_INT_RAW_W<O> {
        CH_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err_int_raw(&mut self) -> CH_TX_ERR_INT_RAW_W<4> {
        CH_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err_int_raw(&mut self) -> CH_TX_ERR_INT_RAW_W<5> {
        CH_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err_int_raw(&mut self) -> CH_TX_ERR_INT_RAW_W<6> {
        CH_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err_int_raw(&mut self) -> CH_TX_ERR_INT_RAW_W<7> {
        CH_TX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event_int_raw<const O: u8>(&mut self) -> CH_TX_THR_EVENT_INT_RAW_W<O> {
        CH_TX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event_int_raw(&mut self) -> CH_TX_THR_EVENT_INT_RAW_W<8> {
        CH_TX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event_int_raw(&mut self) -> CH_TX_THR_EVENT_INT_RAW_W<9> {
        CH_TX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event_int_raw(&mut self) -> CH_TX_THR_EVENT_INT_RAW_W<10> {
        CH_TX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event_int_raw(&mut self) -> CH_TX_THR_EVENT_INT_RAW_W<11> {
        CH_TX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop_int_raw<const O: u8>(&mut self) -> CH_TX_LOOP_INT_RAW_W<O> {
        CH_TX_LOOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop_int_raw(&mut self) -> CH_TX_LOOP_INT_RAW_W<12> {
        CH_TX_LOOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop_int_raw(&mut self) -> CH_TX_LOOP_INT_RAW_W<13> {
        CH_TX_LOOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop_int_raw(&mut self) -> CH_TX_LOOP_INT_RAW_W<14> {
        CH_TX_LOOP_INT_RAW_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop_int_raw(&mut self) -> CH_TX_LOOP_INT_RAW_W<15> {
        CH_TX_LOOP_INT_RAW_W::new(self)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end_int_raw<const O: u8>(&mut self) -> CH_RX_END_INT_RAW_W<O> {
        CH_RX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end_int_raw(&mut self) -> CH_RX_END_INT_RAW_W<16> {
        CH_RX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end_int_raw(&mut self) -> CH_RX_END_INT_RAW_W<17> {
        CH_RX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end_int_raw(&mut self) -> CH_RX_END_INT_RAW_W<18> {
        CH_RX_END_INT_RAW_W::new(self)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end_int_raw(&mut self) -> CH_RX_END_INT_RAW_W<19> {
        CH_RX_END_INT_RAW_W::new(self)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err_int_raw<const O: u8>(&mut self) -> CH_RX_ERR_INT_RAW_W<O> {
        CH_RX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_err_int_raw(&mut self) -> CH_RX_ERR_INT_RAW_W<20> {
        CH_RX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_err_int_raw(&mut self) -> CH_RX_ERR_INT_RAW_W<21> {
        CH_RX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_err_int_raw(&mut self) -> CH_RX_ERR_INT_RAW_W<22> {
        CH_RX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_err_int_raw(&mut self) -> CH_RX_ERR_INT_RAW_W<23> {
        CH_RX_ERR_INT_RAW_W::new(self)
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event_int_raw(&mut self) -> CH4_RX_THR_EVENT_INT_RAW_W<24> {
        CH4_RX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event_int_raw(&mut self) -> CH5_RX_THR_EVENT_INT_RAW_W<25> {
        CH5_RX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event_int_raw(&mut self) -> CH6_RX_THR_EVENT_INT_RAW_W<26> {
        CH6_RX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event_int_raw(&mut self) -> CH7_RX_THR_EVENT_INT_RAW_W<27> {
        CH7_RX_THR_EVENT_INT_RAW_W::new(self)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail_int_raw(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_INT_RAW_W<28> {
        TX_CH3_DMA_ACCESS_FAIL_INT_RAW_W::new(self)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail_int_raw(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_INT_RAW_W<29> {
        RX_CH7_DMA_ACCESS_FAIL_INT_RAW_W::new(self)
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
