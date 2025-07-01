#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CH0_TX_END` reader - The interrupt enable bit for CH0_TX_END_INT."]
pub type CH0_TX_END_R = crate::BitReader;
#[doc = "Field `CH0_TX_END` writer - The interrupt enable bit for CH0_TX_END_INT."]
pub type CH0_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_END` reader - The interrupt enable bit for CH1_TX_END_INT."]
pub type CH1_TX_END_R = crate::BitReader;
#[doc = "Field `CH1_TX_END` writer - The interrupt enable bit for CH1_TX_END_INT."]
pub type CH1_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_END` reader - The interrupt enable bit for CH2_TX_END_INT."]
pub type CH2_TX_END_R = crate::BitReader;
#[doc = "Field `CH2_TX_END` writer - The interrupt enable bit for CH2_TX_END_INT."]
pub type CH2_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_END` reader - The interrupt enable bit for CH3_TX_END_INT."]
pub type CH3_TX_END_R = crate::BitReader;
#[doc = "Field `CH3_TX_END` writer - The interrupt enable bit for CH3_TX_END_INT."]
pub type CH3_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH0_ERR` reader - The interrupt enable bit for CH0_ERR_INT."]
pub type TX_CH0_ERR_R = crate::BitReader;
#[doc = "Field `TX_CH0_ERR` writer - The interrupt enable bit for CH0_ERR_INT."]
pub type TX_CH0_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH1_ERR` reader - The interrupt enable bit for CH1_ERR_INT."]
pub type TX_CH1_ERR_R = crate::BitReader;
#[doc = "Field `TX_CH1_ERR` writer - The interrupt enable bit for CH1_ERR_INT."]
pub type TX_CH1_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH2_ERR` reader - The interrupt enable bit for CH2_ERR_INT."]
pub type TX_CH2_ERR_R = crate::BitReader;
#[doc = "Field `TX_CH2_ERR` writer - The interrupt enable bit for CH2_ERR_INT."]
pub type TX_CH2_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_ERR` reader - The interrupt enable bit for CH3_ERR_INT."]
pub type TX_CH3_ERR_R = crate::BitReader;
#[doc = "Field `TX_CH3_ERR` writer - The interrupt enable bit for CH3_ERR_INT."]
pub type TX_CH3_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_THR_EVENT` reader - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
pub type CH0_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH0_TX_THR_EVENT` writer - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
pub type CH0_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_THR_EVENT` reader - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
pub type CH1_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH1_TX_THR_EVENT` writer - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
pub type CH1_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_THR_EVENT` reader - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
pub type CH2_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH2_TX_THR_EVENT` writer - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
pub type CH2_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_THR_EVENT` reader - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
pub type CH3_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH3_TX_THR_EVENT` writer - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
pub type CH3_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_LOOP` reader - The interrupt enable bit for CH0_TX_LOOP_INT."]
pub type CH0_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH0_TX_LOOP` writer - The interrupt enable bit for CH0_TX_LOOP_INT."]
pub type CH0_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_LOOP` reader - The interrupt enable bit for CH1_TX_LOOP_INT."]
pub type CH1_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH1_TX_LOOP` writer - The interrupt enable bit for CH1_TX_LOOP_INT."]
pub type CH1_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_LOOP` reader - The interrupt enable bit for CH2_TX_LOOP_INT."]
pub type CH2_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH2_TX_LOOP` writer - The interrupt enable bit for CH2_TX_LOOP_INT."]
pub type CH2_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_LOOP` reader - The interrupt enable bit for CH3_TX_LOOP_INT."]
pub type CH3_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH3_TX_LOOP` writer - The interrupt enable bit for CH3_TX_LOOP_INT."]
pub type CH3_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_END` reader - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH4_RX_END_R = crate::BitReader;
#[doc = "Field `CH4_RX_END` writer - The interrupt enable bit for CH4_RX_END_INT."]
pub type CH4_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_END` reader - The interrupt enable bit for CH5_RX_END_INT."]
pub type CH5_RX_END_R = crate::BitReader;
#[doc = "Field `CH5_RX_END` writer - The interrupt enable bit for CH5_RX_END_INT."]
pub type CH5_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_END` reader - The interrupt enable bit for CH6_RX_END_INT."]
pub type CH6_RX_END_R = crate::BitReader;
#[doc = "Field `CH6_RX_END` writer - The interrupt enable bit for CH6_RX_END_INT."]
pub type CH6_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_END` reader - The interrupt enable bit for CH7_RX_END_INT."]
pub type CH7_RX_END_R = crate::BitReader;
#[doc = "Field `CH7_RX_END` writer - The interrupt enable bit for CH7_RX_END_INT."]
pub type CH7_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_ERR` reader - The interrupt enable bit for CH4_ERR_INT."]
pub type CH4_ERR_R = crate::BitReader;
#[doc = "Field `CH4_ERR` writer - The interrupt enable bit for CH4_ERR_INT."]
pub type CH4_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_ERR` reader - The interrupt enable bit for CH5_ERR_INT."]
pub type CH5_ERR_R = crate::BitReader;
#[doc = "Field `CH5_ERR` writer - The interrupt enable bit for CH5_ERR_INT."]
pub type CH5_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_ERR` reader - The interrupt enable bit for CH6_ERR_INT."]
pub type CH6_ERR_R = crate::BitReader;
#[doc = "Field `CH6_ERR` writer - The interrupt enable bit for CH6_ERR_INT."]
pub type CH6_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_ERR` reader - The interrupt enable bit for CH7_ERR_INT."]
pub type CH7_ERR_R = crate::BitReader;
#[doc = "Field `CH7_ERR` writer - The interrupt enable bit for CH7_ERR_INT."]
pub type CH7_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_THR_EVENT` reader - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH4_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT` writer - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
pub type CH4_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_THR_EVENT` reader - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
pub type CH5_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT` writer - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
pub type CH5_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_THR_EVENT` reader - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
pub type CH6_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT` writer - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
pub type CH6_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_THR_EVENT` reader - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
pub type CH7_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT` writer - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
pub type CH7_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
pub type TX_CH3_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` reader - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL` writer - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
pub type RX_CH7_DMA_ACCESS_FAIL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH0_TX_END_R {
        CH0_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH1_TX_END_R {
        CH1_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH2_TX_END_R {
        CH2_TX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH3_TX_END_R {
        CH3_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch0_err(&self) -> TX_CH0_ERR_R {
        TX_CH0_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch1_err(&self) -> TX_CH1_ERR_R {
        TX_CH1_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch2_err(&self) -> TX_CH2_ERR_R {
        TX_CH2_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch3_err(&self) -> TX_CH3_ERR_R {
        TX_CH3_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH0_TX_THR_EVENT_R {
        CH0_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH1_TX_THR_EVENT_R {
        CH1_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH2_TX_THR_EVENT_R {
        CH2_TX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH3_TX_THR_EVENT_R {
        CH3_TX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH0_TX_LOOP_R {
        CH0_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH1_TX_LOOP_R {
        CH1_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH2_TX_LOOP_R {
        CH2_TX_LOOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH3_TX_LOOP_R {
        CH3_TX_LOOP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> CH4_RX_END_R {
        CH4_RX_END_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH5_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> CH5_RX_END_R {
        CH5_RX_END_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH6_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> CH6_RX_END_R {
        CH6_RX_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH7_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> CH7_RX_END_R {
        CH7_RX_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_err(&self) -> CH4_ERR_R {
        CH4_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH5_ERR_INT."]
    #[inline(always)]
    pub fn ch5_err(&self) -> CH5_ERR_R {
        CH5_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch6_err(&self) -> CH6_ERR_R {
        CH6_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH7_ERR_INT."]
    #[inline(always)]
    pub fn ch7_err(&self) -> CH7_ERR_R {
        CH7_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&self) -> CH4_RX_THR_EVENT_R {
        CH4_RX_THR_EVENT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&self) -> CH5_RX_THR_EVENT_R {
        CH5_RX_THR_EVENT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&self) -> CH6_RX_THR_EVENT_R {
        CH6_RX_THR_EVENT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&self) -> CH7_RX_THR_EVENT_R {
        CH7_RX_THR_EVENT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail(&self) -> TX_CH3_DMA_ACCESS_FAIL_R {
        TX_CH3_DMA_ACCESS_FAIL_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
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
    #[doc = "Bit 0 - The interrupt enable bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&mut self) -> CH0_TX_END_W<INT_ENA_SPEC> {
        CH0_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt enable bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&mut self) -> CH1_TX_END_W<INT_ENA_SPEC> {
        CH1_TX_END_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt enable bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&mut self) -> CH2_TX_END_W<INT_ENA_SPEC> {
        CH2_TX_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt enable bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&mut self) -> CH3_TX_END_W<INT_ENA_SPEC> {
        CH3_TX_END_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt enable bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch0_err(&mut self) -> TX_CH0_ERR_W<INT_ENA_SPEC> {
        TX_CH0_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt enable bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch1_err(&mut self) -> TX_CH1_ERR_W<INT_ENA_SPEC> {
        TX_CH1_ERR_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt enable bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch2_err(&mut self) -> TX_CH2_ERR_W<INT_ENA_SPEC> {
        TX_CH2_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt enable bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn tx_ch3_err(&mut self) -> TX_CH3_ERR_W<INT_ENA_SPEC> {
        TX_CH3_ERR_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt enable bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&mut self) -> CH0_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH0_TX_THR_EVENT_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt enable bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&mut self) -> CH1_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH1_TX_THR_EVENT_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt enable bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&mut self) -> CH2_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH2_TX_THR_EVENT_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt enable bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&mut self) -> CH3_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH3_TX_THR_EVENT_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt enable bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&mut self) -> CH0_TX_LOOP_W<INT_ENA_SPEC> {
        CH0_TX_LOOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enable bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&mut self) -> CH1_TX_LOOP_W<INT_ENA_SPEC> {
        CH1_TX_LOOP_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enable bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&mut self) -> CH2_TX_LOOP_W<INT_ENA_SPEC> {
        CH2_TX_LOOP_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enable bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&mut self) -> CH3_TX_LOOP_W<INT_ENA_SPEC> {
        CH3_TX_LOOP_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt enable bit for CH4_RX_END_INT."]
    #[inline(always)]
    pub fn ch4_rx_end(&mut self) -> CH4_RX_END_W<INT_ENA_SPEC> {
        CH4_RX_END_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enable bit for CH5_RX_END_INT."]
    #[inline(always)]
    pub fn ch5_rx_end(&mut self) -> CH5_RX_END_W<INT_ENA_SPEC> {
        CH5_RX_END_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt enable bit for CH6_RX_END_INT."]
    #[inline(always)]
    pub fn ch6_rx_end(&mut self) -> CH6_RX_END_W<INT_ENA_SPEC> {
        CH6_RX_END_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt enable bit for CH7_RX_END_INT."]
    #[inline(always)]
    pub fn ch7_rx_end(&mut self) -> CH7_RX_END_W<INT_ENA_SPEC> {
        CH7_RX_END_W::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt enable bit for CH4_ERR_INT."]
    #[inline(always)]
    pub fn ch4_err(&mut self) -> CH4_ERR_W<INT_ENA_SPEC> {
        CH4_ERR_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt enable bit for CH5_ERR_INT."]
    #[inline(always)]
    pub fn ch5_err(&mut self) -> CH5_ERR_W<INT_ENA_SPEC> {
        CH5_ERR_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt enable bit for CH6_ERR_INT."]
    #[inline(always)]
    pub fn ch6_err(&mut self) -> CH6_ERR_W<INT_ENA_SPEC> {
        CH6_ERR_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt enable bit for CH7_ERR_INT."]
    #[inline(always)]
    pub fn ch7_err(&mut self) -> CH7_ERR_W<INT_ENA_SPEC> {
        CH7_ERR_W::new(self, 23)
    }
    #[doc = "Bit 24 - The interrupt enable bit for CH4_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch4_rx_thr_event(&mut self) -> CH4_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH4_RX_THR_EVENT_W::new(self, 24)
    }
    #[doc = "Bit 25 - The interrupt enable bit for CH5_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch5_rx_thr_event(&mut self) -> CH5_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH5_RX_THR_EVENT_W::new(self, 25)
    }
    #[doc = "Bit 26 - The interrupt enable bit for CH6_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch6_rx_thr_event(&mut self) -> CH6_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH6_RX_THR_EVENT_W::new(self, 26)
    }
    #[doc = "Bit 27 - The interrupt enable bit for CH7_RX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch7_rx_thr_event(&mut self) -> CH7_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH7_RX_THR_EVENT_W::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt enable bit for CH3_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail(&mut self) -> TX_CH3_DMA_ACCESS_FAIL_W<INT_ENA_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_W::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt enable bit for CH7_DMA_ACCESS_FAIL_INT."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail(&mut self) -> RX_CH7_DMA_ACCESS_FAIL_W<INT_ENA_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_W::new(self, 29)
    }
}
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::Reg::read) this register and get [`int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {}
