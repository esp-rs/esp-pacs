#[doc = "Register `SLP_REJECT_CAUSE` reader"]
pub struct R(crate::R<SLP_REJECT_CAUSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLP_REJECT_CAUSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLP_REJECT_CAUSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLP_REJECT_CAUSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLP_REJECT_CAUSE` writer"]
pub struct W(crate::W<SLP_REJECT_CAUSE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_REJECT_CAUSE_SPEC>;
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
impl From<crate::W<SLP_REJECT_CAUSE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_REJECT_CAUSE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REJECT_CAUSE` reader - sleep reject cause"]
pub type REJECT_CAUSE_R = crate::FieldReader<u32>;
#[doc = "Field `REJECT_CAUSE` writer - sleep reject cause"]
pub type REJECT_CAUSE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SLP_REJECT_CAUSE_SPEC, 18, O, u32>;
impl R {
    #[doc = "Bits 0:17 - sleep reject cause"]
    #[inline(always)]
    pub fn reject_cause(&self) -> REJECT_CAUSE_R {
        REJECT_CAUSE_R::new(self.bits & 0x0003_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_REJECT_CAUSE")
            .field(
                "reject_cause",
                &format_args!("{}", self.reject_cause().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_REJECT_CAUSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:17 - sleep reject cause"]
    #[inline(always)]
    #[must_use]
    pub fn reject_cause(&mut self) -> REJECT_CAUSE_W<0> {
        REJECT_CAUSE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_reject_cause](index.html) module"]
pub struct SLP_REJECT_CAUSE_SPEC;
impl crate::RegisterSpec for SLP_REJECT_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slp_reject_cause::R](R) reader structure"]
impl crate::Readable for SLP_REJECT_CAUSE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slp_reject_cause::W](W) writer structure"]
impl crate::Writable for SLP_REJECT_CAUSE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_REJECT_CAUSE to value 0"]
impl crate::Resettable for SLP_REJECT_CAUSE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
