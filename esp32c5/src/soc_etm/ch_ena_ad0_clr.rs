#[doc = "Register `CH_ENA_AD0_CLR` writer"]
pub type W = crate::W<CH_ENA_AD0_CLR_SPEC>;
#[doc = "Field `CH_DISABLE(0-31)` writer - Configures whether or not to clear ch%s enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Configures whether or not to clear ch(0-31) enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_DISABLE0` field.</div>"]
    #[inline(always)]
    pub fn ch_disable(&mut self, n: u8) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_DISABLE_W::new(self, n)
    }
    #[doc = "Bit 0 - Configures whether or not to clear ch0 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable0(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear ch1 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable1(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear ch2 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable2(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear ch3 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable3(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear ch4 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable4(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear ch5 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable5(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear ch6 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable6(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear ch7 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable7(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear ch8 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable8(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear ch9 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable9(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear ch10 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable10(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear ch11 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable11(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear ch12 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable12(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear ch13 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable13(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear ch14 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable14(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear ch15 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable15(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear ch16 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable16(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear ch17 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable17(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to clear ch18 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable18(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to clear ch19 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable19(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to clear ch20 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable20(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to clear ch21 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable21(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to clear ch22 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable22(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to clear ch23 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable23(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to clear ch24 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable24(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to clear ch25 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable25(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to clear ch26 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable26(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to clear ch27 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable27(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to clear ch28 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable28(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to clear ch29 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable29(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to clear ch30 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable30(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to clear ch31 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable31(&mut self) -> CH_DISABLE_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_DISABLE_W::new(self, 31)
    }
}
#[doc = "Channel enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD0_CLR_SPEC;
impl crate::RegisterSpec for CH_ENA_AD0_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad0_clr::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD0_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_ENA_AD0_CLR to value 0"]
impl crate::Resettable for CH_ENA_AD0_CLR_SPEC {}
