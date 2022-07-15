#[doc = "Register `T%sLOADLO` reader"]
pub struct R(crate::R<TLOADLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TLOADLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TLOADLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TLOADLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T%sLOADLO` writer"]
pub struct W(crate::W<TLOADLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TLOADLO_SPEC>;
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
impl From<crate::W<TLOADLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TLOADLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOAD_LO` reader - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
pub type LOAD_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `LOAD_LO` writer - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
pub type LOAD_LO_W<'a> = crate::FieldWriter<'a, u32, TLOADLO_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
    #[inline(always)]
    pub fn load_lo(&self) -> LOAD_LO_R {
        LOAD_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Low 32 bits of the value that a reload will load onto timer %s time-base Counter."]
    #[inline(always)]
    pub fn load_lo(&mut self) -> LOAD_LO_W {
        LOAD_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s reload value, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [tloadlo](index.html) module"]
pub struct TLOADLO_SPEC;
impl crate::RegisterSpec for TLOADLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [tloadlo::R](R) reader structure"]
impl crate::Readable for TLOADLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [tloadlo::W](W) writer structure"]
impl crate::Writable for TLOADLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T%sLOADLO to value 0"]
impl crate::Resettable for TLOADLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
