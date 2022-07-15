#[doc = "Register `T%sALARMHI` reader"]
pub struct R(crate::R<TALARMHI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TALARMHI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TALARMHI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TALARMHI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `T%sALARMHI` writer"]
pub struct W(crate::W<TALARMHI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TALARMHI_SPEC>;
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
impl From<crate::W<TALARMHI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TALARMHI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ALARM_HI` reader - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub type ALARM_HI_R = crate::FieldReader<u32, u32>;
#[doc = "Field `ALARM_HI` writer - Timer %s alarm trigger time-base counter value, high 22 bits."]
pub type ALARM_HI_W<'a> = crate::FieldWriter<'a, u32, TALARMHI_SPEC, u32, u32, 22, 0>;
impl R {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    pub fn alarm_hi(&self) -> ALARM_HI_R {
        ALARM_HI_R::new((self.bits & 0x003f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:21 - Timer %s alarm trigger time-base counter value, high 22 bits."]
    #[inline(always)]
    pub fn alarm_hi(&mut self) -> ALARM_HI_W {
        ALARM_HI_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Timer %s alarm value, high bits\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [talarmhi](index.html) module"]
pub struct TALARMHI_SPEC;
impl crate::RegisterSpec for TALARMHI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [talarmhi::R](R) reader structure"]
impl crate::Readable for TALARMHI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [talarmhi::W](W) writer structure"]
impl crate::Writable for TALARMHI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets T%sALARMHI to value 0"]
impl crate::Resettable for TALARMHI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
