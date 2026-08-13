#[doc = "Register `CH_ENA_AD0_CLR` writer"]
pub type W = crate::W<CH_ENA_AD0_CLR_SPEC>;
#[doc = "Field `CH_CLR(0-31)` writer - Clear channel %s enable"]
pub type CH_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Clear channel (0-31) enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_CLR0` field.</div>"]
    #[inline(always)]
    pub fn ch_clr(&mut self, n: u8) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_CLR_W::new(self, n)
    }
    #[doc = "Bit 0 - Clear channel 0 enable"]
    #[inline(always)]
    pub fn ch_clr0(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Clear channel 1 enable"]
    #[inline(always)]
    pub fn ch_clr1(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - Clear channel 2 enable"]
    #[inline(always)]
    pub fn ch_clr2(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Clear channel 3 enable"]
    #[inline(always)]
    pub fn ch_clr3(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Clear channel 4 enable"]
    #[inline(always)]
    pub fn ch_clr4(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 4)
    }
    #[doc = "Bit 5 - Clear channel 5 enable"]
    #[inline(always)]
    pub fn ch_clr5(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 5)
    }
    #[doc = "Bit 6 - Clear channel 6 enable"]
    #[inline(always)]
    pub fn ch_clr6(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 6)
    }
    #[doc = "Bit 7 - Clear channel 7 enable"]
    #[inline(always)]
    pub fn ch_clr7(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 7)
    }
    #[doc = "Bit 8 - Clear channel 8 enable"]
    #[inline(always)]
    pub fn ch_clr8(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 8)
    }
    #[doc = "Bit 9 - Clear channel 9 enable"]
    #[inline(always)]
    pub fn ch_clr9(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 9)
    }
    #[doc = "Bit 10 - Clear channel 10 enable"]
    #[inline(always)]
    pub fn ch_clr10(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 10)
    }
    #[doc = "Bit 11 - Clear channel 11 enable"]
    #[inline(always)]
    pub fn ch_clr11(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 11)
    }
    #[doc = "Bit 12 - Clear channel 12 enable"]
    #[inline(always)]
    pub fn ch_clr12(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 12)
    }
    #[doc = "Bit 13 - Clear channel 13 enable"]
    #[inline(always)]
    pub fn ch_clr13(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 13)
    }
    #[doc = "Bit 14 - Clear channel 14 enable"]
    #[inline(always)]
    pub fn ch_clr14(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 14)
    }
    #[doc = "Bit 15 - Clear channel 15 enable"]
    #[inline(always)]
    pub fn ch_clr15(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 15)
    }
    #[doc = "Bit 16 - Clear channel 16 enable"]
    #[inline(always)]
    pub fn ch_clr16(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 16)
    }
    #[doc = "Bit 17 - Clear channel 17 enable"]
    #[inline(always)]
    pub fn ch_clr17(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 17)
    }
    #[doc = "Bit 18 - Clear channel 18 enable"]
    #[inline(always)]
    pub fn ch_clr18(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 18)
    }
    #[doc = "Bit 19 - Clear channel 19 enable"]
    #[inline(always)]
    pub fn ch_clr19(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 19)
    }
    #[doc = "Bit 20 - Clear channel 20 enable"]
    #[inline(always)]
    pub fn ch_clr20(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 20)
    }
    #[doc = "Bit 21 - Clear channel 21 enable"]
    #[inline(always)]
    pub fn ch_clr21(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 21)
    }
    #[doc = "Bit 22 - Clear channel 22 enable"]
    #[inline(always)]
    pub fn ch_clr22(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 22)
    }
    #[doc = "Bit 23 - Clear channel 23 enable"]
    #[inline(always)]
    pub fn ch_clr23(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 23)
    }
    #[doc = "Bit 24 - Clear channel 24 enable"]
    #[inline(always)]
    pub fn ch_clr24(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 24)
    }
    #[doc = "Bit 25 - Clear channel 25 enable"]
    #[inline(always)]
    pub fn ch_clr25(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 25)
    }
    #[doc = "Bit 26 - Clear channel 26 enable"]
    #[inline(always)]
    pub fn ch_clr26(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 26)
    }
    #[doc = "Bit 27 - Clear channel 27 enable"]
    #[inline(always)]
    pub fn ch_clr27(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 27)
    }
    #[doc = "Bit 28 - Clear channel 28 enable"]
    #[inline(always)]
    pub fn ch_clr28(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 28)
    }
    #[doc = "Bit 29 - Clear channel 29 enable"]
    #[inline(always)]
    pub fn ch_clr29(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 29)
    }
    #[doc = "Bit 30 - Clear channel 30 enable"]
    #[inline(always)]
    pub fn ch_clr30(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - Clear channel 31 enable"]
    #[inline(always)]
    pub fn ch_clr31(&mut self) -> CH_CLR_W<'_, CH_ENA_AD0_CLR_SPEC> {
        CH_CLR_W::new(self, 31)
    }
}
#[doc = "Channel disable register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
