#[doc = "Register `COMP2_LOAD` writer"]
pub struct W(crate::W<COMP2_LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_LOAD_SPEC>;
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
impl From<crate::W<COMP2_LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_COMP2_LOAD` writer - timer comp2 sync enable signal"]
pub type TIMER_COMP2_LOAD_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_LOAD_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<COMP2_LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - timer comp2 sync enable signal"]
    #[inline(always)]
    #[must_use]
    pub fn timer_comp2_load(&mut self) -> TIMER_COMP2_LOAD_W<0> {
        TIMER_COMP2_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer comp2 conf sync register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_load](index.html) module"]
pub struct COMP2_LOAD_SPEC;
impl crate::RegisterSpec for COMP2_LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [comp2_load::W](W) writer structure"]
impl crate::Writable for COMP2_LOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP2_LOAD to value 0"]
impl crate::Resettable for COMP2_LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
