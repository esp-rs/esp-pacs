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
pub type CH_TX_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_END_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_TX_END_INT."]
pub type CH_TX_END_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH_TX_ERR_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_ERR_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_TX_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH_TX_THR_EVENT_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_THR_EVENT_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH_TX_LOOP_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_LOOP_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH_RX_END_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_RX_END_INT."]
pub type CH_RX_END_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH_RX_END_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_RX_END_INT."]
pub type CH_RX_END_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH_RX_ERR_INT_ENA[0-3]` reader - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_RX_ERR_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH_RX_ERR_INT_ENA[0-3]` writer - The interrupt enable bit for CH%s_ERR_INT."]
pub type CH_RX_ERR_INT_ENA_W<'a, const O: u8> = crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH0_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH0_RX_THR_EVENT_INT."]
pub type CH0_RX_THR_EVENT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH0_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH0_RX_THR_EVENT_INT."]
pub type CH0_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH1_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH1_RX_THR_EVENT_INT."]
pub type CH1_RX_THR_EVENT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH1_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH1_RX_THR_EVENT_INT."]
pub type CH1_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH2_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH2_RX_THR_EVENT_INT."]
pub type CH2_RX_THR_EVENT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH2_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH2_RX_THR_EVENT_INT."]
pub type CH2_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `CH3_RX_THR_EVENT_INT_ENA` reader - The interrupt enable bit for CH3_RX_THR_EVENT_INT."]
pub type CH3_RX_THR_EVENT_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `CH3_RX_THR_EVENT_INT_ENA` writer - The interrupt enable bit for CH3_RX_THR_EVENT_INT."]
pub type CH3_RX_THR_EVENT_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `TX_CH0_DMA_ACCESS_FAIL_INT_ENA` reader - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
pub type TX_CH0_DMA_ACCESS_FAIL_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `TX_CH0_DMA_ACCESS_FAIL_INT_ENA` writer - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
pub type TX_CH0_DMA_ACCESS_FAIL_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
#[doc = "Field `RX_CH0_DMA_ACCESS_FAIL_INT_ENA` reader - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
pub type RX_CH0_DMA_ACCESS_FAIL_INT_ENA_R = crate::BitReader<bool>;
#[doc = "Field `RX_CH0_DMA_ACCESS_FAIL_INT_ENA` writer - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
pub type RX_CH0_DMA_ACCESS_FAIL_INT_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_ENA_SPEC, bool, O>;
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
    #[doc = "The interrupt enable bit for CH[0-3]_RX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_ena(&self, n: u8) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&self) -> CH_RX_END_INT_ENA_R {
        CH_RX_END_INT_ENA_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_ena(&self, n: u8) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> (n + 20)) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_rx_err_int_ena(&self) -> CH_RX_ERR_INT_ENA_R {
        CH_RX_ERR_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH0_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_rx_thr_event_int_ena(&self) -> CH0_RX_THR_EVENT_INT_ENA_R {
        CH0_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH1_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_rx_thr_event_int_ena(&self) -> CH1_RX_THR_EVENT_INT_ENA_R {
        CH1_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH2_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_ena(&self) -> CH2_RX_THR_EVENT_INT_ENA_R {
        CH2_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH3_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_ena(&self) -> CH3_RX_THR_EVENT_INT_ENA_R {
        CH3_RX_THR_EVENT_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch0_dma_access_fail_int_ena(&self) -> TX_CH0_DMA_ACCESS_FAIL_INT_ENA_R {
        TX_CH0_DMA_ACCESS_FAIL_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch0_dma_access_fail_int_ena(&self) -> RX_CH0_DMA_ACCESS_FAIL_INT_ENA_R {
        RX_CH0_DMA_ACCESS_FAIL_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
}
impl W {
    #[doc = "The interrupt enable bit for CH[0-3]_TX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_ena<const O: u8>(&mut self) -> CH_TX_END_INT_ENA_W<O> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<0> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<1> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<2> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end_int_ena(&mut self) -> CH_TX_END_INT_ENA_W<3> {
        CH_TX_END_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_err_int_ena<const O: u8>(&mut self) -> CH_TX_ERR_INT_ENA_W<O> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<4> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<5> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<6> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_tx_err_int_ena(&mut self) -> CH_TX_ERR_INT_ENA_W<7> {
        CH_TX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_ena<const O: u8>(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<O> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<8> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<9> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<10> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_ena(&mut self) -> CH_TX_THR_EVENT_INT_ENA_W<11> {
        CH_TX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_TX_LOOP_INT."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_ena<const O: u8>(&mut self) -> CH_TX_LOOP_INT_ENA_W<O> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<12> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<13> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<14> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_ena(&mut self) -> CH_TX_LOOP_INT_ENA_W<15> {
        CH_TX_LOOP_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_RX_END_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_ena<const O: u8>(&mut self) -> CH_RX_END_INT_ENA_W<O> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<16> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<17> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<18> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end_int_ena(&mut self) -> CH_RX_END_INT_ENA_W<19> {
        CH_RX_END_INT_ENA_W::new(self)
    }
    #[doc = "The interrupt enable bit for CH[0-3]_ERR_INT."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_ena<const O: u8>(&mut self) -> CH_RX_ERR_INT_ENA_W<O> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<20> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<21> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<22> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_rx_err_int_ena(&mut self) -> CH_RX_ERR_INT_ENA_W<23> {
        CH_RX_ERR_INT_ENA_W::new(self)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH0_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_rx_thr_event_int_ena(&mut self) -> CH0_RX_THR_EVENT_INT_ENA_W<24> {
        CH0_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH1_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_rx_thr_event_int_ena(&mut self) -> CH1_RX_THR_EVENT_INT_ENA_W<25> {
        CH1_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH2_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_rx_thr_event_int_ena(&mut self) -> CH2_RX_THR_EVENT_INT_ENA_W<26> {
        CH2_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH3_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_rx_thr_event_int_ena(&mut self) -> CH3_RX_THR_EVENT_INT_ENA_W<27> {
        CH3_RX_THR_EVENT_INT_ENA_W::new(self)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch0_dma_access_fail_int_ena(&mut self) -> TX_CH0_DMA_ACCESS_FAIL_INT_ENA_W<28> {
        TX_CH0_DMA_ACCESS_FAIL_INT_ENA_W::new(self)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH0_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch0_dma_access_fail_int_ena(&mut self) -> RX_CH0_DMA_ACCESS_FAIL_INT_ENA_W<29> {
        RX_CH0_DMA_ACCESS_FAIL_INT_ENA_W::new(self)
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
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
