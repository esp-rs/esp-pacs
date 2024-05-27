///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `CH_TX_END(0-1)` writer - Set this bit to clear theCH%s_TX_END_INT interrupt.
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_RX_END(2-3)` writer - Set this bit to clear theCH2_RX_END_INT interrupt.
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_TX_ERR(0-1)` writer - Set this bit to clear theCH4_ERR_INT interrupt.
pub type CH_TX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_RX_ERR(2-3)` writer - Set this bit to clear theCH6_ERR_INT interrupt.
pub type CH_RX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_TX_THR_EVENT(0-1)` writer - Set this bit to clear theCH%s_TX_THR_EVENT_INT interrupt.
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_RX_THR_EVENT(2-3)` writer - Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt.
pub type CH_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH_TX_LOOP(0-1)` writer - Set this bit to clear theCH%s_TX_LOOP_INT interrupt.
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Set this bit to clear theCH(0-1)_TX_END_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_END_W::new(self, n)
    }
    ///Bit 0 - Set this bit to clear theCH0_TX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear theCH1_TX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 1)
    }
    ///Set this bit to clear theCH2_RX_END_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_END` field
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_END_W::new(self, n + 2)
    }
    ///Bit 2 - Set this bit to clear theCH2_RX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear theCH2_RX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 3)
    }
    ///Set this bit to clear theCH4_ERR_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_err(&mut self, n: u8) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_ERR_W::new(self, n + 4)
    }
    ///Bit 4 - Set this bit to clear theCH4_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        CH_TX_ERR_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to clear theCH4_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        CH_TX_ERR_W::new(self, 5)
    }
    ///Set this bit to clear theCH6_ERR_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_ERR` field
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_err(&mut self, n: u8) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_ERR_W::new(self, n + 6)
    }
    ///Bit 6 - Set this bit to clear theCH6_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        CH_RX_ERR_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to clear theCH6_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        CH_RX_ERR_W::new(self, 7)
    }
    ///Set this bit to clear theCH(0-1)_TX_THR_EVENT_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 8)
    }
    ///Bit 8 - Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 8)
    }
    ///Bit 9 - Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 9)
    }
    ///Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_THR_EVENT` field
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_thr_event(&mut self, n: u8) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_THR_EVENT_W::new(self, n + 10)
    }
    ///Bit 10 - Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 10)
    }
    ///Bit 11 - Set this bit to clear theCH2_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 11)
    }
    ///Set this bit to clear theCH(0-1)_TX_LOOP_INT interrupt.
    ///
    ///NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_LOOP_W::new(self, n + 12)
    }
    ///Bit 12 - Set this bit to clear theCH0_TX_LOOP_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 12)
    }
    ///Bit 13 - Set this bit to clear theCH1_TX_LOOP_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 13)
    }
}
/**Interrupt clear bits

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
