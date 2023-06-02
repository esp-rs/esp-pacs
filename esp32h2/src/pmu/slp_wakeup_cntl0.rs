#[doc = "Register `SLP_WAKEUP_CNTL0` writer"]
pub struct W(crate::W<SLP_WAKEUP_CNTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLP_WAKEUP_CNTL0_SPEC>;
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
impl From<crate::W<SLP_WAKEUP_CNTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLP_WAKEUP_CNTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SLEEP_REQ` writer - need_des"]
pub type SLEEP_REQ_W<'a, const O: u8> = crate::BitWriter<'a, SLP_WAKEUP_CNTL0_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLP_WAKEUP_CNTL0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn sleep_req(&mut self) -> SLEEP_REQ_W<31> {
        SLEEP_REQ_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slp_wakeup_cntl0](index.html) module"]
pub struct SLP_WAKEUP_CNTL0_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL0_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [slp_wakeup_cntl0::W](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL0 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
