#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `CH_TX_END(0-3)` writer - Set this bit to clear theCH%s_TX_END_INT interrupt."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_ERR(0-3)` writer - Set this bit to clear theCH%s_ERR_INT interrupt."]
pub type CH_TX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT(0-3)` writer - Set this bit to clear theCH%s_TX_THR_EVENT_INT interrupt."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_LOOP(0-3)` writer - Set this bit to clear theCH%s_TX_LOOP_INT interrupt."]
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END(4-7)` writer - Set this bit to clear theCH4_RX_END_INT interrupt."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_ERR(4-7)` writer - Set this bit to clear theCH4_ERR_INT interrupt."]
pub type CH_RX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_THR_EVENT(4-7)` writer - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
pub type CH_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` writer - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt."]
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` writer - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt."]
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Set this bit to clear theCH(0-3)_TX_END_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_W::new(self, n)
    }
    #[doc = "Bit 0 - Set this bit to clear theCH0_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear theCH1_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear theCH2_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear theCH3_TX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_CLR_SPEC> {
        CH_TX_END_W::new(self, 3)
    }
    #[doc = "Set this bit to clear theCH(0-3)_ERR_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_err(&mut self, n: u8) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_ERR_W::new(self, n + 4)
    }
    #[doc = "Bit 4 - Set this bit to clear theCH0_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        CH_TX_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear theCH1_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        CH_TX_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear theCH2_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        CH_TX_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear theCH3_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_err(&mut self) -> CH_TX_ERR_W<INT_CLR_SPEC> {
        CH_TX_ERR_W::new(self, 7)
    }
    #[doc = "Set this bit to clear theCH(0-3)_TX_THR_EVENT_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 8)
    }
    #[doc = "Bit 8 - Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 11)
    }
    #[doc = "Set this bit to clear theCH(0-3)_TX_LOOP_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - Set this bit to clear theCH0_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - Set this bit to clear theCH1_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 13)
    }
    #[doc = "Bit 14 - Set this bit to clear theCH2_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - Set this bit to clear theCH3_TX_LOOP_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_CLR_SPEC> {
        CH_TX_LOOP_W::new(self, 15)
    }
    #[doc = "Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 16)
    }
    #[doc = "Bit 17 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 17)
    }
    #[doc = "Bit 18 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 18)
    }
    #[doc = "Bit 19 - Set this bit to clear theCH4_RX_END_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH_RX_END_W<INT_CLR_SPEC> {
        CH_RX_END_W::new(self, 19)
    }
    #[doc = "Set this bit to clear theCH4_ERR_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_err(&mut self, n: u8) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_ERR_W::new(self, n + 20)
    }
    #[doc = "Bit 20 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        CH_RX_ERR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        CH_RX_ERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        CH_RX_ERR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Set this bit to clear theCH4_ERR_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_err(&mut self) -> CH_RX_ERR_W<INT_CLR_SPEC> {
        CH_RX_ERR_W::new(self, 23)
    }
    #[doc = "Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH4_RX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_thr_event(&mut self, n: u8) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_THR_EVENT_W::new(self, n + 24)
    }
    #[doc = "Bit 24 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 24)
    }
    #[doc = "Bit 25 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 25)
    }
    #[doc = "Bit 26 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 26)
    }
    #[doc = "Bit 27 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 27)
    }
    #[doc = "Bit 28 - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<INT_CLR_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<INT_CLR_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self, 29)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Interrupt clear bits\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
