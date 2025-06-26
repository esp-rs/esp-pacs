#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `CH_TX_END(0-3)` writer - Set this bit to clear the CH%s_TX_END_INT interrupt."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END(0-3)` writer - Set this bit to clear the CH%s_RX_END_INT interrupt."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ERR(0-3)` writer - Set this bit to clear the CH%s_ERR_INT interrupt."]
pub type CH_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT(0-3)` writer - Set this bit to clear the CH%s_TX_THR_EVENT_INT interrupt."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_LOOP(0-3)` writer - Set this bit to clear the CH%s_TX_LOOP_INT interrupt."]
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set this bit to clear the CH(0-3)_TX_END_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_W::new(self, n * 3)
    }
    #[doc = "Bit 0 - Set this bit to clear the CH0_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 3 - Set this bit to clear the CH1_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 3)
    }
    #[doc = "Bit 6 - Set this bit to clear the CH2_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 6)
    }
    #[doc = "Bit 9 - Set this bit to clear the CH3_TX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 9)
    }
    #[doc = "Set this bit to clear the CH(0-3)_RX_END_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_W::new(self, n * 3 + 1)
    }
    #[doc = "Bit 1 - Set this bit to clear the CH0_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch0_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 1)
    }
    #[doc = "Bit 4 - Set this bit to clear the CH1_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch1_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 4)
    }
    #[doc = "Bit 7 - Set this bit to clear the CH2_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 7)
    }
    #[doc = "Bit 10 - Set this bit to clear the CH3_RX_END_INT interrupt."]
    #[inline(always)]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 10)
    }
    #[doc = "Set this bit to clear the CH(0-3)_ERR_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_err(&mut self, n: u8) -> CH_ERR_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_ERR_W::new(self, n * 3 + 2)
    }
    #[doc = "Bit 2 - Set this bit to clear the CH0_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch0_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 2)
    }
    #[doc = "Bit 5 - Set this bit to clear the CH1_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch1_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 5)
    }
    #[doc = "Bit 8 - Set this bit to clear the CH2_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch2_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 8)
    }
    #[doc = "Bit 11 - Set this bit to clear the CH3_ERR_INT interrupt."]
    #[inline(always)]
    pub fn ch3_err(&mut self) -> CH_ERR_W<INT_CLR_SPEC> {
        CH_ERR_W::new(self, 11)
    }
    #[doc = "Set this bit to clear the CH(0-3)_TX_THR_EVENT_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - Set this bit to clear the CH0_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear the CH1_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear the CH2_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear the CH3_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 15)
    }
    #[doc = "Set this bit to clear the CH(0-3)_TX_LOOP_INT interrupt."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Set this bit to clear the CH0_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to clear the CH1_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to clear the CH2_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch2_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to clear the CH3_TX_LOOP_INT interrupt."]
    #[inline(always)]
    pub fn ch3_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 19)
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {}
