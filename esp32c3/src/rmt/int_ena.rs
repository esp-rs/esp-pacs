#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CH_TX_END(0-1)` reader - reg_ch%s_tx_end_int_ena."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END(0-1)` writer - reg_ch%s_tx_end_int_ena."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END(2-3)` reader - reg_ch2_rx_end_int_ena."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END(2-3)` writer - reg_ch2_rx_end_int_ena."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_ERR(0-1)` reader - reg_ch%s_err_int_ena."]
pub type CH_TX_ERR_R = crate::BitReader;
#[doc = "Field `CH_TX_ERR(0-1)` writer - reg_ch%s_err_int_ena."]
pub type CH_TX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_ERR(2-3)` reader - reg_ch2_err_int_ena."]
pub type CH_RX_ERR_R = crate::BitReader;
#[doc = "Field `CH_RX_ERR(2-3)` writer - reg_ch2_err_int_ena."]
pub type CH_RX_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT(0-1)` reader - reg_ch%s_tx_thr_event_int_ena."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT(0-1)` writer - reg_ch%s_tx_thr_event_int_ena."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_THR_EVENT(2-3)` reader - reg_ch2_rx_thr_event_int_ena."]
pub type CH_RX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_RX_THR_EVENT(2-3)` writer - reg_ch2_rx_thr_event_int_ena."]
pub type CH_RX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_LOOP(0-1)` reader - reg_ch%s_tx_loop_int_ena."]
pub type CH_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP(0-1)` writer - reg_ch%s_tx_loop_int_ena."]
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "reg_ch(0-1)_tx_end_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_END_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "reg_ch(0-1)_tx_end_int_ena."]
    #[inline(always)]
    pub fn ch_tx_end_iter(&self) -> impl Iterator<Item = CH_TX_END_R> + '_ {
        (0..2).map(move |n| CH_TX_END_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_ena."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_ena."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "reg_ch2_rx_end_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_END` field"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_END_R::new(((self.bits >> (n + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "reg_ch2_rx_end_int_ena."]
    #[inline(always)]
    pub fn ch_rx_end_iter(&self) -> impl Iterator<Item = CH_RX_END_R> + '_ {
        (0..2).map(move |n| CH_RX_END_R::new(((self.bits >> (n + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_ena."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_ena."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "reg_ch(0-1)_err_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field"]
    #[inline(always)]
    pub fn ch_tx_err(&self, n: u8) -> CH_TX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "reg_ch(0-1)_err_int_ena."]
    #[inline(always)]
    pub fn ch_tx_err_iter(&self) -> impl Iterator<Item = CH_TX_ERR_R> + '_ {
        (0..2).map(move |n| CH_TX_ERR_R::new(((self.bits >> (n + 4)) & 1) != 0))
    }
    #[doc = "Bit 4 - reg_ch0_err_int_ena."]
    #[inline(always)]
    pub fn ch0_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_ena."]
    #[inline(always)]
    pub fn ch1_tx_err(&self) -> CH_TX_ERR_R {
        CH_TX_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "reg_ch2_err_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_ERR` field"]
    #[inline(always)]
    pub fn ch_rx_err(&self, n: u8) -> CH_RX_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_ERR_R::new(((self.bits >> (n + 6)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "reg_ch2_err_int_ena."]
    #[inline(always)]
    pub fn ch_rx_err_iter(&self) -> impl Iterator<Item = CH_RX_ERR_R> + '_ {
        (0..2).map(move |n| CH_RX_ERR_R::new(((self.bits >> (n + 6)) & 1) != 0))
    }
    #[doc = "Bit 6 - reg_ch2_err_int_ena."]
    #[inline(always)]
    pub fn ch2_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_ena."]
    #[inline(always)]
    pub fn ch3_rx_err(&self) -> CH_RX_ERR_R {
        CH_RX_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "reg_ch(0-1)_tx_thr_event_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "reg_ch(0-1)_tx_thr_event_int_ena."]
    #[inline(always)]
    pub fn ch_tx_thr_event_iter(&self) -> impl Iterator<Item = CH_TX_THR_EVENT_R> + '_ {
        (0..2).map(move |n| CH_TX_THR_EVENT_R::new(((self.bits >> (n + 8)) & 1) != 0))
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_ena."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_ena."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "reg_ch2_rx_thr_event_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_THR_EVENT` field"]
    #[inline(always)]
    pub fn ch_rx_thr_event(&self, n: u8) -> CH_RX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_THR_EVENT_R::new(((self.bits >> (n + 10)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "reg_ch2_rx_thr_event_int_ena."]
    #[inline(always)]
    pub fn ch_rx_thr_event_iter(&self) -> impl Iterator<Item = CH_RX_THR_EVENT_R> + '_ {
        (0..2).map(move |n| CH_RX_THR_EVENT_R::new(((self.bits >> (n + 10)) & 1) != 0))
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_ena."]
    #[inline(always)]
    pub fn ch2_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - reg_ch2_rx_thr_event_int_ena."]
    #[inline(always)]
    pub fn ch3_rx_thr_event(&self) -> CH_RX_THR_EVENT_R {
        CH_RX_THR_EVENT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "reg_ch(0-1)_tx_loop_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field"]
    #[inline(always)]
    pub fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "reg_ch(0-1)_tx_loop_int_ena."]
    #[inline(always)]
    pub fn ch_tx_loop_iter(&self) -> impl Iterator<Item = CH_TX_LOOP_R> + '_ {
        (0..2).map(move |n| CH_TX_LOOP_R::new(((self.bits >> (n + 12)) & 1) != 0))
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_ena."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_ena."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 13) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("ch0_tx_end", &format_args!("{}", self.ch0_tx_end().bit()))
            .field("ch1_tx_end", &format_args!("{}", self.ch1_tx_end().bit()))
            .field("ch2_rx_end", &format_args!("{}", self.ch2_rx_end().bit()))
            .field("ch3_rx_end", &format_args!("{}", self.ch3_rx_end().bit()))
            .field("ch0_tx_err", &format_args!("{}", self.ch0_tx_err().bit()))
            .field("ch1_tx_err", &format_args!("{}", self.ch1_tx_err().bit()))
            .field("ch2_rx_err", &format_args!("{}", self.ch2_rx_err().bit()))
            .field("ch3_rx_err", &format_args!("{}", self.ch3_rx_err().bit()))
            .field(
                "ch0_tx_thr_event",
                &format_args!("{}", self.ch0_tx_thr_event().bit()),
            )
            .field(
                "ch1_tx_thr_event",
                &format_args!("{}", self.ch1_tx_thr_event().bit()),
            )
            .field(
                "ch2_rx_thr_event",
                &format_args!("{}", self.ch2_rx_thr_event().bit()),
            )
            .field(
                "ch3_rx_thr_event",
                &format_args!("{}", self.ch3_rx_thr_event().bit()),
            )
            .field("ch0_tx_loop", &format_args!("{}", self.ch0_tx_loop().bit()))
            .field("ch1_tx_loop", &format_args!("{}", self.ch1_tx_loop().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "reg_ch(0-1)_tx_end_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_END_W::new(self, n)
    }
    #[doc = "Bit 0 - reg_ch0_tx_end_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_ch1_tx_end_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 1)
    }
    #[doc = "reg_ch2_rx_end_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_END_W::new(self, n + 2)
    }
    #[doc = "Bit 2 - reg_ch2_rx_end_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_ch2_rx_end_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 3)
    }
    #[doc = "reg_ch(0-1)_err_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_err(&mut self, n: u8) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_ERR_W::new(self, n + 4)
    }
    #[doc = "Bit 4 - reg_ch0_err_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_err(&mut self) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        CH_TX_ERR_W::new(self, 4)
    }
    #[doc = "Bit 5 - reg_ch1_err_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_err(&mut self) -> CH_TX_ERR_W<INT_ENA_SPEC> {
        CH_TX_ERR_W::new(self, 5)
    }
    #[doc = "reg_ch2_err_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_err(&mut self, n: u8) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_ERR_W::new(self, n + 6)
    }
    #[doc = "Bit 6 - reg_ch2_err_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_err(&mut self) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        CH_RX_ERR_W::new(self, 6)
    }
    #[doc = "Bit 7 - reg_ch2_err_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_err(&mut self) -> CH_RX_ERR_W<INT_ENA_SPEC> {
        CH_RX_ERR_W::new(self, 7)
    }
    #[doc = "reg_ch(0-1)_tx_thr_event_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 8)
    }
    #[doc = "Bit 8 - reg_ch0_tx_thr_event_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 8)
    }
    #[doc = "Bit 9 - reg_ch1_tx_thr_event_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 9)
    }
    #[doc = "reg_ch2_rx_thr_event_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH2_RX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_thr_event(&mut self, n: u8) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_RX_THR_EVENT_W::new(self, n + 10)
    }
    #[doc = "Bit 10 - reg_ch2_rx_thr_event_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 10)
    }
    #[doc = "Bit 11 - reg_ch2_rx_thr_event_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_thr_event(&mut self) -> CH_RX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_RX_THR_EVENT_W::new(self, 11)
    }
    #[doc = "reg_ch(0-1)_tx_loop_int_ena."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 2][n as usize];
        CH_TX_LOOP_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - reg_ch0_tx_loop_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 12)
    }
    #[doc = "Bit 13 - reg_ch1_tx_loop_int_ena."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 13)
    }
}
#[doc = "RMT_INT_ENA_REG.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
