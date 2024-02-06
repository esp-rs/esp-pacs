#[doc = "Register `CH_ENA_AD1_CLR` writer"]
pub type W = crate::W<CH_ENA_AD1_CLR_SPEC>;
#[doc = "Field `CH_CLR32` writer - ch32 clear"]
pub type CH_CLR32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR33` writer - ch33 clear"]
pub type CH_CLR33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR34` writer - ch34 clear"]
pub type CH_CLR34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR35` writer - ch35 clear"]
pub type CH_CLR35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR36` writer - ch36 clear"]
pub type CH_CLR36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR37` writer - ch37 clear"]
pub type CH_CLR37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR38` writer - ch38 clear"]
pub type CH_CLR38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR39` writer - ch39 clear"]
pub type CH_CLR39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR40` writer - ch40 clear"]
pub type CH_CLR40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR41` writer - ch41 clear"]
pub type CH_CLR41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR42` writer - ch42 clear"]
pub type CH_CLR42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR43` writer - ch43 clear"]
pub type CH_CLR43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR44` writer - ch44 clear"]
pub type CH_CLR44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR45` writer - ch45 clear"]
pub type CH_CLR45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR46` writer - ch46 clear"]
pub type CH_CLR46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR47` writer - ch47 clear"]
pub type CH_CLR47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR48` writer - ch48 clear"]
pub type CH_CLR48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_CLR49` writer - ch49 clear"]
pub type CH_CLR49_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ch32 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr32(&mut self) -> CH_CLR32_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR32_W::new(self, 0)
    }
    #[doc = "Bit 1 - ch33 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr33(&mut self) -> CH_CLR33_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR33_W::new(self, 1)
    }
    #[doc = "Bit 2 - ch34 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr34(&mut self) -> CH_CLR34_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR34_W::new(self, 2)
    }
    #[doc = "Bit 3 - ch35 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr35(&mut self) -> CH_CLR35_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR35_W::new(self, 3)
    }
    #[doc = "Bit 4 - ch36 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr36(&mut self) -> CH_CLR36_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR36_W::new(self, 4)
    }
    #[doc = "Bit 5 - ch37 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr37(&mut self) -> CH_CLR37_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR37_W::new(self, 5)
    }
    #[doc = "Bit 6 - ch38 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr38(&mut self) -> CH_CLR38_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR38_W::new(self, 6)
    }
    #[doc = "Bit 7 - ch39 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr39(&mut self) -> CH_CLR39_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR39_W::new(self, 7)
    }
    #[doc = "Bit 8 - ch40 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr40(&mut self) -> CH_CLR40_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR40_W::new(self, 8)
    }
    #[doc = "Bit 9 - ch41 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr41(&mut self) -> CH_CLR41_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR41_W::new(self, 9)
    }
    #[doc = "Bit 10 - ch42 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr42(&mut self) -> CH_CLR42_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR42_W::new(self, 10)
    }
    #[doc = "Bit 11 - ch43 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr43(&mut self) -> CH_CLR43_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR43_W::new(self, 11)
    }
    #[doc = "Bit 12 - ch44 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr44(&mut self) -> CH_CLR44_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR44_W::new(self, 12)
    }
    #[doc = "Bit 13 - ch45 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr45(&mut self) -> CH_CLR45_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR45_W::new(self, 13)
    }
    #[doc = "Bit 14 - ch46 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr46(&mut self) -> CH_CLR46_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR46_W::new(self, 14)
    }
    #[doc = "Bit 15 - ch47 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr47(&mut self) -> CH_CLR47_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR47_W::new(self, 15)
    }
    #[doc = "Bit 16 - ch48 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr48(&mut self) -> CH_CLR48_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR48_W::new(self, 16)
    }
    #[doc = "Bit 17 - ch49 clear"]
    #[inline(always)]
    #[must_use]
    pub fn ch_clr49(&mut self) -> CH_CLR49_W<CH_ENA_AD1_CLR_SPEC> {
        CH_CLR49_W::new(self, 17)
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
#[doc = "channel enable clear register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD1_CLR_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad1_clr::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD1_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD1_CLR to value 0"]
impl crate::Resettable for CH_ENA_AD1_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
