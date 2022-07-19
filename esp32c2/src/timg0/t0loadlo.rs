#[doc = "Register `T0LOADLO` reader"]
pub struct R(crate::R<T0LOADLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<T0LOADLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<T0LOADLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<T0LOADLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T0LOADLO` writer"]
pub struct W(crate::W<T0LOADLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<T0LOADLO_SPEC>;
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
impl From<crate::W<T0LOADLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<T0LOADLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `T0_LOAD_LO` reader - Low 32 bits of the value that a reload will load onto timer 0 time-base Counter."]
pub type T0_LOAD_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `T0_LOAD_LO` writer - Low 32 bits of the value that a reload will load onto timer 0 time-base Counter."]
pub type T0_LOAD_LO_W<'a> = crate::FieldWriter<'a, u32, T0LOADLO_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Low 32 bits of the value that a reload will load onto timer 0 time-base Counter."]
    #[inline(always)]
    pub fn t0_load_lo(&self) -> T0_LOAD_LO_R {
        T0_LOAD_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low 32 bits of the value that a reload will load onto timer 0 time-base Counter."]
    #[inline(always)]
    pub fn t0_load_lo(&mut self) -> T0_LOAD_LO_W {
        T0_LOAD_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer $x reload value, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [t0loadlo](index.html) module"]
pub struct T0LOADLO_SPEC;
impl crate::RegisterSpec for T0LOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [t0loadlo::R](R) reader structure"]
impl crate::Readable for T0LOADLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [t0loadlo::W](W) writer structure"]
impl crate::Writable for T0LOADLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T0LOADLO to value 0"]
impl crate::Resettable for T0LOADLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
