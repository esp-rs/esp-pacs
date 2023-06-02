#[doc = "Register `PIF_ACCESS_MONITOR_1` reader"]
pub struct R(crate::R<PIF_ACCESS_MONITOR_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PIF_ACCESS_MONITOR_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PIF_ACCESS_MONITOR_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PIF_ACCESS_MONITOR_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PIF_ACCESS_MONITOR_1` writer"]
pub struct W(crate::W<PIF_ACCESS_MONITOR_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PIF_ACCESS_MONITOR_1_SPEC>;
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
impl From<crate::W<PIF_ACCESS_MONITOR_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PIF_ACCESS_MONITOR_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_R = crate::BitReader;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR` writer - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, PIF_ACCESS_MONITOR_1_SPEC, O>;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN` reader - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_R = crate::BitReader;
#[doc = "Field `PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN` writer - Need add description"]
pub type PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_W<'a, const O: u8> =
    crate::BitWriter<'a, PIF_ACCESS_MONITOR_1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_clr(
        &self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_R {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    pub fn pif_access_monitor_nonword_violate_en(&self) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_R {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PIF_ACCESS_MONITOR_1")
            .field(
                "pif_access_monitor_nonword_violate_clr",
                &format_args!("{}", self.pif_access_monitor_nonword_violate_clr().bit()),
            )
            .field(
                "pif_access_monitor_nonword_violate_en",
                &format_args!("{}", self.pif_access_monitor_nonword_violate_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PIF_ACCESS_MONITOR_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn pif_access_monitor_nonword_violate_clr(
        &mut self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_W<0> {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Need add description"]
    #[inline(always)]
    #[must_use]
    pub fn pif_access_monitor_nonword_violate_en(
        &mut self,
    ) -> PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_W<1> {
        PIF_ACCESS_MONITOR_NONWORD_VIOLATE_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pif_access_monitor_1](index.html) module"]
pub struct PIF_ACCESS_MONITOR_1_SPEC;
impl crate::RegisterSpec for PIF_ACCESS_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pif_access_monitor_1::R](R) reader structure"]
impl crate::Readable for PIF_ACCESS_MONITOR_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pif_access_monitor_1::W](W) writer structure"]
impl crate::Writable for PIF_ACCESS_MONITOR_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIF_ACCESS_MONITOR_1 to value 0x03"]
impl crate::Resettable for PIF_ACCESS_MONITOR_1_SPEC {
    const RESET_VALUE: Self::Ux = 0x03;
}
