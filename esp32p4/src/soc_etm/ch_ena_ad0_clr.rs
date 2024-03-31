#[doc = "Register `CH_ENA_AD0_CLR` writer"]
pub type W = crate::W<CH_ENA_AD0_CLR_SPEC>;
#[doc = "Field `CH_CLR(0-31)` writer - Configures whether or not to clear ch%s enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Configures whether or not to clear ch(0-31) enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CH_CLR0` field"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr(&mut self, n: u8) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_CLR_W::new(self, n)
    }
    #[doc = "Bit 0 - Configures whether or not to clear ch0 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr0(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear ch1 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr1(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear ch2 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr2(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear ch3 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr3(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear ch4 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr4(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear ch5 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr5(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear ch6 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr6(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear ch7 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr7(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear ch8 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr8(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear ch9 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr9(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear ch10 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr10(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear ch11 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr11(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear ch12 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr12(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear ch13 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr13(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear ch14 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr14(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear ch15 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr15(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear ch16 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr16(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear ch17 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr17(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to clear ch18 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr18(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to clear ch19 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr19(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to clear ch20 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr20(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to clear ch21 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr21(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to clear ch22 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr22(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to clear ch23 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr23(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to clear ch24 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr24(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to clear ch25 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr25(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to clear ch26 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr26(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to clear ch27 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr27(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to clear ch28 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr28(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to clear ch29 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr29(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to clear ch30 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr30(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to clear ch31 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr31(&mut self) -> CH_CLR_W<CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 31)
    }
}
#[doc = "Channel enable clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD0_CLR_SPEC;
impl crate::RegisterSpec for CH_ENA_AD0_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad0_clr::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD0_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD0_CLR to value 0"]
impl crate::Resettable for CH_ENA_AD0_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
