#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CH_TX_END(0-7)` reader - Set this bit to enable rmt_ch%s_tx_end_int_st."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END(0-7)` writer - Set this bit to enable rmt_ch%s_tx_end_int_st."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END(0-7)` reader - Set this bit to enable rmt_ch%s_rx_end_int_st."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END(0-7)` writer - Set this bit to enable rmt_ch%s_rx_end_int_st."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ERR(0-7)` reader - Set this bit to enable rmt_ch%s_err_int_st."]
pub type CH_ERR_R = crate::BitReader;
#[doc = "Field `CH_ERR(0-7)` writer - Set this bit to enable rmt_ch%s_err_int_st."]
pub type CH_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT(0-7)` reader - Set this bit to enable rmt_ch%s_tx_thr_event_int_st."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT(0-7)` writer - Set this bit to enable rmt_ch%s_tx_thr_event_int_st."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Set this bit to enable rmt_ch(0-7)_tx_end_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to enable rmt_ch(0-7)_tx_end_int_st."]
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..8).map(move |n| CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0))
    }
    #[doc = "Bit 0 - Set this bit to enable rmt_ch0_tx_end_int_st."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable rmt_ch1_tx_end_int_st."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable rmt_ch2_tx_end_int_st."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable rmt_ch3_tx_end_int_st."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to enable rmt_ch4_tx_end_int_st."]
    #[inline(always)]
    pub fn ch4_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable rmt_ch5_tx_end_int_st."]
    #[inline(always)]
    pub fn ch5_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to enable rmt_ch6_tx_end_int_st."]
    #[inline(always)]
    pub fn ch6_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable rmt_ch7_tx_end_int_st."]
    #[inline(always)]
    pub fn ch7_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Set this bit to enable rmt_ch(0-7)_rx_end_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to enable rmt_ch(0-7)_rx_end_int_st."]
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..8).map(move |n| CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - Set this bit to enable rmt_ch0_rx_end_int_st."]
    #[inline(always)]
    pub fn ch0_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable rmt_ch1_rx_end_int_st."]
    #[inline(always)]
    pub fn ch1_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - Set this bit to enable rmt_ch2_rx_end_int_st."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - Set this bit to enable rmt_ch3_rx_end_int_st."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to enable rmt_ch4_rx_end_int_st."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable rmt_ch5_rx_end_int_st."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to enable rmt_ch6_rx_end_int_st."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable rmt_ch7_rx_end_int_st."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Set this bit to enable rmt_ch(0-7)_err_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field"]
    #[inline(always)]
    pub fn ch_err(&self, n: u8) -> CH_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to enable rmt_ch(0-7)_err_int_st."]
    #[inline(always)]
    pub fn ch_err_iter(&self) -> impl Iterator<Item = CH_ERR_R> + '_ {
        (0..8).map(move |n| CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - Set this bit to enable rmt_ch0_err_int_st."]
    #[inline(always)]
    pub fn ch0_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable rmt_ch1_err_int_st."]
    #[inline(always)]
    pub fn ch1_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enable rmt_ch2_err_int_st."]
    #[inline(always)]
    pub fn ch2_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to enable rmt_ch3_err_int_st."]
    #[inline(always)]
    pub fn ch3_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable rmt_ch4_err_int_st."]
    #[inline(always)]
    pub fn ch4_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to enable rmt_ch5_err_int_st."]
    #[inline(always)]
    pub fn ch5_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to enable rmt_ch6_err_int_st."]
    #[inline(always)]
    pub fn ch6_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable rmt_ch7_err_int_st."]
    #[inline(always)]
    pub fn ch7_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Set this bit to enable rmt_ch(0-7)_tx_thr_event_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Set this bit to enable rmt_ch(0-7)_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..8).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - Set this bit to enable rmt_ch0_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Set this bit to enable rmt_ch1_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set this bit to enable rmt_ch2_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bit to enable rmt_ch3_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Set this bit to enable rmt_ch4_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch4_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to enable rmt_ch5_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch5_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to enable rmt_ch6_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch6_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Set this bit to enable rmt_ch7_tx_thr_event_int_st."]
    #[inline(always)]
    pub fn ch7_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("ch0_tx_end", &self.ch0_tx_end())
            .field("ch1_tx_end", &self.ch1_tx_end())
            .field("ch2_tx_end", &self.ch2_tx_end())
            .field("ch3_tx_end", &self.ch3_tx_end())
            .field("ch4_tx_end", &self.ch4_tx_end())
            .field("ch5_tx_end", &self.ch5_tx_end())
            .field("ch6_tx_end", &self.ch6_tx_end())
            .field("ch7_tx_end", &self.ch7_tx_end())
            .field("ch0_rx_end", &self.ch0_rx_end())
            .field("ch1_rx_end", &self.ch1_rx_end())
            .field("ch2_rx_end", &self.ch2_rx_end())
            .field("ch3_rx_end", &self.ch3_rx_end())
            .field("ch4_rx_end", &self.ch4_rx_end())
            .field("ch5_rx_end", &self.ch5_rx_end())
            .field("ch6_rx_end", &self.ch6_rx_end())
            .field("ch7_rx_end", &self.ch7_rx_end())
            .field("ch0_err", &self.ch0_err())
            .field("ch1_err", &self.ch1_err())
            .field("ch2_err", &self.ch2_err())
            .field("ch3_err", &self.ch3_err())
            .field("ch4_err", &self.ch4_err())
            .field("ch5_err", &self.ch5_err())
            .field("ch6_err", &self.ch6_err())
            .field("ch7_err", &self.ch7_err())
            .field("ch0_tx_thr_event", &self.ch0_tx_thr_event())
            .field("ch1_tx_thr_event", &self.ch1_tx_thr_event())
            .field("ch2_tx_thr_event", &self.ch2_tx_thr_event())
            .field("ch3_tx_thr_event", &self.ch3_tx_thr_event())
            .field("ch4_tx_thr_event", &self.ch4_tx_thr_event())
            .field("ch5_tx_thr_event", &self.ch5_tx_thr_event())
            .field("ch6_tx_thr_event", &self.ch6_tx_thr_event())
            .field("ch7_tx_thr_event", &self.ch7_tx_thr_event())
            .finish()
    }
}
impl W {
    #[doc = "Set this bit to enable rmt_ch(0-7)_tx_end_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_END_W::new(self, n * 3)
    }
    #[doc = "Bit 0 - Set this bit to enable rmt_ch0_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 3 - Set this bit to enable rmt_ch1_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 3)
    }
    #[doc = "Bit 6 - Set this bit to enable rmt_ch2_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 6)
    }
    #[doc = "Bit 9 - Set this bit to enable rmt_ch3_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 9)
    }
    #[doc = "Bit 12 - Set this bit to enable rmt_ch4_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 12)
    }
    #[doc = "Bit 15 - Set this bit to enable rmt_ch5_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 15)
    }
    #[doc = "Bit 18 - Set this bit to enable rmt_ch6_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 18)
    }
    #[doc = "Bit 21 - Set this bit to enable rmt_ch7_tx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 21)
    }
    #[doc = "Set this bit to enable rmt_ch(0-7)_rx_end_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_RX_END_W::new(self, n * 3 + 1)
    }
    #[doc = "Bit 1 - Set this bit to enable rmt_ch0_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 1)
    }
    #[doc = "Bit 4 - Set this bit to enable rmt_ch1_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set this bit to enable rmt_ch2_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 7)
    }
    #[doc = "Bit 10 - Set this bit to enable rmt_ch3_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 10)
    }
    #[doc = "Bit 13 - Set this bit to enable rmt_ch4_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 13)
    }
    #[doc = "Bit 16 - Set this bit to enable rmt_ch5_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 16)
    }
    #[doc = "Bit 19 - Set this bit to enable rmt_ch6_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 19)
    }
    #[doc = "Bit 22 - Set this bit to enable rmt_ch7_rx_end_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 22)
    }
    #[doc = "Set this bit to enable rmt_ch(0-7)_err_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_err(&mut self, n: u8) -> CH_ERR_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_ERR_W::new(self, n * 3 + 2)
    }
    #[doc = "Bit 2 - Set this bit to enable rmt_ch0_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 2)
    }
    #[doc = "Bit 5 - Set this bit to enable rmt_ch1_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 5)
    }
    #[doc = "Bit 8 - Set this bit to enable rmt_ch2_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 8)
    }
    #[doc = "Bit 11 - Set this bit to enable rmt_ch3_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 11)
    }
    #[doc = "Bit 14 - Set this bit to enable rmt_ch4_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 14)
    }
    #[doc = "Bit 17 - Set this bit to enable rmt_ch5_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 17)
    }
    #[doc = "Bit 20 - Set this bit to enable rmt_ch6_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 20)
    }
    #[doc = "Bit 23 - Set this bit to enable rmt_ch7_err_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 23)
    }
    #[doc = "Set this bit to enable rmt_ch(0-7)_tx_thr_event_int_st."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - Set this bit to enable rmt_ch0_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to enable rmt_ch1_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to enable rmt_ch2_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set this bit to enable rmt_ch3_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set this bit to enable rmt_ch4_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to enable rmt_ch5_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to enable rmt_ch6_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 30)
    }
    #[doc = "Bit 31 - Set this bit to enable rmt_ch7_tx_thr_event_int_st."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
