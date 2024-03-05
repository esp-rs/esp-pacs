#[doc = "Register `CH_ENA_AD1_SET` writer"]
pub type W = crate::W<CH_ENA_AD1_SET_SPEC>;
#[doc = "Field `CH_SET32` writer - Configures whether or not to enable ch32.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET33` writer - Configures whether or not to enable ch33.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET34` writer - Configures whether or not to enable ch34.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET35` writer - Configures whether or not to enable ch35.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET36` writer - Configures whether or not to enable ch36.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET37` writer - Configures whether or not to enable ch37.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET38` writer - Configures whether or not to enable ch38.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET39` writer - Configures whether or not to enable ch39.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET40` writer - Configures whether or not to enable ch40.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET41` writer - Configures whether or not to enable ch41.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET42` writer - Configures whether or not to enable ch42.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET43` writer - Configures whether or not to enable ch43.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET44` writer - Configures whether or not to enable ch44.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET45` writer - Configures whether or not to enable ch45.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET46` writer - Configures whether or not to enable ch46.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET47` writer - Configures whether or not to enable ch47.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET48` writer - Configures whether or not to enable ch48.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_SET49` writer - Configures whether or not to enable ch49.\\\\0: Invalid, No effect\\\\1: Enable"]
pub type CH_SET49_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable ch32.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set32(&mut self) -> CH_SET32_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable ch33.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set33(&mut self) -> CH_SET33_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to enable ch34.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set34(&mut self) -> CH_SET34_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to enable ch35.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set35(&mut self) -> CH_SET35_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to enable ch36.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set36(&mut self) -> CH_SET36_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to enable ch37.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set37(&mut self) -> CH_SET37_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to enable ch38.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set38(&mut self) -> CH_SET38_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to enable ch39.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set39(&mut self) -> CH_SET39_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to enable ch40.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set40(&mut self) -> CH_SET40_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to enable ch41.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set41(&mut self) -> CH_SET41_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to enable ch42.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set42(&mut self) -> CH_SET42_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to enable ch43.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set43(&mut self) -> CH_SET43_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to enable ch44.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set44(&mut self) -> CH_SET44_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET44_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to enable ch45.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set45(&mut self) -> CH_SET45_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET45_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to enable ch46.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set46(&mut self) -> CH_SET46_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET46_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to enable ch47.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set47(&mut self) -> CH_SET47_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET47_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to enable ch48.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set48(&mut self) -> CH_SET48_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET48_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to enable ch49.\\\\0: Invalid, No effect\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set49(&mut self) -> CH_SET49_W<CH_ENA_AD1_SET_SPEC> {
        CH_SET49_W::new(self, 17)
    }
}
#[doc = "Channel enable set register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_set::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
