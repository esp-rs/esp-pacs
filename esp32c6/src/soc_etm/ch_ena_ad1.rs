#[doc = "Register `CH_ENA_AD1` reader"]
pub type R = crate::R<CH_ENA_AD1_SPEC>;
#[doc = "Register `CH_ENA_AD1` writer"]
pub type W = crate::W<CH_ENA_AD1_SPEC>;
#[doc = "Field `CH_ENA32` reader - ch32 enable"]
pub type CH_ENA32_R = crate::BitReader;
#[doc = "Field `CH_ENA32` writer - ch32 enable"]
pub type CH_ENA32_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA33` reader - ch33 enable"]
pub type CH_ENA33_R = crate::BitReader;
#[doc = "Field `CH_ENA33` writer - ch33 enable"]
pub type CH_ENA33_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA34` reader - ch34 enable"]
pub type CH_ENA34_R = crate::BitReader;
#[doc = "Field `CH_ENA34` writer - ch34 enable"]
pub type CH_ENA34_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA35` reader - ch35 enable"]
pub type CH_ENA35_R = crate::BitReader;
#[doc = "Field `CH_ENA35` writer - ch35 enable"]
pub type CH_ENA35_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA36` reader - ch36 enable"]
pub type CH_ENA36_R = crate::BitReader;
#[doc = "Field `CH_ENA36` writer - ch36 enable"]
pub type CH_ENA36_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA37` reader - ch37 enable"]
pub type CH_ENA37_R = crate::BitReader;
#[doc = "Field `CH_ENA37` writer - ch37 enable"]
pub type CH_ENA37_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA38` reader - ch38 enable"]
pub type CH_ENA38_R = crate::BitReader;
#[doc = "Field `CH_ENA38` writer - ch38 enable"]
pub type CH_ENA38_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA39` reader - ch39 enable"]
pub type CH_ENA39_R = crate::BitReader;
#[doc = "Field `CH_ENA39` writer - ch39 enable"]
pub type CH_ENA39_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA40` reader - ch40 enable"]
pub type CH_ENA40_R = crate::BitReader;
#[doc = "Field `CH_ENA40` writer - ch40 enable"]
pub type CH_ENA40_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA41` reader - ch41 enable"]
pub type CH_ENA41_R = crate::BitReader;
#[doc = "Field `CH_ENA41` writer - ch41 enable"]
pub type CH_ENA41_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA42` reader - ch42 enable"]
pub type CH_ENA42_R = crate::BitReader;
#[doc = "Field `CH_ENA42` writer - ch42 enable"]
pub type CH_ENA42_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA43` reader - ch43 enable"]
pub type CH_ENA43_R = crate::BitReader;
#[doc = "Field `CH_ENA43` writer - ch43 enable"]
pub type CH_ENA43_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA44` reader - ch44 enable"]
pub type CH_ENA44_R = crate::BitReader;
#[doc = "Field `CH_ENA44` writer - ch44 enable"]
pub type CH_ENA44_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA45` reader - ch45 enable"]
pub type CH_ENA45_R = crate::BitReader;
#[doc = "Field `CH_ENA45` writer - ch45 enable"]
pub type CH_ENA45_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA46` reader - ch46 enable"]
pub type CH_ENA46_R = crate::BitReader;
#[doc = "Field `CH_ENA46` writer - ch46 enable"]
pub type CH_ENA46_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA47` reader - ch47 enable"]
pub type CH_ENA47_R = crate::BitReader;
#[doc = "Field `CH_ENA47` writer - ch47 enable"]
pub type CH_ENA47_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA48` reader - ch48 enable"]
pub type CH_ENA48_R = crate::BitReader;
#[doc = "Field `CH_ENA48` writer - ch48 enable"]
pub type CH_ENA48_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH_ENA49` reader - ch49 enable"]
pub type CH_ENA49_R = crate::BitReader;
#[doc = "Field `CH_ENA49` writer - ch49 enable"]
pub type CH_ENA49_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - ch32 enable"]
    #[inline(always)]
    pub fn ch_ena32(&self) -> CH_ENA32_R {
        CH_ENA32_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ch33 enable"]
    #[inline(always)]
    pub fn ch_ena33(&self) -> CH_ENA33_R {
        CH_ENA33_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ch34 enable"]
    #[inline(always)]
    pub fn ch_ena34(&self) -> CH_ENA34_R {
        CH_ENA34_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ch35 enable"]
    #[inline(always)]
    pub fn ch_ena35(&self) -> CH_ENA35_R {
        CH_ENA35_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ch36 enable"]
    #[inline(always)]
    pub fn ch_ena36(&self) -> CH_ENA36_R {
        CH_ENA36_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - ch37 enable"]
    #[inline(always)]
    pub fn ch_ena37(&self) -> CH_ENA37_R {
        CH_ENA37_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - ch38 enable"]
    #[inline(always)]
    pub fn ch_ena38(&self) -> CH_ENA38_R {
        CH_ENA38_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - ch39 enable"]
    #[inline(always)]
    pub fn ch_ena39(&self) -> CH_ENA39_R {
        CH_ENA39_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ch40 enable"]
    #[inline(always)]
    pub fn ch_ena40(&self) -> CH_ENA40_R {
        CH_ENA40_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - ch41 enable"]
    #[inline(always)]
    pub fn ch_ena41(&self) -> CH_ENA41_R {
        CH_ENA41_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - ch42 enable"]
    #[inline(always)]
    pub fn ch_ena42(&self) -> CH_ENA42_R {
        CH_ENA42_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ch43 enable"]
    #[inline(always)]
    pub fn ch_ena43(&self) -> CH_ENA43_R {
        CH_ENA43_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - ch44 enable"]
    #[inline(always)]
    pub fn ch_ena44(&self) -> CH_ENA44_R {
        CH_ENA44_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - ch45 enable"]
    #[inline(always)]
    pub fn ch_ena45(&self) -> CH_ENA45_R {
        CH_ENA45_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ch46 enable"]
    #[inline(always)]
    pub fn ch_ena46(&self) -> CH_ENA46_R {
        CH_ENA46_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - ch47 enable"]
    #[inline(always)]
    pub fn ch_ena47(&self) -> CH_ENA47_R {
        CH_ENA47_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - ch48 enable"]
    #[inline(always)]
    pub fn ch_ena48(&self) -> CH_ENA48_R {
        CH_ENA48_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - ch49 enable"]
    #[inline(always)]
    pub fn ch_ena49(&self) -> CH_ENA49_R {
        CH_ENA49_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_ENA_AD1")
            .field("ch_ena32", &format_args!("{}", self.ch_ena32().bit()))
            .field("ch_ena33", &format_args!("{}", self.ch_ena33().bit()))
            .field("ch_ena34", &format_args!("{}", self.ch_ena34().bit()))
            .field("ch_ena35", &format_args!("{}", self.ch_ena35().bit()))
            .field("ch_ena36", &format_args!("{}", self.ch_ena36().bit()))
            .field("ch_ena37", &format_args!("{}", self.ch_ena37().bit()))
            .field("ch_ena38", &format_args!("{}", self.ch_ena38().bit()))
            .field("ch_ena39", &format_args!("{}", self.ch_ena39().bit()))
            .field("ch_ena40", &format_args!("{}", self.ch_ena40().bit()))
            .field("ch_ena41", &format_args!("{}", self.ch_ena41().bit()))
            .field("ch_ena42", &format_args!("{}", self.ch_ena42().bit()))
            .field("ch_ena43", &format_args!("{}", self.ch_ena43().bit()))
            .field("ch_ena44", &format_args!("{}", self.ch_ena44().bit()))
            .field("ch_ena45", &format_args!("{}", self.ch_ena45().bit()))
            .field("ch_ena46", &format_args!("{}", self.ch_ena46().bit()))
            .field("ch_ena47", &format_args!("{}", self.ch_ena47().bit()))
            .field("ch_ena48", &format_args!("{}", self.ch_ena48().bit()))
            .field("ch_ena49", &format_args!("{}", self.ch_ena49().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - ch32 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena32(&mut self) -> CH_ENA32_W<CH_ENA_AD1_SPEC> {
        CH_ENA32_W::new(self, 0)
    }
    #[doc = "Bit 1 - ch33 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena33(&mut self) -> CH_ENA33_W<CH_ENA_AD1_SPEC> {
        CH_ENA33_W::new(self, 1)
    }
    #[doc = "Bit 2 - ch34 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena34(&mut self) -> CH_ENA34_W<CH_ENA_AD1_SPEC> {
        CH_ENA34_W::new(self, 2)
    }
    #[doc = "Bit 3 - ch35 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena35(&mut self) -> CH_ENA35_W<CH_ENA_AD1_SPEC> {
        CH_ENA35_W::new(self, 3)
    }
    #[doc = "Bit 4 - ch36 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena36(&mut self) -> CH_ENA36_W<CH_ENA_AD1_SPEC> {
        CH_ENA36_W::new(self, 4)
    }
    #[doc = "Bit 5 - ch37 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena37(&mut self) -> CH_ENA37_W<CH_ENA_AD1_SPEC> {
        CH_ENA37_W::new(self, 5)
    }
    #[doc = "Bit 6 - ch38 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena38(&mut self) -> CH_ENA38_W<CH_ENA_AD1_SPEC> {
        CH_ENA38_W::new(self, 6)
    }
    #[doc = "Bit 7 - ch39 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena39(&mut self) -> CH_ENA39_W<CH_ENA_AD1_SPEC> {
        CH_ENA39_W::new(self, 7)
    }
    #[doc = "Bit 8 - ch40 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena40(&mut self) -> CH_ENA40_W<CH_ENA_AD1_SPEC> {
        CH_ENA40_W::new(self, 8)
    }
    #[doc = "Bit 9 - ch41 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena41(&mut self) -> CH_ENA41_W<CH_ENA_AD1_SPEC> {
        CH_ENA41_W::new(self, 9)
    }
    #[doc = "Bit 10 - ch42 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena42(&mut self) -> CH_ENA42_W<CH_ENA_AD1_SPEC> {
        CH_ENA42_W::new(self, 10)
    }
    #[doc = "Bit 11 - ch43 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena43(&mut self) -> CH_ENA43_W<CH_ENA_AD1_SPEC> {
        CH_ENA43_W::new(self, 11)
    }
    #[doc = "Bit 12 - ch44 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena44(&mut self) -> CH_ENA44_W<CH_ENA_AD1_SPEC> {
        CH_ENA44_W::new(self, 12)
    }
    #[doc = "Bit 13 - ch45 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena45(&mut self) -> CH_ENA45_W<CH_ENA_AD1_SPEC> {
        CH_ENA45_W::new(self, 13)
    }
    #[doc = "Bit 14 - ch46 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena46(&mut self) -> CH_ENA46_W<CH_ENA_AD1_SPEC> {
        CH_ENA46_W::new(self, 14)
    }
    #[doc = "Bit 15 - ch47 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena47(&mut self) -> CH_ENA47_W<CH_ENA_AD1_SPEC> {
        CH_ENA47_W::new(self, 15)
    }
    #[doc = "Bit 16 - ch48 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena48(&mut self) -> CH_ENA48_W<CH_ENA_AD1_SPEC> {
        CH_ENA48_W::new(self, 16)
    }
    #[doc = "Bit 17 - ch49 enable"]
    #[inline(always)]
    #[must_use]
    pub fn ch_ena49(&mut self) -> CH_ENA49_W<CH_ENA_AD1_SPEC> {
        CH_ENA49_W::new(self, 17)
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
#[doc = "channel enable register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ch_ena_ad1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ch_ena_ad1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CH_ENA_AD1_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch_ena_ad1::R`](R) reader structure"]
impl crate::Readable for CH_ENA_AD1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ch_ena_ad1::W`](W) writer structure"]
impl crate::Writable for CH_ENA_AD1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD1 to value 0"]
impl crate::Resettable for CH_ENA_AD1_SPEC {
    const RESET_VALUE: u32 = 0;
}
