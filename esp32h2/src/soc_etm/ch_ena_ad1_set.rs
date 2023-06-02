#[doc = "Register `CH_ENA_AD1_SET` writer"]
pub struct W(crate::W<CH_ENA_AD1_SET_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_ENA_AD1_SET_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH_ENA_AD1_SET_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_ENA_AD1_SET_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH_SET32` writer - ch32 set"]
pub type CH_SET32_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET33` writer - ch33 set"]
pub type CH_SET33_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET34` writer - ch34 set"]
pub type CH_SET34_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET35` writer - ch35 set"]
pub type CH_SET35_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET36` writer - ch36 set"]
pub type CH_SET36_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET37` writer - ch37 set"]
pub type CH_SET37_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET38` writer - ch38 set"]
pub type CH_SET38_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET39` writer - ch39 set"]
pub type CH_SET39_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET40` writer - ch40 set"]
pub type CH_SET40_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET41` writer - ch41 set"]
pub type CH_SET41_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET42` writer - ch42 set"]
pub type CH_SET42_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET43` writer - ch43 set"]
pub type CH_SET43_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET44` writer - ch44 set"]
pub type CH_SET44_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET45` writer - ch45 set"]
pub type CH_SET45_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET46` writer - ch46 set"]
pub type CH_SET46_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET47` writer - ch47 set"]
pub type CH_SET47_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET48` writer - ch48 set"]
pub type CH_SET48_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[doc = "Field `CH_SET49` writer - ch49 set"]
pub type CH_SET49_W<'a, const O: u8> = crate::BitWriter<'a, CH_ENA_AD1_SET_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_ENA_AD1_SET_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - ch32 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set32(&mut self) -> CH_SET32_W<0> {
        CH_SET32_W::new(self)
    }
    #[doc = "Bit 1 - ch33 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set33(&mut self) -> CH_SET33_W<1> {
        CH_SET33_W::new(self)
    }
    #[doc = "Bit 2 - ch34 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set34(&mut self) -> CH_SET34_W<2> {
        CH_SET34_W::new(self)
    }
    #[doc = "Bit 3 - ch35 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set35(&mut self) -> CH_SET35_W<3> {
        CH_SET35_W::new(self)
    }
    #[doc = "Bit 4 - ch36 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set36(&mut self) -> CH_SET36_W<4> {
        CH_SET36_W::new(self)
    }
    #[doc = "Bit 5 - ch37 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set37(&mut self) -> CH_SET37_W<5> {
        CH_SET37_W::new(self)
    }
    #[doc = "Bit 6 - ch38 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set38(&mut self) -> CH_SET38_W<6> {
        CH_SET38_W::new(self)
    }
    #[doc = "Bit 7 - ch39 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set39(&mut self) -> CH_SET39_W<7> {
        CH_SET39_W::new(self)
    }
    #[doc = "Bit 8 - ch40 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set40(&mut self) -> CH_SET40_W<8> {
        CH_SET40_W::new(self)
    }
    #[doc = "Bit 9 - ch41 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set41(&mut self) -> CH_SET41_W<9> {
        CH_SET41_W::new(self)
    }
    #[doc = "Bit 10 - ch42 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set42(&mut self) -> CH_SET42_W<10> {
        CH_SET42_W::new(self)
    }
    #[doc = "Bit 11 - ch43 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set43(&mut self) -> CH_SET43_W<11> {
        CH_SET43_W::new(self)
    }
    #[doc = "Bit 12 - ch44 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set44(&mut self) -> CH_SET44_W<12> {
        CH_SET44_W::new(self)
    }
    #[doc = "Bit 13 - ch45 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set45(&mut self) -> CH_SET45_W<13> {
        CH_SET45_W::new(self)
    }
    #[doc = "Bit 14 - ch46 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set46(&mut self) -> CH_SET46_W<14> {
        CH_SET46_W::new(self)
    }
    #[doc = "Bit 15 - ch47 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set47(&mut self) -> CH_SET47_W<15> {
        CH_SET47_W::new(self)
    }
    #[doc = "Bit 16 - ch48 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set48(&mut self) -> CH_SET48_W<16> {
        CH_SET48_W::new(self)
    }
    #[doc = "Bit 17 - ch49 set"]
    #[inline(always)]
    #[must_use]
    pub fn ch_set49(&mut self) -> CH_SET49_W<17> {
        CH_SET49_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "channel enable set register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_ena_ad1_set](index.html) module"]
pub struct CH_ENA_AD1_SET_SPEC;
impl crate::RegisterSpec for CH_ENA_AD1_SET_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ch_ena_ad1_set::W](W) writer structure"]
impl crate::Writable for CH_ENA_AD1_SET_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH_ENA_AD1_SET to value 0"]
impl crate::Resettable for CH_ENA_AD1_SET_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
