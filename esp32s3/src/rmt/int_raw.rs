#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `CH_TX_END(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmission done."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END(0-3)` writer - The interrupt raw bit for CHANNEL%s. Triggered when transmission done."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_ERR(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when error occurs."]
pub type CH_TX_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR(0-3)` writer - The interrupt raw bit for CHANNEL%s. Triggered when error occurs."]
pub type CH_TX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT(0-3)` writer - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_LOOP(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value."]
pub type CH_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP(0-3)` writer - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value."]
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END(4-7)` reader - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END(4-7)` writer - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_ERR(4-7)` reader - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH_RX_ERR_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR(4-7)` writer - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH_RX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_THR_EVENT(4-7)` reader - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type CH_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_RX_THR_EVENT(4-7)` writer - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type CH_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` reader - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type TX_CH3_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` writer - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` reader - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type RX_CH7_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` writer - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when transmission done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when transmission done."]
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..4).map(move |n| CH_TX_END_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when error occurs."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field"]
    #[inline(always)]
    pub fn ch_tx_err(&self, n: u8) -> CH_TX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when error occurs."]
    #[inline(always)]
    pub fn ch_tx_err_iter(&self) -> impl Iterator<Item = CH_TX_ERR_R> + '_ {
        (0..4).map(move |n| CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when transmitter sent more data than configured value."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..4).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when the loop count reaches the configured threshold value."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field"]
    #[inline(always)]
    pub fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch_tx_loop_iter(&self) -> impl Iterator<Item = CH_TX_LOOP_R> + '_ {
        (0..4).map(move |n| CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_END` field"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..4).map(move |n| CH_RX_END_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_ERR` field"]
    #[inline(always)]
    pub fn ch_rx_err(&self, n: u8) -> CH_RX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_ERR_R::new(((self.bits >> (n + 20)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch_rx_err_iter(&self) -> impl Iterator<Item = CH_RX_ERR_R> + '_ {
        (0..4).map(move |n| CH_RX_ERR_R::new(((self.bits >> (n + 20)) & 1) != 0))
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch4_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch5_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch6_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch7_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_THR_EVENT` field"]
    #[inline(always)]
    pub fn ch_rx_thr_event(&self, n: u8) -> CH_RX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch_rx_thr_event_iter(&self) -> impl Iterator<Item = CH_RX_THR_EVENT_R> + '_ {
        (0..4).map(move |n| CH_RX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail(&self) -> TX_CH3_DMA_ACCESS_FAIL_R {
        TX_CH3_DMA_ACCESS_FAIL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail(&self) -> RX_CH7_DMA_ACCESS_FAIL_R {
        RX_CH7_DMA_ACCESS_FAIL_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("ch0_tx_end", &self.ch0_tx_end().bit())
            .field("ch1_tx_end", &self.ch1_tx_end().bit())
            .field("ch2_tx_end", &self.ch2_tx_end().bit())
            .field("ch3_tx_end", &self.ch3_tx_end().bit())
            .field("ch0_tx_err", &self.ch0_tx_err().bit())
            .field("ch1_tx_err", &self.ch1_tx_err().bit())
            .field("ch2_tx_err", &self.ch2_tx_err().bit())
            .field("ch3_tx_err", &self.ch3_tx_err().bit())
            .field("ch0_tx_thr_event", &self.ch0_tx_thr_event().bit())
            .field("ch1_tx_thr_event", &self.ch1_tx_thr_event().bit())
            .field("ch2_tx_thr_event", &self.ch2_tx_thr_event().bit())
            .field("ch3_tx_thr_event", &self.ch3_tx_thr_event().bit())
            .field("ch0_tx_loop", &self.ch0_tx_loop().bit())
            .field("ch1_tx_loop", &self.ch1_tx_loop().bit())
            .field("ch2_tx_loop", &self.ch2_tx_loop().bit())
            .field("ch3_tx_loop", &self.ch3_tx_loop().bit())
            .field("ch4_rx_end", &self.ch4_rx_end().bit())
            .field("ch5_rx_end", &self.ch5_rx_end().bit())
            .field("ch6_rx_end", &self.ch6_rx_end().bit())
            .field("ch7_rx_end", &self.ch7_rx_end().bit())
            .field("ch4_rx_err", &self.ch4_rx_err().bit())
            .field("ch5_rx_err", &self.ch5_rx_err().bit())
            .field("ch6_rx_err", &self.ch6_rx_err().bit())
            .field("ch7_rx_err", &self.ch7_rx_err().bit())
            .field("ch4_rx_thr_event", &self.ch4_rx_thr_event().bit())
            .field("ch5_rx_thr_event", &self.ch5_rx_thr_event().bit())
            .field("ch6_rx_thr_event", &self.ch6_rx_thr_event().bit())
            .field("ch7_rx_thr_event", &self.ch7_rx_thr_event().bit())
            .field(
                "tx_ch3_dma_access_fail",
                &self.tx_ch3_dma_access_fail().bit(),
            )
            .field(
                "rx_ch7_dma_access_fail",
                &self.rx_ch7_dma_access_fail().bit(),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when transmission done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_W::new(self, n)
    }
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC> {
        CH_TX_END_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC> {
        CH_TX_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC> {
        CH_TX_END_W::new(self, 3)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when error occurs."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_err(&mut self, n: u8) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_ERR_W::new(self, n + 4)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        CH_TX_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        CH_TX_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        CH_TX_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        CH_TX_ERR_W::new(self, 7)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when transmitter sent more data than configured value."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 8)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 11)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-3). Triggered when the loop count reaches the configured threshold value."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        CH_TX_LOOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        CH_TX_LOOP_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        CH_TX_LOOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        CH_TX_LOOP_W::new(self, 15)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC> {
        CH_RX_END_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC> {
        CH_RX_END_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC> {
        CH_RX_END_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC> {
        CH_RX_END_W::new(self, 19)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_err(&mut self, n: u8) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_ERR_W::new(self, n + 20)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        CH_RX_ERR_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        CH_RX_ERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        CH_RX_ERR_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        CH_RX_ERR_W::new(self, 23)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_thr_event(&mut self, n: u8) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_THR_EVENT_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 24)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 25)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 26)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<INT_RAW_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<INT_RAW_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self, 29)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_raw::W`](W) writer structure"]
impl crate::Writable for INT_RAW_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
