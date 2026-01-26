#[doc = "Register `CH_ENA_AD0_SET` writer"]
pub type W = crate::W<CH_ENA_AD0_SET_SPEC>;
#[doc = "Field `CH_SET(0-31)` writer - Configures whether or not to enable ch%s.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Configures whether or not to enable ch(0-31).\\\\0: Invalid, No effect\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_SET0` field.</div>"]
    #[inline(always)]
    pub fn ch_set(&mut self, n: u8) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_SET_W::new(self, n)
    }
    #[doc = "Bit 0 - Configures whether or not to enable ch0.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set0(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable ch1.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set1(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable ch2.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set2(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable ch3.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set3(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable ch4.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set4(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ch5.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set5(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable ch6.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set6(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ch7.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set7(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable ch8.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set8(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable ch9.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set9(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable ch10.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set10(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable ch11.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set11(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable ch12.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set12(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable ch13.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set13(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable ch14.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set14(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable ch15.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set15(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable ch16.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set16(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable ch17.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set17(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to enable ch18.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set18(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to enable ch19.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set19(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to enable ch20.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set20(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to enable ch21.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set21(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to enable ch22.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set22(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to enable ch23.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set23(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to enable ch24.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set24(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to enable ch25.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set25(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to enable ch26.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set26(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to enable ch27.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set27(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable ch28.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set28(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to enable ch29.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set29(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to enable ch30.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set30(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to enable ch31.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_set31(&mut self) -> CH_SET_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_SET_W::new(self, 31)
    }
}
#[doc = "Channel enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad0_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD0_SET_SPEC;
impl crate::RegisterSpec for CH_ENA_AD0_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad0_set::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD0_SET_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_ENA_AD0_SET to value 0"]
impl crate::Resettable for CH_ENA_AD0_SET_SPEC {}
