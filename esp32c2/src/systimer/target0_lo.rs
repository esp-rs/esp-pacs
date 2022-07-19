#[doc = "Register `TARGET0_LO` reader"]
pub struct R(crate::R<TARGET0_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TARGET0_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TARGET0_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TARGET0_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TARGET0_LO` writer"]
pub struct W(crate::W<TARGET0_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TARGET0_LO_SPEC>;
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
impl From<crate::W<TARGET0_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TARGET0_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_TARGET0_LO` reader - timer taget0 low 32 bits"]
pub type TIMER_TARGET0_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER_TARGET0_LO` writer - timer taget0 low 32 bits"]
pub type TIMER_TARGET0_LO_W<'a> = crate::FieldWriter<'a, u32, TARGET0_LO_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - timer taget0 low 32 bits"]
    #[inline(always)]
    pub fn timer_target0_lo(&self) -> TIMER_TARGET0_LO_R {
        TIMER_TARGET0_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - timer taget0 low 32 bits"]
    #[inline(always)]
    pub fn timer_target0_lo(&mut self) -> TIMER_TARGET0_LO_W {
        TIMER_TARGET0_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer comp0 value low register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [target0_lo](index.html) module"]
pub struct TARGET0_LO_SPEC;
impl crate::RegisterSpec for TARGET0_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [target0_lo::R](R) reader structure"]
impl crate::Readable for TARGET0_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [target0_lo::W](W) writer structure"]
impl crate::Writable for TARGET0_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TARGET0_LO to value 0"]
impl crate::Resettable for TARGET0_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
