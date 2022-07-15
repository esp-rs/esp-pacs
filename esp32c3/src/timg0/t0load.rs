#[doc = "Register `T0LOAD` writer"]
pub struct W(crate::W<T0LOAD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0LOAD_SPEC>;
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
impl From<crate::W<T0LOAD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0LOAD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD` writer - t0_load"]
pub type LOAD_W<'a> = crate::FieldWriter<'a, u32, T0LOAD_SPEC, u32, u32, 32, 0>;
impl W {
    #[doc = "Bits 0:31 - t0_load"]
    #[inline(always)]
    pub fn load(&mut self) -> LOAD_W {
        LOAD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_T0LOAD_REG.\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0load](index.html) module"]
pub struct T0LOAD_SPEC;
impl crate::RegisterSpec for T0LOAD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [t0load::W](W) writer structure"]
impl crate::Writable for T0LOAD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0LOAD to value 0"]
impl crate::Resettable for T0LOAD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
