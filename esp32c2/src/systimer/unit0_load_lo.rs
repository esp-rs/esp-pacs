#[doc = "Register `UNIT0_LOAD_LO` reader"]
pub struct R(crate::R<UNIT0_LOAD_LO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIT0_LOAD_LO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIT0_LOAD_LO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIT0_LOAD_LO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `UNIT0_LOAD_LO` writer"]
pub struct W(crate::W<UNIT0_LOAD_LO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<UNIT0_LOAD_LO_SPEC>;
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
impl From<crate::W<UNIT0_LOAD_LO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<UNIT0_LOAD_LO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TIMER_UNIT0_LOAD_LO` reader - timer unit0 load low 32 bits"]
pub type TIMER_UNIT0_LOAD_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TIMER_UNIT0_LOAD_LO` writer - timer unit0 load low 32 bits"]
pub type TIMER_UNIT0_LOAD_LO_W<'a> =
    crate::FieldWriter<'a, u32, UNIT0_LOAD_LO_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - timer unit0 load low 32 bits"]
    #[inline(always)]
    pub fn timer_unit0_load_lo(&self) -> TIMER_UNIT0_LOAD_LO_R {
        TIMER_UNIT0_LOAD_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - timer unit0 load low 32 bits"]
    #[inline(always)]
    pub fn timer_unit0_load_lo(&mut self) -> TIMER_UNIT0_LOAD_LO_W {
        TIMER_UNIT0_LOAD_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "system timer unit0 value low load register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit0_load_lo](index.html) module"]
pub struct UNIT0_LOAD_LO_SPEC;
impl crate::RegisterSpec for UNIT0_LOAD_LO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unit0_load_lo::R](R) reader structure"]
impl crate::Readable for UNIT0_LOAD_LO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [unit0_load_lo::W](W) writer structure"]
impl crate::Writable for UNIT0_LOAD_LO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets UNIT0_LOAD_LO to value 0"]
impl crate::Resettable for UNIT0_LOAD_LO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
