#[doc = "Register `LACTUPDATE` writer"]
pub struct W(crate::W<LACTUPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTUPDATE_SPEC>;
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
impl From<crate::W<LACTUPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTUPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_UPDATE` writer - Reserved."]
pub type LACT_UPDATE_W<'a, const O: u8> = crate::FieldWriter<'a, LACTUPDATE_SPEC, 32, O, u32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LACTUPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn lact_update(&mut self) -> LACT_UPDATE_W<0> {
        LACT_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "LACT update register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactupdate](index.html) module"]
pub struct LACTUPDATE_SPEC;
impl crate::RegisterSpec for LACTUPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lactupdate::W](W) writer structure"]
impl crate::Writable for LACTUPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LACTUPDATE to value 0"]
impl crate::Resettable for LACTUPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
