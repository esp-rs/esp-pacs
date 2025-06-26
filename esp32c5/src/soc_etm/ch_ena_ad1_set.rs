#[doc = "Register `CH_ENA_AD1_SET` writer"]
pub type W = crate::W<CH_ENA_AD1_SET_SPEC>;
#[doc = "Field `CH_ENABLE(32-49)` writer - Configures whether or not to enable ch%s.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Configures whether or not to enable ch(32-49).\\\\0: Invalid, No effect\\\\1: Enable"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `CH_ENABLE32` field.</div>"]
    #[inline(always)]
    pub fn ch_enable(&mut self, n: u8) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 18][n as usize];
        CH_ENABLE_W::new(self, n)
    }
    #[doc = "Bit 0 - Configures whether or not to enable ch32.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable32(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable ch33.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable33(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable ch34.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable34(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable ch35.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable35(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable ch36.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable36(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ch37.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable37(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable ch38.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable38(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ch39.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable39(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable ch40.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable40(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable ch41.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable41(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable ch42.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable42(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable ch43.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable43(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable ch44.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable44(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable ch45.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable45(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable ch46.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable46(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable ch47.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable47(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable ch48.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable48(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable ch49.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    pub fn ch_enable49(&mut self) -> CH_ENABLE_W<CH_ENA_AD1_SET_SPEC> {
        CH_ENABLE_W::new(self, 17)
    }
}
#[doc = "Channel enable set register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD1_SET_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad1_set::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD1_SET_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD1_SET to value 0"]
impl crate::Resettable for CH_ENA_AD1_SET_SPEC {
    const RESET_VALUE: u32 = 0;
}
