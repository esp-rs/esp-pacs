///Register `INT_RAW` reader
pub type R = crate::R<INT_RAW_SPEC>;
///Field `CH_TX_END(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmission done.
pub type CH_TX_END_R = crate::BitReader;
///Field `CH_RX_END(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when reception done.
pub type CH_RX_END_R = crate::BitReader;
///Field `CH_ERR(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when error occurs.
pub type CH_ERR_R = crate::BitReader;
///Field `CH_TX_THR_EVENT(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value.
pub type CH_TX_THR_EVENT_R = crate::BitReader;
///Field `CH_TX_LOOP(0-3)` reader - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value.
pub type CH_TX_LOOP_R = crate::BitReader;
impl R {
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when transmission done.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when transmission done.
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..4).map(move |n| CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0))
    }
    ///Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done.
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    ///Bit 3 - The interrupt raw bit for CHANNEL1. Triggered when transmission done.
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when transmission done.
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 9 - The interrupt raw bit for CHANNEL3. Triggered when transmission done.
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when reception done.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when reception done.
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..4).map(move |n| CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0))
    }
    ///Bit 1 - The interrupt raw bit for CHANNEL0. Triggered when reception done.
    #[inline(always)]
    pub fn ch0_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 4 - The interrupt raw bit for CHANNEL1. Triggered when reception done.
    #[inline(always)]
    pub fn ch1_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 7 - The interrupt raw bit for CHANNEL2. Triggered when reception done.
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 10 - The interrupt raw bit for CHANNEL3. Triggered when reception done.
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when error occurs.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field
    #[inline(always)]
    pub fn ch_err(&self, n: u8) -> CH_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when error occurs.
    #[inline(always)]
    pub fn ch_err_iter(&self) -> impl Iterator<Item = CH_ERR_R> + '_ {
        (0..4).map(move |n| CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0))
    }
    ///Bit 2 - The interrupt raw bit for CHANNEL0. Triggered when error occurs.
    #[inline(always)]
    pub fn ch0_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs.
    #[inline(always)]
    pub fn ch1_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 8 - The interrupt raw bit for CHANNEL2. Triggered when error occurs.
    #[inline(always)]
    pub fn ch2_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when error occurs.
    #[inline(always)]
    pub fn ch3_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when transmitter sent more data than configured value.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..4).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    ///Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value.
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when the loop count reaches the configured threshold value.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field
    #[inline(always)]
    pub fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    ///Iterator for array of:
    ///The interrupt raw bit for CHANNEL(0-3). Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch_tx_loop_iter(&self) -> impl Iterator<Item = CH_TX_LOOP_R> + '_ {
        (0..4).map(move |n| CH_TX_LOOP_R::new(((self.bits >> (n + 16)) & 1) != 0))
    }
    ///Bit 16 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value.
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
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
/**Raw interrupt status

You can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_raw::R`](R) reader structure
impl crate::Readable for INT_RAW_SPEC {}
///`reset()` method sets INT_RAW to value 0
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
