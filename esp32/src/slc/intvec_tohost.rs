#[doc = "Register `INTVEC_TOHOST` writer"]
pub struct W(crate::W<INTVEC_TOHOST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTVEC_TOHOST_SPEC>;
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
impl From<crate::W<INTVEC_TOHOST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTVEC_TOHOST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLC0_TOHOST_INTVEC` writer - "]
pub type SLC0_TOHOST_INTVEC_W<'a, const O: u8> = crate::FieldWriter<'a, INTVEC_TOHOST_SPEC, 8, O>;
#[doc = "Field `SLC1_TOHOST_INTVEC` writer - "]
pub type SLC1_TOHOST_INTVEC_W<'a, const O: u8> = crate::FieldWriter<'a, INTVEC_TOHOST_SPEC, 8, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTVEC_TOHOST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn slc0_tohost_intvec(&mut self) -> SLC0_TOHOST_INTVEC_W<0> {
        SLC0_TOHOST_INTVEC_W::new(self)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn slc1_tohost_intvec(&mut self) -> SLC1_TOHOST_INTVEC_W<16> {
        SLC1_TOHOST_INTVEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [intvec_tohost](index.html) module"]
pub struct INTVEC_TOHOST_SPEC;
impl crate::RegisterSpec for INTVEC_TOHOST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [intvec_tohost::W](W) writer structure"]
impl crate::Writable for INTVEC_TOHOST_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTVEC_TOHOST to value 0"]
impl crate::Resettable for INTVEC_TOHOST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
