#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `CH_TX_END(0-3)` reader - The masked interrupt status bit for CH%s_TX_END_INT."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END(0-3)` reader - The masked interrupt status bit for CH%s_RX_END_INT."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_ERR(0-3)` reader - The masked interrupt status bit for CH%s_ERR_INT."]
pub type CH_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT(0-3)` reader - The masked interrupt status bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP(0-3)` reader - The masked interrupt status bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_R = crate::BitReader;
impl R {
    #[doc = "The masked interrupt status bit for CH(0-3)_TX_END_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for CH(0-3)_TX_END_INT."]
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..4).map(move |n| CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0))
    }
    #[doc = "Bit 0 - The masked interrupt status bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - The masked interrupt status bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - The masked interrupt status bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - The masked interrupt status bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH(0-3)_RX_END_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for CH(0-3)_RX_END_INT."]
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..4).map(move |n| CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - The masked interrupt status bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - The masked interrupt status bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - The masked interrupt status bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - The masked interrupt status bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH(0-3)_ERR_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_err(&self, n: u8) -> CH_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for CH(0-3)_ERR_INT."]
    #[inline(always)]
    pub fn ch_err_iter(&self) -> impl Iterator<Item = CH_ERR_R> + '_ {
        (0..4).map(move |n| CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - The masked interrupt status bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - The masked interrupt status bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - The masked interrupt status bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - The masked interrupt status bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH(0-3)_TX_THR_EVENT_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for CH(0-3)_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..4).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - The masked interrupt status bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The masked interrupt status bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The masked interrupt status bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The masked interrupt status bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The masked interrupt status bit for CH(0-3)_TX_LOOP_INT."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The masked interrupt status bit for CH(0-3)_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch_tx_loop_iter(&self) -> impl Iterator<Item = CH_TX_LOOP_R> + '_ {
        (0..4).map(move |n| CH_TX_LOOP_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - The masked interrupt status bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The masked interrupt status bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The masked interrupt status bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The masked interrupt status bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("ch0_tx_end", &self.ch0_tx_end())
            .field("ch1_tx_end", &self.ch1_tx_end())
            .field("ch2_tx_end", &self.ch2_tx_end())
            .field("ch3_tx_end", &self.ch3_tx_end())
            .field("ch0_rx_end", &self.ch0_rx_end())
            .field("ch1_rx_end", &self.ch1_rx_end())
            .field("ch2_rx_end", &self.ch2_rx_end())
            .field("ch3_rx_end", &self.ch3_rx_end())
            .field("ch0_err", &self.ch0_err())
            .field("ch1_err", &self.ch1_err())
            .field("ch2_err", &self.ch2_err())
            .field("ch3_err", &self.ch3_err())
            .field("ch0_tx_thr_event", &self.ch0_tx_thr_event())
            .field("ch1_tx_thr_event", &self.ch1_tx_thr_event())
            .field("ch2_tx_thr_event", &self.ch2_tx_thr_event())
            .field("ch3_tx_thr_event", &self.ch3_tx_thr_event())
            .field("ch0_tx_loop", &self.ch0_tx_loop())
            .field("ch1_tx_loop", &self.ch1_tx_loop())
            .field("ch2_tx_loop", &self.ch2_tx_loop())
            .field("ch3_tx_loop", &self.ch3_tx_loop())
            .finish()
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
