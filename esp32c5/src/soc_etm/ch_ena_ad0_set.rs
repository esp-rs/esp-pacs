#[doc = "Register `CH_ENA_AD0_SET` writer"]
pub type W = crate::W<CH_ENA_AD0_SET_SPEC>;
#[doc = "Field `CH_ENABLE(0-31)` writer - Configures whether or not to enable ch%s.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD0_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Configures whether or not to enable ch(0-31).\\\\0: Invalid, No effect\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_ENABLE0` field.</div>"]
    #[inline(always)]
    pub fn ch_enable(&mut self, n: u8) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 32][n as usize];
        CH_ENABLE_W::new(self, n)
    }
    #[doc = "Bit 0 - Configures whether or not to enable ch0.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable0(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable ch1.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable1(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable ch2.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable2(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable ch3.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable3(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable ch4.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable4(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ch5.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable5(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable ch6.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable6(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ch7.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable7(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable ch8.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable8(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable ch9.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable9(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable ch10.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable10(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable ch11.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable11(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable ch12.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable12(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable ch13.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable13(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable ch14.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable14(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable ch15.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable15(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable ch16.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable16(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable ch17.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable17(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Configures whether or not to enable ch18.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable18(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Configures whether or not to enable ch19.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable19(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Configures whether or not to enable ch20.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable20(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Configures whether or not to enable ch21.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable21(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Configures whether or not to enable ch22.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable22(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Configures whether or not to enable ch23.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable23(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Configures whether or not to enable ch24.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable24(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 24)
    }
    #[doc = "Bit 25 - Configures whether or not to enable ch25.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable25(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 25)
    }
    #[doc = "Bit 26 - Configures whether or not to enable ch26.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable26(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 26)
    }
    #[doc = "Bit 27 - Configures whether or not to enable ch27.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable27(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 27)
    }
    #[doc = "Bit 28 - Configures whether or not to enable ch28.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable28(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 28)
    }
    #[doc = "Bit 29 - Configures whether or not to enable ch29.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable29(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 29)
    }
    #[doc = "Bit 30 - Configures whether or not to enable ch30.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable30(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 30)
    }
    #[doc = "Bit 31 - Configures whether or not to enable ch31.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable31(&mut self) -> CH_ENABLE_W<'_, CH_ENA_AD0_SET_SPEC> {
        CH_ENABLE_W::new(self, 31)
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
