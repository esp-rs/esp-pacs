#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `CH0_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
pub type CH0_TX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH0_TX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
pub type CH0_TX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
pub type CH1_TX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH1_TX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
pub type CH1_TX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
pub type CH2_TX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH2_TX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
pub type CH2_TX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
pub type CH3_TX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH3_TX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
pub type CH3_TX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH0_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
pub type TX_CH0_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_CH0_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
pub type TX_CH0_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH1_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
pub type TX_CH1_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_CH1_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
pub type TX_CH1_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH2_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
pub type TX_CH2_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_CH2_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
pub type TX_CH2_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
pub type TX_CH3_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_CH3_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
pub type TX_CH3_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
pub type CH0_TX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH0_TX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
pub type CH0_TX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
pub type CH1_TX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH1_TX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
pub type CH1_TX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
pub type CH2_TX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH2_TX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
pub type CH2_TX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
pub type CH3_TX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH3_TX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
pub type CH3_TX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH0_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
pub type CH0_TX_LOOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH0_TX_LOOP_INT_RAW` writer - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
pub type CH0_TX_LOOP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
pub type CH1_TX_LOOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH1_TX_LOOP_INT_RAW` writer - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
pub type CH1_TX_LOOP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH2_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
pub type CH2_TX_LOOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH2_TX_LOOP_INT_RAW` writer - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
pub type CH2_TX_LOOP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH3_TX_LOOP_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
pub type CH3_TX_LOOP_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH3_TX_LOOP_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
pub type CH3_TX_LOOP_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type CH4_RX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH4_RX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type CH4_RX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
pub type CH5_RX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH5_RX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
pub type CH5_RX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
pub type CH6_RX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH6_RX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
pub type CH6_RX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_END_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
pub type CH7_RX_END_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH7_RX_END_INT_RAW` writer - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
pub type CH7_RX_END_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH4_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type RX_CH4_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_CH4_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type RX_CH4_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH5_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
pub type RX_CH5_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_CH5_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
pub type RX_CH5_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH6_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type RX_CH6_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_CH6_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type RX_CH6_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH7_ERR_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
pub type RX_CH7_ERR_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_CH7_ERR_INT_RAW` writer - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
pub type RX_CH7_ERR_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH4_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type CH4_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH4_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type CH4_RX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH5_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
pub type CH5_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH5_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
pub type CH5_RX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH6_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
pub type CH6_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH6_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
pub type CH6_RX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH7_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
pub type CH7_RX_THR_EVENT_INT_RAW_R = crate::BitReader;
#[doc = "Field `CH7_RX_THR_EVENT_INT_RAW` writer - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
pub type CH7_RX_THR_EVENT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_RAW` writer - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_RAW_R = crate::BitReader;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_RAW` writer - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH0_TX_END_INT_RAW_R {
        CH0_TX_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH1_TX_END_INT_RAW_R {
        CH1_TX_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch2_tx_end_int_raw(&self) -> CH2_TX_END_INT_RAW_R {
        CH2_TX_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch3_tx_end_int_raw(&self) -> CH3_TX_END_INT_RAW_R {
        CH3_TX_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    pub fn tx_ch0_err_int_raw(&self) -> TX_CH0_ERR_INT_RAW_R {
        TX_CH0_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    pub fn tx_ch1_err_int_raw(&self) -> TX_CH1_ERR_INT_RAW_R {
        TX_CH1_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    pub fn tx_ch2_err_int_raw(&self) -> TX_CH2_ERR_INT_RAW_R {
        TX_CH2_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    pub fn tx_ch3_err_int_raw(&self) -> TX_CH3_ERR_INT_RAW_R {
        TX_CH3_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH0_TX_THR_EVENT_INT_RAW_R {
        CH0_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH1_TX_THR_EVENT_INT_RAW_R {
        CH1_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_raw(&self) -> CH2_TX_THR_EVENT_INT_RAW_R {
        CH2_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_raw(&self) -> CH3_TX_THR_EVENT_INT_RAW_R {
        CH3_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> CH0_TX_LOOP_INT_RAW_R {
        CH0_TX_LOOP_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> CH1_TX_LOOP_INT_RAW_R {
        CH1_TX_LOOP_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_raw(&self) -> CH2_TX_LOOP_INT_RAW_R {
        CH2_TX_LOOP_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_raw(&self) -> CH3_TX_LOOP_INT_RAW_R {
        CH3_TX_LOOP_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch4_rx_end_int_raw(&self) -> CH4_RX_END_INT_RAW_R {
        CH4_RX_END_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
    #[inline(always)]
    pub fn ch5_rx_end_int_raw(&self) -> CH5_RX_END_INT_RAW_R {
        CH5_RX_END_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
    #[inline(always)]
    pub fn ch6_rx_end_int_raw(&self) -> CH6_RX_END_INT_RAW_R {
        CH6_RX_END_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
    #[inline(always)]
    pub fn ch7_rx_end_int_raw(&self) -> CH7_RX_END_INT_RAW_R {
        CH7_RX_END_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn rx_ch4_err_int_raw(&self) -> RX_CH4_ERR_INT_RAW_R {
        RX_CH4_ERR_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
    #[inline(always)]
    pub fn rx_ch5_err_int_raw(&self) -> RX_CH5_ERR_INT_RAW_R {
        RX_CH5_ERR_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn rx_ch6_err_int_raw(&self) -> RX_CH6_ERR_INT_RAW_R {
        RX_CH6_ERR_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
    #[inline(always)]
    pub fn rx_ch7_err_int_raw(&self) -> RX_CH7_ERR_INT_RAW_R {
        RX_CH7_ERR_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch4_rx_thr_event_int_raw(&self) -> CH4_RX_THR_EVENT_INT_RAW_R {
        CH4_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch5_rx_thr_event_int_raw(&self) -> CH5_RX_THR_EVENT_INT_RAW_R {
        CH5_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch6_rx_thr_event_int_raw(&self) -> CH6_RX_THR_EVENT_INT_RAW_R {
        CH6_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch7_rx_thr_event_int_raw(&self) -> CH7_RX_THR_EVENT_INT_RAW_R {
        CH7_RX_THR_EVENT_INT_RAW_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    pub fn tx_ch3_dma_access_fail_int_raw(&self) -> TX_CH3_DMA_ACCESS_FAIL_INT_RAW_R {
        TX_CH3_DMA_ACCESS_FAIL_INT_RAW_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    pub fn rx_ch7_dma_access_fail_int_raw(&self) -> RX_CH7_DMA_ACCESS_FAIL_INT_RAW_R {
        RX_CH7_DMA_ACCESS_FAIL_INT_RAW_R::new(((self.bits >> 29) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field(
                "ch0_tx_end_int_raw",
                &format_args!("{}", self.ch0_tx_end_int_raw().bit()),
            )
            .field(
                "ch1_tx_end_int_raw",
                &format_args!("{}", self.ch1_tx_end_int_raw().bit()),
            )
            .field(
                "ch2_tx_end_int_raw",
                &format_args!("{}", self.ch2_tx_end_int_raw().bit()),
            )
            .field(
                "ch3_tx_end_int_raw",
                &format_args!("{}", self.ch3_tx_end_int_raw().bit()),
            )
            .field(
                "tx_ch0_err_int_raw",
                &format_args!("{}", self.tx_ch0_err_int_raw().bit()),
            )
            .field(
                "tx_ch1_err_int_raw",
                &format_args!("{}", self.tx_ch1_err_int_raw().bit()),
            )
            .field(
                "tx_ch2_err_int_raw",
                &format_args!("{}", self.tx_ch2_err_int_raw().bit()),
            )
            .field(
                "tx_ch3_err_int_raw",
                &format_args!("{}", self.tx_ch3_err_int_raw().bit()),
            )
            .field(
                "ch0_tx_thr_event_int_raw",
                &format_args!("{}", self.ch0_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch1_tx_thr_event_int_raw",
                &format_args!("{}", self.ch1_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch2_tx_thr_event_int_raw",
                &format_args!("{}", self.ch2_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch3_tx_thr_event_int_raw",
                &format_args!("{}", self.ch3_tx_thr_event_int_raw().bit()),
            )
            .field(
                "ch0_tx_loop_int_raw",
                &format_args!("{}", self.ch0_tx_loop_int_raw().bit()),
            )
            .field(
                "ch1_tx_loop_int_raw",
                &format_args!("{}", self.ch1_tx_loop_int_raw().bit()),
            )
            .field(
                "ch2_tx_loop_int_raw",
                &format_args!("{}", self.ch2_tx_loop_int_raw().bit()),
            )
            .field(
                "ch3_tx_loop_int_raw",
                &format_args!("{}", self.ch3_tx_loop_int_raw().bit()),
            )
            .field(
                "ch4_rx_end_int_raw",
                &format_args!("{}", self.ch4_rx_end_int_raw().bit()),
            )
            .field(
                "ch5_rx_end_int_raw",
                &format_args!("{}", self.ch5_rx_end_int_raw().bit()),
            )
            .field(
                "ch6_rx_end_int_raw",
                &format_args!("{}", self.ch6_rx_end_int_raw().bit()),
            )
            .field(
                "ch7_rx_end_int_raw",
                &format_args!("{}", self.ch7_rx_end_int_raw().bit()),
            )
            .field(
                "rx_ch4_err_int_raw",
                &format_args!("{}", self.rx_ch4_err_int_raw().bit()),
            )
            .field(
                "rx_ch5_err_int_raw",
                &format_args!("{}", self.rx_ch5_err_int_raw().bit()),
            )
            .field(
                "rx_ch6_err_int_raw",
                &format_args!("{}", self.rx_ch6_err_int_raw().bit()),
            )
            .field(
                "rx_ch7_err_int_raw",
                &format_args!("{}", self.rx_ch7_err_int_raw().bit()),
            )
            .field(
                "ch4_rx_thr_event_int_raw",
                &format_args!("{}", self.ch4_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch5_rx_thr_event_int_raw",
                &format_args!("{}", self.ch5_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch6_rx_thr_event_int_raw",
                &format_args!("{}", self.ch6_rx_thr_event_int_raw().bit()),
            )
            .field(
                "ch7_rx_thr_event_int_raw",
                &format_args!("{}", self.ch7_rx_thr_event_int_raw().bit()),
            )
            .field(
                "tx_ch3_dma_access_fail_int_raw",
                &format_args!("{}", self.tx_ch3_dma_access_fail_int_raw().bit()),
            )
            .field(
                "rx_ch7_dma_access_fail_int_raw",
                &format_args!("{}", self.rx_ch7_dma_access_fail_int_raw().bit()),
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
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end_int_raw(&mut self) -> CH0_TX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH0_TX_END_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end_int_raw(&mut self) -> CH1_TX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH1_TX_END_INT_RAW_W::new(self, 1)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end_int_raw(&mut self) -> CH2_TX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH2_TX_END_INT_RAW_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end_int_raw(&mut self) -> CH3_TX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH3_TX_END_INT_RAW_W::new(self, 3)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch0_err_int_raw(&mut self) -> TX_CH0_ERR_INT_RAW_W<INT_RAW_SPEC> {
        TX_CH0_ERR_INT_RAW_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch1_err_int_raw(&mut self) -> TX_CH1_ERR_INT_RAW_W<INT_RAW_SPEC> {
        TX_CH1_ERR_INT_RAW_W::new(self, 5)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch2_err_int_raw(&mut self) -> TX_CH2_ERR_INT_RAW_W<INT_RAW_SPEC> {
        TX_CH2_ERR_INT_RAW_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_err_int_raw(&mut self) -> TX_CH3_ERR_INT_RAW_W<INT_RAW_SPEC> {
        TX_CH3_ERR_INT_RAW_W::new(self, 7)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event_int_raw(&mut self) -> CH0_TX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH0_TX_THR_EVENT_INT_RAW_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event_int_raw(&mut self) -> CH1_TX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH1_TX_THR_EVENT_INT_RAW_W::new(self, 9)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event_int_raw(&mut self) -> CH2_TX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH2_TX_THR_EVENT_INT_RAW_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event_int_raw(&mut self) -> CH3_TX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH3_TX_THR_EVENT_INT_RAW_W::new(self, 11)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop_int_raw(&mut self) -> CH0_TX_LOOP_INT_RAW_W<INT_RAW_SPEC> {
        CH0_TX_LOOP_INT_RAW_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop_int_raw(&mut self) -> CH1_TX_LOOP_INT_RAW_W<INT_RAW_SPEC> {
        CH1_TX_LOOP_INT_RAW_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop_int_raw(&mut self) -> CH2_TX_LOOP_INT_RAW_W<INT_RAW_SPEC> {
        CH2_TX_LOOP_INT_RAW_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop_int_raw(&mut self) -> CH3_TX_LOOP_INT_RAW_W<INT_RAW_SPEC> {
        CH3_TX_LOOP_INT_RAW_W::new(self, 15)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_end_int_raw(&mut self) -> CH4_RX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH4_RX_END_INT_RAW_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL5. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_end_int_raw(&mut self) -> CH5_RX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH5_RX_END_INT_RAW_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL6. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_end_int_raw(&mut self) -> CH6_RX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH6_RX_END_INT_RAW_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL7. Triggered when reception done."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_end_int_raw(&mut self) -> CH7_RX_END_INT_RAW_W<INT_RAW_SPEC> {
        CH7_RX_END_INT_RAW_W::new(self, 19)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch4_err_int_raw(&mut self) -> RX_CH4_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RX_CH4_ERR_INT_RAW_W::new(self, 20)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL5. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch5_err_int_raw(&mut self) -> RX_CH5_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RX_CH5_ERR_INT_RAW_W::new(self, 21)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch6_err_int_raw(&mut self) -> RX_CH6_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RX_CH6_ERR_INT_RAW_W::new(self, 22)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL7. Triggered when error occurs."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_err_int_raw(&mut self) -> RX_CH7_ERR_INT_RAW_W<INT_RAW_SPEC> {
        RX_CH7_ERR_INT_RAW_W::new(self, 23)
    }
    #[doc = "Bit 24 - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch4_rx_thr_event_int_raw(&mut self) -> CH4_RX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH4_RX_THR_EVENT_INT_RAW_W::new(self, 24)
    }
    #[doc = "Bit 25 - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch5_rx_thr_event_int_raw(&mut self) -> CH5_RX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH5_RX_THR_EVENT_INT_RAW_W::new(self, 25)
    }
    #[doc = "Bit 26 - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch6_rx_thr_event_int_raw(&mut self) -> CH6_RX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH6_RX_THR_EVENT_INT_RAW_W::new(self, 26)
    }
    #[doc = "Bit 27 - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    #[must_use]
    pub fn ch7_rx_thr_event_int_raw(&mut self) -> CH7_RX_THR_EVENT_INT_RAW_W<INT_RAW_SPEC> {
        CH7_RX_THR_EVENT_INT_RAW_W::new(self, 27)
    }
    #[doc = "Bit 28 - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
    #[inline(always)]
    #[must_use]
    pub fn tx_ch3_dma_access_fail_int_raw(
        &mut self,
    ) -> TX_CH3_DMA_ACCESS_FAIL_INT_RAW_W<INT_RAW_SPEC> {
        TX_CH3_DMA_ACCESS_FAIL_INT_RAW_W::new(self, 28)
    }
    #[doc = "Bit 29 - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
    #[inline(always)]
    #[must_use]
    pub fn rx_ch7_dma_access_fail_int_raw(
        &mut self,
    ) -> RX_CH7_DMA_ACCESS_FAIL_INT_RAW_W<INT_RAW_SPEC> {
        RX_CH7_DMA_ACCESS_FAIL_INT_RAW_W::new(self, 29)
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
