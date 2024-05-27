///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `CH_TX_END(0-7)` writer - Set this bit to clear the rmt_ch%s_rx_end_int_raw..
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_RX_END(0-7)` writer - Set this bit to clear the rmt_ch%s_tx_end_int_raw.
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_ERR(0-7)` writer - Set this bit to clear the rmt_ch%s_err_int_raw.
pub type CH_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_TX_THR_EVENT(0-7)` writer - Set this bit to clear the rmt_ch%s_tx_thr_event_int_raw interrupt.
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Set this bit to clear the rmt_ch(0-7)_rx_end_int_raw..
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_END_W::new(self, n * 3)
    }
    ///Bit 0 - Set this bit to clear the rmt_ch0_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    ///Bit 3 - Set this bit to clear the rmt_ch1_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 3)
    }
    ///Bit 6 - Set this bit to clear the rmt_ch2_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 6)
    }
    ///Bit 9 - Set this bit to clear the rmt_ch3_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 9)
    }
    ///Bit 12 - Set this bit to clear the rmt_ch4_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch4_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 12)
    }
    ///Bit 15 - Set this bit to clear the rmt_ch5_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch5_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 15)
    }
    ///Bit 18 - Set this bit to clear the rmt_ch6_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch6_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 18)
    }
    ///Bit 21 - Set this bit to clear the rmt_ch7_rx_end_int_raw..
    #[inline(always)]
    #[must_use]
    pub fn ch7_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 21)
    }
    ///Set this bit to clear the rmt_ch(0-7)_tx_end_int_raw.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_RX_END_W::new(self, n * 3 + 1)
    }
    ///Bit 1 - Set this bit to clear the rmt_ch0_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch0_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 1)
    }
    ///Bit 4 - Set this bit to clear the rmt_ch1_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch1_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 4)
    }
    ///Bit 7 - Set this bit to clear the rmt_ch2_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 7)
    }
    ///Bit 10 - Set this bit to clear the rmt_ch3_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 10)
    }
    ///Bit 13 - Set this bit to clear the rmt_ch4_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 13)
    }
    ///Bit 16 - Set this bit to clear the rmt_ch5_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 16)
    }
    ///Bit 19 - Set this bit to clear the rmt_ch6_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 19)
    }
    ///Bit 22 - Set this bit to clear the rmt_ch7_tx_end_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 22)
    }
    ///Set this bit to clear the rmt_ch(0-7)_err_int_raw.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field
    #[inline(always)]
    #[must_use]
    pub fn ch_err(&mut self, n: u8) -> CH_ERR_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_ERR_W::new(self, n * 3 + 2)
    }
    ///Bit 2 - Set this bit to clear the rmt_ch0_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch0_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 2)
    }
    ///Bit 5 - Set this bit to clear the rmt_ch1_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch1_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 5)
    }
    ///Bit 8 - Set this bit to clear the rmt_ch2_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch2_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 8)
    }
    ///Bit 11 - Set this bit to clear the rmt_ch3_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch3_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 11)
    }
    ///Bit 14 - Set this bit to clear the rmt_ch4_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch4_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 14)
    }
    ///Bit 17 - Set this bit to clear the rmt_ch5_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch5_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 17)
    }
    ///Bit 20 - Set this bit to clear the rmt_ch6_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch6_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 20)
    }
    ///Bit 23 - Set this bit to clear the rmt_ch7_err_int_raw.
    #[inline(always)]
    #[must_use]
    pub fn ch7_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 23)
    }
    ///Set this bit to clear the rmt_ch(0-7)_tx_thr_event_int_raw interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 24)
    }
    ///Bit 24 - Set this bit to clear the rmt_ch0_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 24)
    }
    ///Bit 25 - Set this bit to clear the rmt_ch1_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 25)
    }
    ///Bit 26 - Set this bit to clear the rmt_ch2_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 26)
    }
    ///Bit 27 - Set this bit to clear the rmt_ch3_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 27)
    }
    ///Bit 28 - Set this bit to clear the rmt_ch4_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch4_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 28)
    }
    ///Bit 29 - Set this bit to clear the rmt_ch5_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch5_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 29)
    }
    ///Bit 30 - Set this bit to clear the rmt_ch6_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch6_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 30)
    }
    ///Bit 31 - Set this bit to clear the rmt_ch7_tx_thr_event_int_raw interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch7_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 31)
    }
}
/**

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`int_clr::W`](W) writer structure
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
