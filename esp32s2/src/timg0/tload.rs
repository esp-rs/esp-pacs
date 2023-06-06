#[doc = "Register `T%sLOAD` writer"]
pub struct W(crate::W<TLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TLOAD_SPEC>;
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
impl From<crate::W<TLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` writer - Write any value to trigger a timer %s time-base counter reload."]
pub type LOAD_W<'a, const O: u8> = crate::FieldWriter<'a, TLOAD_SPEC, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TLOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value to trigger a timer %s time-base counter reload."]
    #[inline(always)]
    #[must_use]
    pub fn load(&mut self) -> LOAD_W<0> {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Write to reload timer from TIMG_T%sLOADLO_REG or TIMG_T%sLOADHI_REG\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tload](index.html) module"]
pub struct TLOAD_SPEC;
impl crate::RegisterSpec for TLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [tload::W](W) writer structure"]
impl crate::Writable for TLOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T%sLOAD to value 0"]
impl crate::Resettable for TLOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
