///Register `INT_CLR` writer
pub type W = crate::W<INT_CLR_SPEC>;
///Field `CH0_TX_END` writer - Set this bit to clear theCH0_TX_END_INT interrupt.
pub type CH0_TX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH1_TX_END` writer - Set this bit to clear theCH1_TX_END_INT interrupt.
pub type CH1_TX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH2_TX_END` writer - Set this bit to clear theCH2_TX_END_INT interrupt.
pub type CH2_TX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH3_TX_END` writer - Set this bit to clear theCH3_TX_END_INT interrupt.
pub type CH3_TX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_CH0_ERR` writer - Set this bit to clear theCH0_ERR_INT interrupt.
pub type TX_CH0_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_CH1_ERR` writer - Set this bit to clear theCH1_ERR_INT interrupt.
pub type TX_CH1_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_CH2_ERR` writer - Set this bit to clear theCH2_ERR_INT interrupt.
pub type TX_CH2_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_CH3_ERR` writer - Set this bit to clear theCH3_ERR_INT interrupt.
pub type TX_CH3_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH0_TX_THR_EVENT` writer - Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt.
pub type CH0_TX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH1_TX_THR_EVENT` writer - Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt.
pub type CH1_TX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH2_TX_THR_EVENT` writer - Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt.
pub type CH2_TX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH3_TX_THR_EVENT` writer - Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt.
pub type CH3_TX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH0_TX_LOOP` writer - Set this bit to clear theCH0_TX_LOOP_INT interrupt.
pub type CH0_TX_LOOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH1_TX_LOOP` writer - Set this bit to clear theCH1_TX_LOOP_INT interrupt.
pub type CH1_TX_LOOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH2_TX_LOOP` writer - Set this bit to clear theCH2_TX_LOOP_INT interrupt.
pub type CH2_TX_LOOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH3_TX_LOOP` writer - Set this bit to clear theCH3_TX_LOOP_INT interrupt.
pub type CH3_TX_LOOP_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH4_RX_END` writer - Set this bit to clear theCH4_RX_END_INT interrupt.
pub type CH4_RX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH5_RX_END` writer - Set this bit to clear theCH5_RX_END_INT interrupt.
pub type CH5_RX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH6_RX_END` writer - Set this bit to clear theCH6_RX_END_INT interrupt.
pub type CH6_RX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH7_RX_END` writer - Set this bit to clear theCH7_RX_END_INT interrupt.
pub type CH7_RX_END_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_CH4_ERR` writer - Set this bit to clear theCH4_ERR_INT interrupt.
pub type RX_CH4_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_CH5_ERR` writer - Set this bit to clear theCH5_ERR_INT interrupt.
pub type RX_CH5_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_CH6_ERR` writer - Set this bit to clear theCH6_ERR_INT interrupt.
pub type RX_CH6_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_CH7_ERR` writer - Set this bit to clear theCH7_ERR_INT interrupt.
pub type RX_CH7_ERR_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH4_RX_THR_EVENT` writer - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt.
pub type CH4_RX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH5_RX_THR_EVENT` writer - Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt.
pub type CH5_RX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH6_RX_THR_EVENT` writer - Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt.
pub type CH6_RX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `CH7_RX_THR_EVENT` writer - Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt.
pub type CH7_RX_THR_EVENT_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `TX_CH3_DMA_ACCESS_FAIL` writer - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt.
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
///Field `RX_CH7_DMA_ACCESS_FAIL` writer - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt.
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bit 0 - Set this bit to clear theCH0_TX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH0_TX_END_W<INT_CLR_SPEC> {
        CH0_TX_END_W::new(self, 0)
    }
    ///Bit 1 - Set this bit to clear theCH1_TX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH1_TX_END_W<INT_CLR_SPEC> {
        CH1_TX_END_W::new(self, 1)
    }
    ///Bit 2 - Set this bit to clear theCH2_TX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH2_TX_END_W<INT_CLR_SPEC> {
        CH2_TX_END_W::new(self, 2)
    }
    ///Bit 3 - Set this bit to clear theCH3_TX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH3_TX_END_W<INT_CLR_SPEC> {
        CH3_TX_END_W::new(self, 3)
    }
    ///Bit 4 - Set this bit to clear theCH0_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch0_err(&mut self) -> TX_CH0_ERR_W<INT_CLR_SPEC> {
        TX_CH0_ERR_W::new(self, 4)
    }
    ///Bit 5 - Set this bit to clear theCH1_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch1_err(&mut self) -> TX_CH1_ERR_W<INT_CLR_SPEC> {
        TX_CH1_ERR_W::new(self, 5)
    }
    ///Bit 6 - Set this bit to clear theCH2_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch2_err(&mut self) -> TX_CH2_ERR_W<INT_CLR_SPEC> {
        TX_CH2_ERR_W::new(self, 6)
    }
    ///Bit 7 - Set this bit to clear theCH3_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_err(&mut self) -> TX_CH3_ERR_W<INT_CLR_SPEC> {
        TX_CH3_ERR_W::new(self, 7)
    }
    ///Bit 8 - Set this bit to clear theCH0_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH0_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH0_TX_THR_EVENT_W::new(self, 8)
    }
    ///Bit 9 - Set this bit to clear theCH1_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH1_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH1_TX_THR_EVENT_W::new(self, 9)
    }
    ///Bit 10 - Set this bit to clear theCH2_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH2_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH2_TX_THR_EVENT_W::new(self, 10)
    }
    ///Bit 11 - Set this bit to clear theCH3_TX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH3_TX_THR_EVENT_W<INT_CLR_SPEC> {
        CH3_TX_THR_EVENT_W::new(self, 11)
    }
    ///Bit 12 - Set this bit to clear theCH0_TX_LOOP_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH0_TX_LOOP_W<INT_CLR_SPEC> {
        CH0_TX_LOOP_W::new(self, 12)
    }
    ///Bit 13 - Set this bit to clear theCH1_TX_LOOP_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH1_TX_LOOP_W<INT_CLR_SPEC> {
        CH1_TX_LOOP_W::new(self, 13)
    }
    ///Bit 14 - Set this bit to clear theCH2_TX_LOOP_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop(&mut self) -> CH2_TX_LOOP_W<INT_CLR_SPEC> {
        CH2_TX_LOOP_W::new(self, 14)
    }
    ///Bit 15 - Set this bit to clear theCH3_TX_LOOP_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop(&mut self) -> CH3_TX_LOOP_W<INT_CLR_SPEC> {
        CH3_TX_LOOP_W::new(self, 15)
    }
    ///Bit 16 - Set this bit to clear theCH4_RX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH4_RX_END_W<INT_CLR_SPEC> {
        CH4_RX_END_W::new(self, 16)
    }
    ///Bit 17 - Set this bit to clear theCH5_RX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH5_RX_END_W<INT_CLR_SPEC> {
        CH5_RX_END_W::new(self, 17)
    }
    ///Bit 18 - Set this bit to clear theCH6_RX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH6_RX_END_W<INT_CLR_SPEC> {
        CH6_RX_END_W::new(self, 18)
    }
    ///Bit 19 - Set this bit to clear theCH7_RX_END_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH7_RX_END_W<INT_CLR_SPEC> {
        CH7_RX_END_W::new(self, 19)
    }
    ///Bit 20 - Set this bit to clear theCH4_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_ch4_err(&mut self) -> RX_CH4_ERR_W<INT_CLR_SPEC> {
        RX_CH4_ERR_W::new(self, 20)
    }
    ///Bit 21 - Set this bit to clear theCH5_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_ch5_err(&mut self) -> RX_CH5_ERR_W<INT_CLR_SPEC> {
        RX_CH5_ERR_W::new(self, 21)
    }
    ///Bit 22 - Set this bit to clear theCH6_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_ch6_err(&mut self) -> RX_CH6_ERR_W<INT_CLR_SPEC> {
        RX_CH6_ERR_W::new(self, 22)
    }
    ///Bit 23 - Set this bit to clear theCH7_ERR_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_err(&mut self) -> RX_CH7_ERR_W<INT_CLR_SPEC> {
        RX_CH7_ERR_W::new(self, 23)
    }
    ///Bit 24 - Set this bit to clear theCH4_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event(&mut self) -> CH4_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH4_RX_THR_EVENT_W::new(self, 24)
    }
    ///Bit 25 - Set this bit to clear theCH5_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event(&mut self) -> CH5_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH5_RX_THR_EVENT_W::new(self, 25)
    }
    ///Bit 26 - Set this bit to clear theCH6_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event(&mut self) -> CH6_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH6_RX_THR_EVENT_W::new(self, 26)
    }
    ///Bit 27 - Set this bit to clear theCH7_RX_THR_EVENT_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event(&mut self) -> CH7_RX_THR_EVENT_W<INT_CLR_SPEC> {
        CH7_RX_THR_EVENT_W::new(self, 27)
    }
    ///Bit 28 - Set this bit to clear the CH3_DMA_ACCESS_FAIL_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<INT_CLR_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self, 28)
    }
    ///Bit 29 - Set this bit to clear the CH7_DMA_ACCESS_FAIL_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<INT_CLR_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self, 29)
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
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x3fff_ffff;
}
///`reset()` method sets INT_CLR to value 0
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
