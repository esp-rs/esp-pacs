#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CH_TX_END[0-3]` reader - The interrupt enabled bit for CH%s_TX_END_INT."]
pub type CH_TX_END_R = crate::BitReader;
#[doc = "Field `CH_TX_END[0-3]` writer - The interrupt enabled bit for CH%s_TX_END_INT."]
pub type CH_TX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_RX_END[0-3]` reader - The interrupt enabled bit for CH%s_RX_END_INT."]
pub type CH_RX_END_R = crate::BitReader;
#[doc = "Field `CH_RX_END[0-3]` writer - The interrupt enabled bit for CH%s_RX_END_INT."]
pub type CH_RX_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ERR[0-3]` reader - The interrupt enabled bit for CH%s_ERR_INT."]
pub type CH_ERR_R = crate::BitReader;
#[doc = "Field `CH_ERR[0-3]` writer - The interrupt enabled bit for CH%s_ERR_INT."]
pub type CH_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_THR_EVENT[0-3]` reader - The interrupt enabled bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_R = crate::BitReader;
#[doc = "Field `CH_TX_THR_EVENT[0-3]` writer - The interrupt enabled bit for CH%s_TX_THR_EVENT_INT."]
pub type CH_TX_THR_EVENT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_TX_LOOP[0-3]` reader - The interrupt enabled bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_R = crate::BitReader;
#[doc = "Field `CH_TX_LOOP[0-3]` writer - The interrupt enabled bit for CH%s_TX_LOOP_INT."]
pub type CH_TX_LOOP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "The interrupt enabled bit for CH[0-3]_TX_END_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    pub fn ch_tx_end(&self, n: u8) -> CH_TX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_R::new(((self.bits >> (n * 3)) & 1) != 0)
    }
    #[doc = "Bit 0 - The interrupt enabled bit for CH0_TX_END_INT."]
    #[inline(always)]
    pub fn ch0_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 3 - The interrupt enabled bit for CH1_TX_END_INT."]
    #[inline(always)]
    pub fn ch1_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - The interrupt enabled bit for CH2_TX_END_INT."]
    #[inline(always)]
    pub fn ch2_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 9 - The interrupt enabled bit for CH3_TX_END_INT."]
    #[inline(always)]
    pub fn ch3_tx_end(&self) -> CH_TX_END_R {
        CH_TX_END_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_RX_END_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field"]
    #[inline(always)]
    pub fn ch_rx_end(&self, n: u8) -> CH_RX_END_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_R::new(((self.bits >> (n * 3 + 1)) & 1) != 0)
    }
    #[doc = "Bit 1 - The interrupt enabled bit for CH0_RX_END_INT."]
    #[inline(always)]
    pub fn ch0_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - The interrupt enabled bit for CH1_RX_END_INT."]
    #[inline(always)]
    pub fn ch1_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - The interrupt enabled bit for CH2_RX_END_INT."]
    #[inline(always)]
    pub fn ch2_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 10 - The interrupt enabled bit for CH3_RX_END_INT."]
    #[inline(always)]
    pub fn ch3_rx_end(&self) -> CH_RX_END_R {
        CH_RX_END_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_ERR_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field"]
    #[inline(always)]
    pub fn ch_err(&self, n: u8) -> CH_ERR_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_ERR_R::new(((self.bits >> (n * 3 + 2)) & 1) != 0)
    }
    #[doc = "Bit 2 - The interrupt enabled bit for CH0_ERR_INT."]
    #[inline(always)]
    pub fn ch0_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - The interrupt enabled bit for CH1_ERR_INT."]
    #[inline(always)]
    pub fn ch1_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - The interrupt enabled bit for CH2_ERR_INT."]
    #[inline(always)]
    pub fn ch2_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - The interrupt enabled bit for CH3_ERR_INT."]
    #[inline(always)]
    pub fn ch3_err(&self) -> CH_ERR_R {
        CH_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_TX_THR_EVENT_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    pub fn ch_tx_thr_event(&self, n: u8) -> CH_TX_THR_EVENT_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_R::new(((self.bits >> (n + 12)) & 1) != 0)
    }
    #[doc = "Bit 12 - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch0_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch1_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch2_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    pub fn ch3_tx_thr_event(&self) -> CH_TX_THR_EVENT_R {
        CH_TX_THR_EVENT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_TX_LOOP_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field"]
    #[inline(always)]
    pub fn ch_tx_loop(&self, n: u8) -> CH_TX_LOOP_R {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_R::new(((self.bits >> (n + 16)) & 1) != 0)
    }
    #[doc = "Bit 16 - The interrupt enabled bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch0_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - The interrupt enabled bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch1_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - The interrupt enabled bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch2_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - The interrupt enabled bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    pub fn ch3_tx_loop(&self) -> CH_TX_LOOP_R {
        CH_TX_LOOP_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("ch0_tx_end", &format_args!("{}", self.ch0_tx_end().bit()))
            .field("ch1_tx_end", &format_args!("{}", self.ch1_tx_end().bit()))
            .field("ch2_tx_end", &format_args!("{}", self.ch2_tx_end().bit()))
            .field("ch3_tx_end", &format_args!("{}", self.ch3_tx_end().bit()))
            .field("ch0_rx_end", &format_args!("{}", self.ch0_rx_end().bit()))
            .field("ch1_rx_end", &format_args!("{}", self.ch1_rx_end().bit()))
            .field("ch2_rx_end", &format_args!("{}", self.ch2_rx_end().bit()))
            .field("ch3_rx_end", &format_args!("{}", self.ch3_rx_end().bit()))
            .field("ch0_err", &format_args!("{}", self.ch0_err().bit()))
            .field("ch1_err", &format_args!("{}", self.ch1_err().bit()))
            .field("ch2_err", &format_args!("{}", self.ch2_err().bit()))
            .field("ch3_err", &format_args!("{}", self.ch3_err().bit()))
            .field(
                "ch0_tx_thr_event",
                &format_args!("{}", self.ch0_tx_thr_event().bit()),
            )
            .field(
                "ch1_tx_thr_event",
                &format_args!("{}", self.ch1_tx_thr_event().bit()),
            )
            .field(
                "ch2_tx_thr_event",
                &format_args!("{}", self.ch2_tx_thr_event().bit()),
            )
            .field(
                "ch3_tx_thr_event",
                &format_args!("{}", self.ch3_tx_thr_event().bit()),
            )
            .field("ch0_tx_loop", &format_args!("{}", self.ch0_tx_loop().bit()))
            .field("ch1_tx_loop", &format_args!("{}", self.ch1_tx_loop().bit()))
            .field("ch2_tx_loop", &format_args!("{}", self.ch2_tx_loop().bit()))
            .field("ch3_tx_loop", &format_args!("{}", self.ch3_tx_loop().bit()))
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
    #[doc = "The interrupt enabled bit for CH[0-3]_TX_END_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_end(&mut self, n: u8) -> CH_TX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_END_W::new(self, n * 3)
    }
    #[doc = "Bit 0 - The interrupt enabled bit for CH0_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 0)
    }
    #[doc = "Bit 3 - The interrupt enabled bit for CH1_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 3)
    }
    #[doc = "Bit 6 - The interrupt enabled bit for CH2_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 6)
    }
    #[doc = "Bit 9 - The interrupt enabled bit for CH3_TX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_end(&mut self) -> CH_TX_END_W<INT_ENA_SPEC> {
        CH_TX_END_W::new(self, 9)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_RX_END_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_RX_END` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_rx_end(&mut self, n: u8) -> CH_RX_END_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_RX_END_W::new(self, n * 3 + 1)
    }
    #[doc = "Bit 1 - The interrupt enabled bit for CH0_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 1)
    }
    #[doc = "Bit 4 - The interrupt enabled bit for CH1_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 4)
    }
    #[doc = "Bit 7 - The interrupt enabled bit for CH2_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 7)
    }
    #[doc = "Bit 10 - The interrupt enabled bit for CH3_RX_END_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_rx_end(&mut self) -> CH_RX_END_W<INT_ENA_SPEC> {
        CH_RX_END_W::new(self, 10)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_ERR_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_ERR` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_err(&mut self, n: u8) -> CH_ERR_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_ERR_W::new(self, n * 3 + 2)
    }
    #[doc = "Bit 2 - The interrupt enabled bit for CH0_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 2)
    }
    #[doc = "Bit 5 - The interrupt enabled bit for CH1_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 5)
    }
    #[doc = "Bit 8 - The interrupt enabled bit for CH2_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 8)
    }
    #[doc = "Bit 11 - The interrupt enabled bit for CH3_ERR_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_err(&mut self) -> CH_ERR_W<INT_ENA_SPEC> {
        CH_ERR_W::new(self, 11)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_TX_THR_EVENT_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_THR_EVENT` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_thr_event(&mut self, n: u8) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_THR_EVENT_W::new(self, n + 12)
    }
    #[doc = "Bit 12 - The interrupt enabled bit for CH0_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 12)
    }
    #[doc = "Bit 13 - The interrupt enabled bit for CH1_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 13)
    }
    #[doc = "Bit 14 - The interrupt enabled bit for CH2_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 14)
    }
    #[doc = "Bit 15 - The interrupt enabled bit for CH3_TX_THR_EVENT_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_thr_event(&mut self) -> CH_TX_THR_EVENT_W<INT_ENA_SPEC> {
        CH_TX_THR_EVENT_W::new(self, 15)
    }
    #[doc = "The interrupt enabled bit for CH[0-3]_TX_LOOP_INT.\n\nNOTE: `n` is number of field in register. `n == 0` corresponds to `CH0_TX_LOOP` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_tx_loop(&mut self, n: u8) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 4][n as usize];
        CH_TX_LOOP_W::new(self, n + 16)
    }
    #[doc = "Bit 16 - The interrupt enabled bit for CH0_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch0_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 16)
    }
    #[doc = "Bit 17 - The interrupt enabled bit for CH1_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch1_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 17)
    }
    #[doc = "Bit 18 - The interrupt enabled bit for CH2_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch2_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 18)
    }
    #[doc = "Bit 19 - The interrupt enabled bit for CH3_TX_LOOP_INT."]
    #[inline(always)]
    #[must_use]
    pub fn ch3_tx_loop(&mut self) -> CH_TX_LOOP_W<INT_ENA_SPEC> {
        CH_TX_LOOP_W::new(self, 19)
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
#[doc = "Interrupt enable bits\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
