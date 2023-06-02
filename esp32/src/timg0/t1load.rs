#[doc = "Register `T1LOAD` writer"]
pub struct W(crate::W<T1LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1LOAD_SPEC>;
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
impl From<crate::W<T1LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` writer - Write any value will trigger timer 1 time-base counter reload"]
pub type LOAD_W<'a, const O: u8> = crate::FieldWriter<'a, T1LOAD_SPEC, 32, O, u32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T1LOAD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value will trigger timer 1 time-base counter reload"]
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
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1load](index.html) module"]
pub struct T1LOAD_SPEC;
impl crate::RegisterSpec for T1LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [t1load::W](W) writer structure"]
impl crate::Writable for T1LOAD_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T1LOAD to value 0"]
impl crate::Resettable for T1LOAD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
