#[doc = "Register `CH_ENA_AD1_CLR` writer"]
pub type W = crate::W<CH_ENA_AD1_CLR_SPEC>;
#[doc = "Field `CH_DISABLE32` writer - Configures whether or not to clear ch32 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE33` writer - Configures whether or not to clear ch33 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE34` writer - Configures whether or not to clear ch34 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE35` writer - Configures whether or not to clear ch35 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE36` writer - Configures whether or not to clear ch36 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE37` writer - Configures whether or not to clear ch37 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE38` writer - Configures whether or not to clear ch38 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE39` writer - Configures whether or not to clear ch39 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE40` writer - Configures whether or not to clear ch40 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE41` writer - Configures whether or not to clear ch41 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE42` writer - Configures whether or not to clear ch42 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE43` writer - Configures whether or not to clear ch43 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE44` writer - Configures whether or not to clear ch44 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE45` writer - Configures whether or not to clear ch45 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE46` writer - Configures whether or not to clear ch46 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE47` writer - Configures whether or not to clear ch47 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE48` writer - Configures whether or not to clear ch48 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_DISABLE49` writer - Configures whether or not to clear ch49 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
pub type CH_DISABLE49_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to clear ch32 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable32(&mut self) -> CH_DISABLE32_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE32_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to clear ch33 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable33(&mut self) -> CH_DISABLE33_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE33_W::new(self, 1)
    }
    #[doc = "Bit 2 - Configures whether or not to clear ch34 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable34(&mut self) -> CH_DISABLE34_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE34_W::new(self, 2)
    }
    #[doc = "Bit 3 - Configures whether or not to clear ch35 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable35(&mut self) -> CH_DISABLE35_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE35_W::new(self, 3)
    }
    #[doc = "Bit 4 - Configures whether or not to clear ch36 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable36(&mut self) -> CH_DISABLE36_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE36_W::new(self, 4)
    }
    #[doc = "Bit 5 - Configures whether or not to clear ch37 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable37(&mut self) -> CH_DISABLE37_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE37_W::new(self, 5)
    }
    #[doc = "Bit 6 - Configures whether or not to clear ch38 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable38(&mut self) -> CH_DISABLE38_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE38_W::new(self, 6)
    }
    #[doc = "Bit 7 - Configures whether or not to clear ch39 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable39(&mut self) -> CH_DISABLE39_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE39_W::new(self, 7)
    }
    #[doc = "Bit 8 - Configures whether or not to clear ch40 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable40(&mut self) -> CH_DISABLE40_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE40_W::new(self, 8)
    }
    #[doc = "Bit 9 - Configures whether or not to clear ch41 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable41(&mut self) -> CH_DISABLE41_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE41_W::new(self, 9)
    }
    #[doc = "Bit 10 - Configures whether or not to clear ch42 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable42(&mut self) -> CH_DISABLE42_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE42_W::new(self, 10)
    }
    #[doc = "Bit 11 - Configures whether or not to clear ch43 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable43(&mut self) -> CH_DISABLE43_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE43_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures whether or not to clear ch44 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable44(&mut self) -> CH_DISABLE44_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE44_W::new(self, 12)
    }
    #[doc = "Bit 13 - Configures whether or not to clear ch45 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable45(&mut self) -> CH_DISABLE45_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE45_W::new(self, 13)
    }
    #[doc = "Bit 14 - Configures whether or not to clear ch46 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable46(&mut self) -> CH_DISABLE46_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE46_W::new(self, 14)
    }
    #[doc = "Bit 15 - Configures whether or not to clear ch47 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable47(&mut self) -> CH_DISABLE47_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE47_W::new(self, 15)
    }
    #[doc = "Bit 16 - Configures whether or not to clear ch48 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable48(&mut self) -> CH_DISABLE48_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE48_W::new(self, 16)
    }
    #[doc = "Bit 17 - Configures whether or not to clear ch49 enable.\\\\0: Invalid, No effect\\\\1: Clear"]
    #[inline(always)]
    pub fn ch_disable49(&mut self) -> CH_DISABLE49_W<CH_ENA_AD1_CLR_SPEC> {
        CH_DISABLE49_W::new(self, 17)
    }
}
#[doc = "Channel enable clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD1_CLR_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad1_clr::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD1_CLR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH_ENA_AD1_CLR to value 0"]
impl crate::Resettable for CH_ENA_AD1_CLR_SPEC {}
