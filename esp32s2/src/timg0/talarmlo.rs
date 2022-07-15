#[doc = "Register `T%sALARMLO` reader"]
pub struct R(crate::R<TALARMLO_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TALARMLO_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TALARMLO_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TALARMLO_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T%sALARMLO` writer"]
pub struct W(crate::W<TALARMLO_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TALARMLO_SPEC>;
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
impl From<crate::W<TALARMLO_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TALARMLO_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARM_LO` reader - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type ALARM_LO_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ALARM_LO` writer - Timer %s alarm trigger time-base counter value, low 32 bits."]
pub type ALARM_LO_W<'a> = crate::FieldWriter<'a, u32, TALARMLO_SPEC, u32, u32, 32, 0>;
impl R {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    pub fn alarm_lo(&self) -> ALARM_LO_R {
        ALARM_LO_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Timer %s alarm trigger time-base counter value, low 32 bits."]
    #[inline(always)]
    pub fn alarm_lo(&mut self) -> ALARM_LO_W {
        ALARM_LO_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s alarm value, low 32 bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [talarmlo](index.html) module"]
pub struct TALARMLO_SPEC;
impl crate::RegisterSpec for TALARMLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [talarmlo::R](R) reader structure"]
impl crate::Readable for TALARMLO_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [talarmlo::W](W) writer structure"]
impl crate::Writable for TALARMLO_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T%sALARMLO to value 0"]
impl crate::Resettable for TALARMLO_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
