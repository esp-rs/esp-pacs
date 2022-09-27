#[doc = "Register `LACTLOAD` writer"]
pub struct W(crate::W<LACTLOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LACTLOAD_SPEC>;
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
impl From<crate::W<LACTLOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LACTLOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LACT_LOAD` writer - Reserved."]
pub type LACT_LOAD_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LACTLOAD_SPEC, u32, u32, 32, O>;
impl W {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn lact_load(&mut self) -> LACT_LOAD_W<0> {
        LACT_LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer LACT load register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lactload](index.html) module"]
pub struct LACTLOAD_SPEC;
impl crate::RegisterSpec for LACTLOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [lactload::W](W) writer structure"]
impl crate::Writable for LACTLOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LACTLOAD to value 0"]
impl crate::Resettable for LACTLOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
