#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_TX_END_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_TX_END_INT."]
pub type CH_TX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH_TX_END_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_TX_END_INT."]
pub type CH_TX_END_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_TX_ERR_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_TX_THR_EVENT_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_TX_LOOP_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_RX_END_INT_ENA[4-7]` reader - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH_RX_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH_RX_END_INT_ENA[4-7]` writer - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH_RX_END_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH_RX_ERR_INT_ENA[4-7]` reader - The interrupt enable bit for CH4_ERR_INT."]
pub type CH_RX_ERR_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR_INT_ENA[4-7]` writer - The interrupt enable bit for CH4_ERR_INT."]
pub type CH_RX_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH4_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH4_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH4_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH5_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
pub type CH5_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
pub type CH5_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH6_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
pub type CH6_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
pub type CH6_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CH7_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
pub type CH7_RX_THR_EVENT_INT_ENA_R = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
pub type CH7_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_ENA` reader - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_ENA` writer - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_ENA` reader - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_ENA_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_ENA` writer - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "The interrupt enable bit for CH[0-3]_TX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_ena(&self, n: u8) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&self) -> CH_TX_END_INT_ENA_R {
        CH_TX_END_INT_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_err_int_ena(&self, n: u8) -> CH_TX_ERR_INT_ENA_R {
        CH_TX_ERR_INT_ENA_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_tx_err_int_ena(&self) -> CH_TX_ERR_INT_ENA_R {
        CH_TX_ERR_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_tx_err_int_ena(&self) -> CH_TX_ERR_INT_ENA_R {
        CH_TX_ERR_INT_ENA_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_tx_err_int_ena(&self) -> CH_TX_ERR_INT_ENA_R {
        CH_TX_ERR_INT_ENA_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_tx_err_int_ena(&self) -> CH_TX_ERR_INT_ENA_R {
        CH_TX_ERR_INT_ENA_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_ena(&self, n: u8) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&self) -> CH_TX_THR_EVENT_INT_ENA_R {
        CH_TX_THR_EVENT_INT_ENA_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_LOOP_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_ena(&self, n: u8) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&self) -> CH_TX_LOOP_INT_ENA_R {
        CH_TX_LOOP_INT_ENA_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_ena(&self, n: u8) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> (n - 4 + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_ena(&self, n: u8) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> (n - 4 + 20)) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch5_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch6_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch7_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_ena(&self) -> CH4_RX_THR_EVENT_INT_ENA_R {
        CH4_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_ena(&self) -> CH5_RX_THR_EVENT_INT_ENA_R {
        CH5_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_ena(&self) -> CH6_RX_THR_EVENT_INT_ENA_R {
        CH6_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_ena(&self) -> CH7_RX_THR_EVENT_INT_ENA_R {
        CH7_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_ena(&self) -> TX_CH3_DMA_ACCESS_FAIL_INT_ENA_R {
        TX_CH3_DMA_ACCESS_FAIL_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_ena(&self) -> RX_CH7_DMA_ACCESS_FAIL_INT_ENA_R {
        RX_CH7_DMA_ACCESS_FAIL_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "ch0_tx_end_int_ena",
                &format_args!("{}", self.ch0_tx_end_int_ena().bit()),
            )
            .field(
                "ch1_tx_end_int_ena",
                &format_args!("{}", self.ch1_tx_end_int_ena().bit()),
            )
            .field(
                "ch2_tx_end_int_ena",
                &format_args!("{}", self.ch2_tx_end_int_ena().bit()),
            )
            .field(
                "ch3_tx_end_int_ena",
                &format_args!("{}", self.ch3_tx_end_int_ena().bit()),
            )
            .field(
                "ch0_tx_err_int_ena",
                &format_args!("{}", self.ch0_tx_err_int_ena().bit()),
            )
            .field(
                "ch1_tx_err_int_ena",
                &format_args!("{}", self.ch1_tx_err_int_ena().bit()),
            )
            .field(
                "ch2_tx_err_int_ena",
                &format_args!("{}", self.ch2_tx_err_int_ena().bit()),
            )
            .field(
                "ch3_tx_err_int_ena",
                &format_args!("{}", self.ch3_tx_err_int_ena().bit()),
            )
            .field(
                "ch0_tx_thr_event_int_ena",
                &format_args!("{}", self.ch0_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch1_tx_thr_event_int_ena",
                &format_args!("{}", self.ch1_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch2_tx_thr_event_int_ena",
                &format_args!("{}", self.ch2_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch3_tx_thr_event_int_ena",
                &format_args!("{}", self.ch3_tx_thr_event_int_ena().bit()),
            )
            .field(
                "ch0_tx_loop_int_ena",
                &format_args!("{}", self.ch0_tx_loop_int_ena().bit()),
            )
            .field(
                "ch1_tx_loop_int_ena",
                &format_args!("{}", self.ch1_tx_loop_int_ena().bit()),
            )
            .field(
                "ch2_tx_loop_int_ena",
                &format_args!("{}", self.ch2_tx_loop_int_ena().bit()),
            )
            .field(
                "ch3_tx_loop_int_ena",
                &format_args!("{}", self.ch3_tx_loop_int_ena().bit()),
            )
            .field(
                "ch4_rx_end_int_ena",
                &format_args!("{}", self.ch4_rx_end_int_ena().bit()),
            )
            .field(
                "ch5_rx_end_int_ena",
                &format_args!("{}", self.ch5_rx_end_int_ena().bit()),
            )
            .field(
                "ch6_rx_end_int_ena",
                &format_args!("{}", self.ch6_rx_end_int_ena().bit()),
            )
            .field(
                "ch7_rx_end_int_ena",
                &format_args!("{}", self.ch7_rx_end_int_ena().bit()),
            )
            .field(
                "ch4_rx_err_int_ena",
                &format_args!("{}", self.ch4_rx_err_int_ena().bit()),
            )
            .field(
                "ch5_rx_err_int_ena",
                &format_args!("{}", self.ch5_rx_err_int_ena().bit()),
            )
            .field(
                "ch6_rx_err_int_ena",
                &format_args!("{}", self.ch6_rx_err_int_ena().bit()),
            )
            .field(
                "ch7_rx_err_int_ena",
                &format_args!("{}", self.ch7_rx_err_int_ena().bit()),
            )
            .field(
                "ch4_rx_thr_event_int_ena",
                &format_args!("{}", self.ch4_rx_thr_event_int_ena().bit()),
            )
            .field(
                "ch5_rx_thr_event_int_ena",
                &format_args!("{}", self.ch5_rx_thr_event_int_ena().bit()),
            )
            .field(
                "ch6_rx_thr_event_int_ena",
                &format_args!("{}", self.ch6_rx_thr_event_int_ena().bit()),
            )
            .field(
                "ch7_rx_thr_event_int_ena",
                &format_args!("{}", self.ch7_rx_thr_event_int_ena().bit()),
            )
            .field(
                "tx_ch3_dma_access_fail_int_ena",
                &format_args!("{}", self.tx_ch3_dma_access_fail_int_ena().bit()),
            )
            .field(
                "rx_ch7_dma_access_fail_int_ena",
                &format_args!("{}", self.rx_ch7_dma_access_fail_int_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "The interrupt enable bit for CH[0-3]_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_end_int_ena<const O: u8>(&mut self) -> CH_TX_END_INT_ENA_W<O> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<0> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<1> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<2> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<3> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_err_int_ena<const O: u8>(&mut self) -> CH_TX_ERR_INT_ENA_W<O> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<4> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<5> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<6> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<7> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_thr_event_int_ena<const O: u8>(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<O> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<8> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<9> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<10> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<11> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_tx_loop_int_ena<const O: u8>(&mut self) -> CH_TX_LOOP_INT_ENA_W<O> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<12> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<13> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<14> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<15> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_end_int_ena<const O: u8>(&mut self) -> CH_RX_END_INT_ENA_W<O> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<16> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<17> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<18> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<19> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch_rx_err_int_ena<const O: u8>(&mut self) -> CH_RX_ERR_INT_ENA_W<O> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<20> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<21> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<22> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<23> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event_int_ena(&mut self) -> CH4_RX_THR_EVENT_INT_ENA_W<24> {
        CH4_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event_int_ena(&mut self) -> CH5_RX_THR_EVENT_INT_ENA_W<25> {
        CH5_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event_int_ena(&mut self) -> CH6_RX_THR_EVENT_INT_ENA_W<26> {
        CH6_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event_int_ena(&mut self) -> CH7_RX_THR_EVENT_INT_ENA_W<27> {
        CH7_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail_int_ena(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_INT_ENA_W<28> {
        TX_CH3_DMA_ACCESS_FAIL_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail_int_ena(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_INT_ENA_W<29> {
        RX_CH7_DMA_ACCESS_FAIL_INT_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt enable bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
