#[doc = "Register `INT_RAW` reader"]
pub struct R(crate::R<INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH_TX_END_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmission done."]
pub type CH_TX_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_ERR_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when error occurs."]
pub type CH_TX_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_THR_EVENT_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value."]
pub type CH_TX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_TX_LOOP_INT_RAW[0-3]` reader - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value."]
pub type CH_TX_LOOP_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_RX_END_INT_RAW[4-7]` reader - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
pub type CH_RX_END_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH_RX_ERR_INT_RAW[4-7]` reader - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH_RX_ERR_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH4_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL4. Triggered when receiver receive more data than configured value."]
pub type CH4_RX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH5_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL5. Triggered when receiver receive more data than configured value."]
pub type CH5_RX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH6_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL6. Triggered when receiver receive more data than configured value."]
pub type CH6_RX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `CH7_RX_THR_EVENT_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when receiver receive more data than configured value."]
pub type CH7_RX_THR_EVENT_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `TX_CH3_DMA_ACCESS_FAIL_INT_RAW` reader - The interrupt raw bit for CHANNEL3. Triggered when dma accessing CHANNEL3 fails."]
pub type TX_CH3_DMA_ACCESS_FAIL_INT_RAW_R = crate::BitReader<bool>;
#[doc = "Field `RX_CH7_DMA_ACCESS_FAIL_INT_RAW` reader - The interrupt raw bit for CHANNEL7. Triggered when dma accessing CHANNEL7 fails."]
pub type RX_CH7_DMA_ACCESS_FAIL_INT_RAW_R = crate::BitReader<bool>;
impl R {
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when transmission done."]
    #[inline(always)]
    pub unsafe fn ch_tx_end_int_raw(&self, n: u8) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch2_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL3. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch3_tx_end_int_raw(&self) -> CH_TX_END_INT_RAW_R {
        CH_TX_END_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when error occurs."]
    #[inline(always)]
    pub unsafe fn ch_tx_err_int_raw(&self, n: u8) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL0. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL1. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL2. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL3. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_tx_err_int_raw(&self) -> CH_TX_ERR_INT_RAW_R {
        CH_TX_ERR_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub unsafe fn ch_tx_thr_event_int_raw(&self, n: u8) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch2_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL3. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch3_tx_thr_event_int_raw(&self) -> CH_TX_THR_EVENT_INT_RAW_R {
        CH_TX_THR_EVENT_INT_RAW_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL[0-3]. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub unsafe fn ch_tx_loop_int_raw(&self, n: u8) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for CHANNEL2. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch2_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for CHANNEL3. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch3_tx_loop_int_raw(&self) -> CH_TX_LOOP_INT_RAW_R {
        CH_TX_LOOP_INT_RAW_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub unsafe fn ch_rx_end_int_raw(&self, n: u8) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> (n - 4 + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch4_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch5_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch6_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for CHANNEL4. Triggered when reception done."]
    #[inline(always)]
    pub fn ch7_rx_end_int_raw(&self) -> CH_RX_END_INT_RAW_R {
        CH_RX_END_INT_RAW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub unsafe fn ch_rx_err_int_raw(&self, n: u8) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> (n - 4 + 20)) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch4_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch5_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch6_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch7_rx_err_int_raw(&self) -> CH_RX_ERR_INT_RAW_R {
        CH_RX_ERR_INT_RAW_R::new(((self.bits >> 23) & 1) != 0)
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
#[doc = "Raw interrupt status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_raw](index.html) module"]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_raw::R](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
