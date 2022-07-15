#[doc = "Register `T1UPDATE` writer"]
pub struct W(crate::W<T1UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T1UPDATE_SPEC>;
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
impl From<crate::W<T1UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T1UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `UPDATE` writer - Write any value will trigger a timer 1 time-base counter value update (timer 1 current value will be stored in registers above)"]
pub type UPDATE_W<'a> = crate::FieldWriter<'a, u32, T1UPDATE_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - Write any value will trigger a timer 1 time-base counter value update (timer 1 current value will be stored in registers above)"]
    #[inline(always)]
    pub fn update(&mut self) -> UPDATE_W {
        UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t1update](index.html) module"]
pub struct T1UPDATE_SPEC;
impl crate::RegisterSpec for T1UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [t1update::W](W) writer structure"]
impl crate::Writable for T1UPDATE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T1UPDATE to value 0"]
impl crate::Resettable for T1UPDATE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
