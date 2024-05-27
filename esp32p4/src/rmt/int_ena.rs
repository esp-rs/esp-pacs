///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `CH0_TX_END` reader - The interrupt enable bit for CH0_TX_END_INT.
pub type CH0_TX_END_R = crate::BitReader;
///Field `CH0_TX_END` writer - The interrupt enable bit for CH0_TX_END_INT.
pub type CH0_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_TX_END` reader - The interrupt enable bit for CH1_TX_END_INT.
pub type CH1_TX_END_R = crate::BitReader;
///Field `CH1_TX_END` writer - The interrupt enable bit for CH1_TX_END_INT.
pub type CH1_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2_TX_END` reader - The interrupt enable bit for CH2_TX_END_INT.
pub type CH2_TX_END_R = crate::BitReader;
///Field `CH2_TX_END` writer - The interrupt enable bit for CH2_TX_END_INT.
pub type CH2_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3_TX_END` reader - The interrupt enable bit for CH3_TX_END_INT.
pub type CH3_TX_END_R = crate::BitReader;
///Field `CH3_TX_END` writer - The interrupt enable bit for CH3_TX_END_INT.
pub type CH3_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CH0_ERR` reader - The interrupt enable bit for CH0_ERR_INT.
pub type TX_CH0_ERR_R = crate::BitReader;
///Field `TX_CH0_ERR` writer - The interrupt enable bit for CH0_ERR_INT.
pub type TX_CH0_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CH1_ERR` reader - The interrupt enable bit for CH1_ERR_INT.
pub type TX_CH1_ERR_R = crate::BitReader;
///Field `TX_CH1_ERR` writer - The interrupt enable bit for CH1_ERR_INT.
pub type TX_CH1_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CH2_ERR` reader - The interrupt enable bit for CH2_ERR_INT.
pub type TX_CH2_ERR_R = crate::BitReader;
///Field `TX_CH2_ERR` writer - The interrupt enable bit for CH2_ERR_INT.
pub type TX_CH2_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CH3_ERR` reader - The interrupt enable bit for CH3_ERR_INT.
pub type TX_CH3_ERR_R = crate::BitReader;
///Field `TX_CH3_ERR` writer - The interrupt enable bit for CH3_ERR_INT.
pub type TX_CH3_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0_TX_THR_EVENT` reader - The interrupt enable bit for CH0_TX_THR_EVENT_INT.
pub type CH0_TX_THR_EVENT_R = crate::BitReader;
///Field `CH0_TX_THR_EVENT` writer - The interrupt enable bit for CH0_TX_THR_EVENT_INT.
pub type CH0_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_TX_THR_EVENT` reader - The interrupt enable bit for CH1_TX_THR_EVENT_INT.
pub type CH1_TX_THR_EVENT_R = crate::BitReader;
///Field `CH1_TX_THR_EVENT` writer - The interrupt enable bit for CH1_TX_THR_EVENT_INT.
pub type CH1_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2_TX_THR_EVENT` reader - The interrupt enable bit for CH2_TX_THR_EVENT_INT.
pub type CH2_TX_THR_EVENT_R = crate::BitReader;
///Field `CH2_TX_THR_EVENT` writer - The interrupt enable bit for CH2_TX_THR_EVENT_INT.
pub type CH2_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3_TX_THR_EVENT` reader - The interrupt enable bit for CH3_TX_THR_EVENT_INT.
pub type CH3_TX_THR_EVENT_R = crate::BitReader;
///Field `CH3_TX_THR_EVENT` writer - The interrupt enable bit for CH3_TX_THR_EVENT_INT.
pub type CH3_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH0_TX_LOOP` reader - The interrupt enable bit for CH0_TX_LOOP_INT.
pub type CH0_TX_LOOP_R = crate::BitReader;
///Field `CH0_TX_LOOP` writer - The interrupt enable bit for CH0_TX_LOOP_INT.
pub type CH0_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH1_TX_LOOP` reader - The interrupt enable bit for CH1_TX_LOOP_INT.
pub type CH1_TX_LOOP_R = crate::BitReader;
///Field `CH1_TX_LOOP` writer - The interrupt enable bit for CH1_TX_LOOP_INT.
pub type CH1_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH2_TX_LOOP` reader - The interrupt enable bit for CH2_TX_LOOP_INT.
pub type CH2_TX_LOOP_R = crate::BitReader;
///Field `CH2_TX_LOOP` writer - The interrupt enable bit for CH2_TX_LOOP_INT.
pub type CH2_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH3_TX_LOOP` reader - The interrupt enable bit for CH3_TX_LOOP_INT.
pub type CH3_TX_LOOP_R = crate::BitReader;
///Field `CH3_TX_LOOP` writer - The interrupt enable bit for CH3_TX_LOOP_INT.
pub type CH3_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH4_RX_END` reader - The interrupt enable bit for CH4_RX_END_INT.
pub type CH4_RX_END_R = crate::BitReader;
///Field `CH4_RX_END` writer - The interrupt enable bit for CH4_RX_END_INT.
pub type CH4_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH5_RX_END` reader - The interrupt enable bit for CH5_RX_END_INT.
pub type CH5_RX_END_R = crate::BitReader;
///Field `CH5_RX_END` writer - The interrupt enable bit for CH5_RX_END_INT.
pub type CH5_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH6_RX_END` reader - The interrupt enable bit for CH6_RX_END_INT.
pub type CH6_RX_END_R = crate::BitReader;
///Field `CH6_RX_END` writer - The interrupt enable bit for CH6_RX_END_INT.
pub type CH6_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH7_RX_END` reader - The interrupt enable bit for CH7_RX_END_INT.
pub type CH7_RX_END_R = crate::BitReader;
///Field `CH7_RX_END` writer - The interrupt enable bit for CH7_RX_END_INT.
pub type CH7_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH4_ERR` reader - The interrupt enable bit for CH4_ERR_INT.
pub type CH4_ERR_R = crate::BitReader;
///Field `CH4_ERR` writer - The interrupt enable bit for CH4_ERR_INT.
pub type CH4_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH5_ERR` reader - The interrupt enable bit for CH5_ERR_INT.
pub type CH5_ERR_R = crate::BitReader;
///Field `CH5_ERR` writer - The interrupt enable bit for CH5_ERR_INT.
pub type CH5_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH6_ERR` reader - The interrupt enable bit for CH6_ERR_INT.
pub type CH6_ERR_R = crate::BitReader;
///Field `CH6_ERR` writer - The interrupt enable bit for CH6_ERR_INT.
pub type CH6_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH7_ERR` reader - The interrupt enable bit for CH7_ERR_INT.
pub type CH7_ERR_R = crate::BitReader;
///Field `CH7_ERR` writer - The interrupt enable bit for CH7_ERR_INT.
pub type CH7_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH4_RX_THR_EVENT` reader - The interrupt enable bit for CH4_RX_THR_EVENT_INT.
pub type CH4_RX_THR_EVENT_R = crate::BitReader;
///Field `CH4_RX_THR_EVENT` writer - The interrupt enable bit for CH4_RX_THR_EVENT_INT.
pub type CH4_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH5_RX_THR_EVENT` reader - The interrupt enable bit for CH5_RX_THR_EVENT_INT.
pub type CH5_RX_THR_EVENT_R = crate::BitReader;
///Field `CH5_RX_THR_EVENT` writer - The interrupt enable bit for CH5_RX_THR_EVENT_INT.
pub type CH5_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH6_RX_THR_EVENT` reader - The interrupt enable bit for CH6_RX_THR_EVENT_INT.
pub type CH6_RX_THR_EVENT_R = crate::BitReader;
///Field `CH6_RX_THR_EVENT` writer - The interrupt enable bit for CH6_RX_THR_EVENT_INT.
pub type CH6_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CH7_RX_THR_EVENT` reader - The interrupt enable bit for CH7_RX_THR_EVENT_INT.
pub type CH7_RX_THR_EVENT_R = crate::BitReader;
///Field `CH7_RX_THR_EVENT` writer - The interrupt enable bit for CH7_RX_THR_EVENT_INT.
pub type CH7_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `TX_CH3_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
pub type TX_CH3_DMA_ACCESS_FAIL_R = crate::BitReader;
///Field `TX_CH3_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `RX_CH7_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
pub type RX_CH7_DMA_ACCESS_FAIL_R = crate::BitReader;
///Field `RX_CH7_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The interrupt enable bit for CH0_TX_END_INT.
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH0_TX_END_R {
        CH0_TX_END_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The interrupt enable bit for CH1_TX_END_INT.
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH1_TX_END_R {
        CH1_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The interrupt enable bit for CH2_TX_END_INT.
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH2_TX_END_R {
        CH2_TX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The interrupt enable bit for CH3_TX_END_INT.
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH3_TX_END_R {
        CH3_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The interrupt enable bit for CH0_ERR_INT.
    #[inline(always)]
    pub fn tx_ch0_err(&self) -> TX_CH0_ERR_R {
        TX_CH0_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - The interrupt enable bit for CH1_ERR_INT.
    #[inline(always)]
    pub fn tx_ch1_err(&self) -> TX_CH1_ERR_R {
        TX_CH1_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - The interrupt enable bit for CH2_ERR_INT.
    #[inline(always)]
    pub fn tx_ch2_err(&self) -> TX_CH2_ERR_R {
        TX_CH2_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - The interrupt enable bit for CH3_ERR_INT.
    #[inline(always)]
    pub fn tx_ch3_err(&self) -> TX_CH3_ERR_R {
        TX_CH3_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH0_TX_THR_EVENT_R {
        CH0_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH1_TX_THR_EVENT_R {
        CH1_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH2_TX_THR_EVENT_R {
        CH2_TX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH3_TX_THR_EVENT_R {
        CH3_TX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    ///Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH0_TX_LOOP_R {
        CH0_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    ///Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH1_TX_LOOP_R {
        CH1_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    ///Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH2_TX_LOOP_R {
        CH2_TX_LOOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    ///Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT.
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH3_TX_LOOP_R {
        CH3_TX_LOOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    ///Bit 16 - The interrupt enable bit for CH4_RX_END_INT.
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> CH4_RX_END_R {
        CH4_RX_END_R::new(((self.bits >> 16) & 1) != 0)
    }
    ///Bit 17 - The interrupt enable bit for CH5_RX_END_INT.
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> CH5_RX_END_R {
        CH5_RX_END_R::new(((self.bits >> 17) & 1) != 0)
    }
    ///Bit 18 - The interrupt enable bit for CH6_RX_END_INT.
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> CH6_RX_END_R {
        CH6_RX_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    ///Bit 19 - The interrupt enable bit for CH7_RX_END_INT.
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> CH7_RX_END_R {
        CH7_RX_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    ///Bit 20 - The interrupt enable bit for CH4_ERR_INT.
    #[inline(always)]
    pub fn ch4_err(&self) -> CH4_ERR_R {
        CH4_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    ///Bit 21 - The interrupt enable bit for CH5_ERR_INT.
    #[inline(always)]
    pub fn ch5_err(&self) -> CH5_ERR_R {
        CH5_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - The interrupt enable bit for CH6_ERR_INT.
    #[inline(always)]
    pub fn ch6_err(&self) -> CH6_ERR_R {
        CH6_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - The interrupt enable bit for CH7_ERR_INT.
    #[inline(always)]
    pub fn ch7_err(&self) -> CH7_ERR_R {
        CH7_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch4_rx_thr_event(&self) -> CH4_RX_THR_EVENT_R {
        CH4_RX_THR_EVENT_R::new(((self.bits >> 24) & 1) != 0)
    }
    ///Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch5_rx_thr_event(&self) -> CH5_RX_THR_EVENT_R {
        CH5_RX_THR_EVENT_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch6_rx_thr_event(&self) -> CH6_RX_THR_EVENT_R {
        CH6_RX_THR_EVENT_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT.
    #[inline(always)]
    pub fn ch7_rx_thr_event(&self) -> CH7_RX_THR_EVENT_R {
        CH7_RX_THR_EVENT_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail(&self) -> TX_CH3_DMA_ACCESS_FAIL_R {
        TX_CH3_DMA_ACCESS_FAIL_R::new(((self.bits >> 28) & 1) != 0)
    }
    ///Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail(&self) -> RX_CH7_DMA_ACCESS_FAIL_R {
        RX_CH7_DMA_ACCESS_FAIL_R::new(((self.bits >> 29) & 1) != 0)
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
            .field("tx_ch0_err", &self.tx_ch0_err())
            .field("tx_ch1_err", &self.tx_ch1_err())
            .field("tx_ch2_err", &self.tx_ch2_err())
            .field("tx_ch3_err", &self.tx_ch3_err())
            .field("ch0_tx_thr_event", &self.ch0_tx_thr_event())
            .field("ch1_tx_thr_event", &self.ch1_tx_thr_event())
            .field("ch2_tx_thr_event", &self.ch2_tx_thr_event())
            .field("ch3_tx_thr_event", &self.ch3_tx_thr_event())
            .field("ch0_tx_loop", &self.ch0_tx_loop())
            .field("ch1_tx_loop", &self.ch1_tx_loop())
            .field("ch2_tx_loop", &self.ch2_tx_loop())
            .field("ch3_tx_loop", &self.ch3_tx_loop())
            .field("ch4_rx_end", &self.ch4_rx_end())
            .field("ch5_rx_end", &self.ch5_rx_end())
            .field("ch6_rx_end", &self.ch6_rx_end())
            .field("ch7_rx_end", &self.ch7_rx_end())
            .field("ch4_err", &self.ch4_err())
            .field("ch5_err", &self.ch5_err())
            .field("ch6_err", &self.ch6_err())
            .field("ch7_err", &self.ch7_err())
            .field("ch4_rx_thr_event", &self.ch4_rx_thr_event())
            .field("ch5_rx_thr_event", &self.ch5_rx_thr_event())
            .field("ch6_rx_thr_event", &self.ch6_rx_thr_event())
            .field("ch7_rx_thr_event", &self.ch7_rx_thr_event())
            .field("tx_ch3_dma_access_fail", &self.tx_ch3_dma_access_fail())
            .field("rx_ch7_dma_access_fail", &self.rx_ch7_dma_access_fail())
            .finish()
    }
}
impl W {
    ///Bit 0 - The interrupt enable bit for CH0_TX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH0_TX_END_W<INT_ENA_SPEC> {
        CH0_TX_END_W::new(self, 0)
    }
    ///Bit 1 - The interrupt enable bit for CH1_TX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH1_TX_END_W<INT_ENA_SPEC> {
        CH1_TX_END_W::new(self, 1)
    }
    ///Bit 2 - The interrupt enable bit for CH2_TX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH2_TX_END_W<INT_ENA_SPEC> {
        CH2_TX_END_W::new(self, 2)
    }
    ///Bit 3 - The interrupt enable bit for CH3_TX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH3_TX_END_W<INT_ENA_SPEC> {
        CH3_TX_END_W::new(self, 3)
    }
    ///Bit 4 - The interrupt enable bit for CH0_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch0_err(&mut self) -> TX_CH0_ERR_W<INT_ENA_SPEC> {
        TX_CH0_ERR_W::new(self, 4)
    }
    ///Bit 5 - The interrupt enable bit for CH1_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch1_err(&mut self) -> TX_CH1_ERR_W<INT_ENA_SPEC> {
        TX_CH1_ERR_W::new(self, 5)
    }
    ///Bit 6 - The interrupt enable bit for CH2_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch2_err(&mut self) -> TX_CH2_ERR_W<INT_ENA_SPEC> {
        TX_CH2_ERR_W::new(self, 6)
    }
    ///Bit 7 - The interrupt enable bit for CH3_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_err(&mut self) -> TX_CH3_ERR_W<INT_ENA_SPEC> {
        TX_CH3_ERR_W::new(self, 7)
    }
    ///Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH0_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH0_TX_THR_EVENT_W::new(self, 8)
    }
    ///Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH1_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH1_TX_THR_EVENT_W::new(self, 9)
    }
    ///Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH2_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH2_TX_THR_EVENT_W::new(self, 10)
    }
    ///Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH3_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH3_TX_THR_EVENT_W::new(self, 11)
    }
    ///Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH0_TX_LOOP_W<INT_ENA_SPEC> {
        CH0_TX_LOOP_W::new(self, 12)
    }
    ///Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH1_TX_LOOP_W<INT_ENA_SPEC> {
        CH1_TX_LOOP_W::new(self, 13)
    }
    ///Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop(&mut self) -> CH2_TX_LOOP_W<INT_ENA_SPEC> {
        CH2_TX_LOOP_W::new(self, 14)
    }
    ///Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop(&mut self) -> CH3_TX_LOOP_W<INT_ENA_SPEC> {
        CH3_TX_LOOP_W::new(self, 15)
    }
    ///Bit 16 - The interrupt enable bit for CH4_RX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end(&mut self) -> CH4_RX_END_W<INT_ENA_SPEC> {
        CH4_RX_END_W::new(self, 16)
    }
    ///Bit 17 - The interrupt enable bit for CH5_RX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end(&mut self) -> CH5_RX_END_W<INT_ENA_SPEC> {
        CH5_RX_END_W::new(self, 17)
    }
    ///Bit 18 - The interrupt enable bit for CH6_RX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end(&mut self) -> CH6_RX_END_W<INT_ENA_SPEC> {
        CH6_RX_END_W::new(self, 18)
    }
    ///Bit 19 - The interrupt enable bit for CH7_RX_END_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end(&mut self) -> CH7_RX_END_W<INT_ENA_SPEC> {
        CH7_RX_END_W::new(self, 19)
    }
    ///Bit 20 - The interrupt enable bit for CH4_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch4_err(&mut self) -> CH4_ERR_W<INT_ENA_SPEC> {
        CH4_ERR_W::new(self, 20)
    }
    ///Bit 21 - The interrupt enable bit for CH5_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch5_err(&mut self) -> CH5_ERR_W<INT_ENA_SPEC> {
        CH5_ERR_W::new(self, 21)
    }
    ///Bit 22 - The interrupt enable bit for CH6_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch6_err(&mut self) -> CH6_ERR_W<INT_ENA_SPEC> {
        CH6_ERR_W::new(self, 22)
    }
    ///Bit 23 - The interrupt enable bit for CH7_ERR_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch7_err(&mut self) -> CH7_ERR_W<INT_ENA_SPEC> {
        CH7_ERR_W::new(self, 23)
    }
    ///Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event(&mut self) -> CH4_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH4_RX_THR_EVENT_W::new(self, 24)
    }
    ///Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event(&mut self) -> CH5_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH5_RX_THR_EVENT_W::new(self, 25)
    }
    ///Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event(&mut self) -> CH6_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH6_RX_THR_EVENT_W::new(self, 26)
    }
    ///Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT.
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event(&mut self) -> CH7_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH7_RX_THR_EVENT_W::new(self, 27)
    }
    ///Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<INT_ENA_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self, 28)
    }
    ///Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT.
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<INT_ENA_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self, 29)
    }
}
/**Interrupt enable bits

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
