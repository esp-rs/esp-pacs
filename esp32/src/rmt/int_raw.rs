#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Field `CH_TX_END(0-7)` reader - The interrupt raw bit for channel %s turns to high level when the transmit process is done."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END(0-7)` reader - The interrupt raw bit for channel %s turns to high level when the receive process is done."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_ERR(0-7)` reader - The interrupt raw bit for channel %s turns to high level when channle %s detects some errors."]
pub type CH_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT(0-7)` reader - The interrupt raw bit for channel %s turns to high level when transmitter in channle%s have send datas more than reg_rmt_tx_lim_ch%s after detecting this interrupt software can updata the old datas with new datas."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
impl R {
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when the transmit process is done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..8).map(move |n| CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0))
    }
    #[doc = "Bit 0 - The interrupt raw bit for channel 0 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for channel 1 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt raw bit for channel 2 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for channel 3 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt raw bit for channel 4 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch4_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt raw bit for channel 5 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch5_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt raw bit for channel 6 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch6_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - The interrupt raw bit for channel 7 turns to high level when the transmit process is done."]
    #[inline(always)]
    pub fn ch7_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when the receive process is done."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..8).map(move |n| CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0))
    }
    #[doc = "Bit 1 - The interrupt raw bit for channel 0 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch0_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt raw bit for channel 1 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch1_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for channel 2 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt raw bit for channel 3 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for channel 4 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch4_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt raw bit for channel 5 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch5_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt raw bit for channel 6 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch6_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 22 - The interrupt raw bit for channel 7 turns to high level when the receive process is done."]
    #[inline(always)]
    pub fn ch7_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when channle (0-7) detects some errors."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field"]
    #[inline(always)]
    pub fn ch_err(&self, n: u8) -> CH_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when channle (0-7) detects some errors."]
    #[inline(always)]
    pub fn ch_err_iter(&self) -> impl Iterator<Item = CH_ERR_R> + '_ {
        (0..8).map(move |n| CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - The interrupt raw bit for channel 0 turns to high level when channle 0 detects some errors."]
    #[inline(always)]
    pub fn ch0_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for channel 1 turns to high level when channle 1 detects some errors."]
    #[inline(always)]
    pub fn ch1_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt raw bit for channel 2 turns to high level when channle 2 detects some errors."]
    #[inline(always)]
    pub fn ch2_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for channel 3 turns to high level when channle 3 detects some errors."]
    #[inline(always)]
    pub fn ch3_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt raw bit for channel 4 turns to high level when channle 4 detects some errors."]
    #[inline(always)]
    pub fn ch4_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt raw bit for channel 5 turns to high level when channle 5 detects some errors."]
    #[inline(always)]
    pub fn ch5_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 20 - The interrupt raw bit for channel 6 turns to high level when channle 6 detects some errors."]
    #[inline(always)]
    pub fn ch6_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 23 - The interrupt raw bit for channel 7 turns to high level when channle 7 detects some errors."]
    #[inline(always)]
    pub fn ch7_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when transmitter in channle(0-7) have send datas more than reg_rmt_tx_lim_ch(0-7) after detecting this interrupt software can updata the old datas with new datas."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for channel (0-7) turns to high level when transmitter in channle(0-7) have send datas more than reg_rmt_tx_lim_ch(0-7) after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..8).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 24)) & 1) != 0))
    }
    #[doc = "Bit 24 - The interrupt raw bit for channel 0 turns to high level when transmitter in channle0 have send datas more than reg_rmt_tx_lim_ch0 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - The interrupt raw bit for channel 1 turns to high level when transmitter in channle1 have send datas more than reg_rmt_tx_lim_ch1 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - The interrupt raw bit for channel 2 turns to high level when transmitter in channle2 have send datas more than reg_rmt_tx_lim_ch2 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - The interrupt raw bit for channel 3 turns to high level when transmitter in channle3 have send datas more than reg_rmt_tx_lim_ch3 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - The interrupt raw bit for channel 4 turns to high level when transmitter in channle4 have send datas more than reg_rmt_tx_lim_ch4 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch4_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - The interrupt raw bit for channel 5 turns to high level when transmitter in channle5 have send datas more than reg_rmt_tx_lim_ch5 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch5_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - The interrupt raw bit for channel 6 turns to high level when transmitter in channle6 have send datas more than reg_rmt_tx_lim_ch6 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch6_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - The interrupt raw bit for channel 7 turns to high level when transmitter in channle7 have send datas more than reg_rmt_tx_lim_ch7 after detecting this interrupt software can updata the old datas with new datas."]
    #[inline(always)]
    pub fn ch7_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 31) & 1) != 0)
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
            .field("ch4_tx_end", &self.ch4_tx_end())
            .field("ch5_tx_end", &self.ch5_tx_end())
            .field("ch6_tx_end", &self.ch6_tx_end())
            .field("ch7_tx_end", &self.ch7_tx_end())
            .field("ch0_rx_end", &self.ch0_rx_end())
            .field("ch1_rx_end", &self.ch1_rx_end())
            .field("ch2_rx_end", &self.ch2_rx_end())
            .field("ch3_rx_end", &self.ch3_rx_end())
            .field("ch4_rx_end", &self.ch4_rx_end())
            .field("ch5_rx_end", &self.ch5_rx_end())
            .field("ch6_rx_end", &self.ch6_rx_end())
            .field("ch7_rx_end", &self.ch7_rx_end())
            .field("ch0_err", &self.ch0_err())
            .field("ch1_err", &self.ch1_err())
            .field("ch2_err", &self.ch2_err())
            .field("ch3_err", &self.ch3_err())
            .field("ch4_err", &self.ch4_err())
            .field("ch5_err", &self.ch5_err())
            .field("ch6_err", &self.ch6_err())
            .field("ch7_err", &self.ch7_err())
            .field("ch0_tx_thr_event", &self.ch0_tx_thr_event())
            .field("ch1_tx_thr_event", &self.ch1_tx_thr_event())
            .field("ch2_tx_thr_event", &self.ch2_tx_thr_event())
            .field("ch3_tx_thr_event", &self.ch3_tx_thr_event())
            .field("ch4_tx_thr_event", &self.ch4_tx_thr_event())
            .field("ch5_tx_thr_event", &self.ch5_tx_thr_event())
            .field("ch6_tx_thr_event", &self.ch6_tx_thr_event())
            .field("ch7_tx_thr_event", &self.ch7_tx_thr_event())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_raw::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_RAW_SPEC;
impl crate::RegisterSpec for INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_raw::R`](R) reader structure"]
impl crate::Readable for INT_RAW_SPEC {}
#[doc = "`reset()` method sets INT_RAW to value 0"]
impl crate::Resettable for INT_RAW_SPEC {
    const RESET_VALUE: u32 = 0;
}
