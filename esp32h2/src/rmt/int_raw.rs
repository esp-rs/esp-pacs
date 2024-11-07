#[doc = "Register `INT_RAW` reader"]
pub type R = crate::R<INT_RAW_SPEC>;
#[doc = "Register `INT_RAW` writer"]
pub type W = crate::W<INT_RAW_SPEC>;
#[doc = "Field `CH_TX_END(0-1)` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmission done."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END(0-1)` writer - The interrupt raw bit for CHANNEL%s. Triggered when transmission done."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END(2-3)` reader - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END(2-3)` writer - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_ERR(0-1)` reader - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH_TX_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR(0-1)` writer - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
pub type CH_TX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_ERR(2-3)` reader - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type CH_RX_ERR_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR(2-3)` writer - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
pub type CH_RX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT(0-1)` reader - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT(0-1)` writer - The interrupt raw bit for CHANNEL%s. Triggered when transmitter sent more data than configured value."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_THR_EVENT(2-3)` reader - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
pub type CH_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_RX_THR_EVENT(2-3)` writer - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
pub type CH_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_LOOP(0-1)` reader - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value."]
pub type CH_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP(0-1)` writer - The interrupt raw bit for CHANNEL%s. Triggered when the loop count reaches the configured threshold value."]
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when transmission done."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_END_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when transmission done."]
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..2).map(move |n| CH_TX_END_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH2_RX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_END_R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..2).map(move |n| CH_RX_END_R::new(((self.bits >> (n + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_err(&self, n: u8) -> CH_TX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch_tx_err_iter(&self) -> impl Iterator<Item = CH_TX_ERR_R> + '_ {
        (0..2).map(move |n| CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH2_RX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_err(&self, n: u8) -> CH_RX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_ERR_R::new(((self.bits >> (n + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch_rx_err_iter(&self) -> impl Iterator<Item = CH_RX_ERR_R> + '_ {
        (0..2).map(move |n| CH_RX_ERR_R::new(((self.bits >> (n + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when transmitter sent more data than configured value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..2).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH2_RX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_thr_event(&self, n: u8) -> CH_RX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_THR_EVENT_R::new(((self.bits >> (n + 10)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch_rx_thr_event_iter(&self) -> impl Iterator<Item = CH_RX_THR_EVENT_R> + '_ {
        (0..2).map(move |n| CH_RX_THR_EVENT_R::new(((self.bits >> (n + 10)) & 1) != 0))
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch2_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch3_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when the loop count reaches the configured threshold value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch_tx_loop_iter(&self) -> impl Iterator<Item = CH_TX_LOOP_R> + '_ {
        (0..2).map(move |n| CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_RAW")
            .field("ch0_tx_end", &self.ch0_tx_end())
            .field("ch1_tx_end", &self.ch1_tx_end())
            .field("ch2_rx_end", &self.ch2_rx_end())
            .field("ch3_rx_end", &self.ch3_rx_end())
            .field("ch0_tx_err", &self.ch0_tx_err())
            .field("ch1_tx_err", &self.ch1_tx_err())
            .field("ch2_rx_err", &self.ch2_rx_err())
            .field("ch3_rx_err", &self.ch3_rx_err())
            .field("ch0_tx_thr_event", &self.ch0_tx_thr_event())
            .field("ch1_tx_thr_event", &self.ch1_tx_thr_event())
            .field("ch2_rx_thr_event", &self.ch2_rx_thr_event())
            .field("ch3_rx_thr_event", &self.ch3_rx_thr_event())
            .field("ch0_tx_loop", &self.ch0_tx_loop())
            .field("ch1_tx_loop", &self.ch1_tx_loop())
            .finish()
    }
}
impl W {
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when transmission done."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_END_W::new(self, n)
    }
    #[doc = "Bit 0 - The interrupt raw bit for CHANNEL0. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - The interrupt raw bit for CHANNEL1. Triggered when transmission done."]
    #[inline(always)]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_RAW_SPEC> {
        CH_TX_END_W::new(self, 1)
    }
    #[doc = "The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH2_RX_END` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_END_W::new(self, n + 2)
    }
    #[doc = "Bit 2 - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC> {
        CH_RX_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - The interrupt raw bit for CHANNEL2. Triggered when reception done."]
    #[inline(always)]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_RAW_SPEC> {
        CH_RX_END_W::new(self, 3)
    }
    #[doc = "The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_err(&mut self, n: u8) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_ERR_W::new(self, n + 4)
    }
    #[doc = "Bit 4 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        CH_TX_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - The interrupt raw bit for CHANNEL4. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_RAW_SPEC> {
        CH_TX_ERR_W::new(self, 5)
    }
    #[doc = "The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH2_RX_ERR` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_err(&mut self, n: u8) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_ERR_W::new(self, n + 6)
    }
    #[doc = "Bit 6 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch2_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        CH_RX_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - The interrupt raw bit for CHANNEL6. Triggered when error occurs."]
    #[inline(always)]
    pub fn ch3_rx_err(&mut self) -> CH_RX_ERR_W<INT_RAW_SPEC> {
        CH_RX_ERR_W::new(self, 7)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when transmitter sent more data than configured value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 8)
    }
    #[doc = "Bit 8 - The interrupt raw bit for CHANNEL0. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 8)
    }
    #[doc = "Bit 9 - The interrupt raw bit for CHANNEL1. Triggered when transmitter sent more data than configured value."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 9)
    }
    #[doc = "The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH2_RX_THR_EVENT` field.</div>"]
    #[inline(always)]
    pub fn ch_rx_thr_event(&mut self, n: u8) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_THR_EVENT_W::new(self, n + 10)
    }
    #[doc = "Bit 10 - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch2_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 10)
    }
    #[doc = "Bit 11 - The interrupt raw bit for CHANNEL2. Triggered when receiver receive more data than configured value."]
    #[inline(always)]
    pub fn ch3_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_RAW_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 11)
    }
    #[doc = "The interrupt raw bit for CHANNEL(0-1). Triggered when the loop count reaches the configured threshold value."]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field.</div>"]
    #[inline(always)]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_LOOP_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - The interrupt raw bit for CHANNEL0. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        CH_TX_LOOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt raw bit for CHANNEL1. Triggered when the loop count reaches the configured threshold value."]
    #[inline(always)]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_RAW_SPEC> {
        CH_TX_LOOP_W::new(self, 13)
    }
}
#[doc = "Raw interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
