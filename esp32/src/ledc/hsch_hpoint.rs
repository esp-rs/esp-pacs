#[doc = "Register `HSCH%s_HPOINT` reader"]
pub struct R(crate::R<HSCH_HPOINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HSCH_HPOINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HSCH_HPOINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HSCH_HPOINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HSCH%s_HPOINT` writer"]
pub struct W(crate::W<HSCH_HPOINT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HSCH_HPOINT_SPEC>;
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
impl From<crate::W<HSCH_HPOINT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HSCH_HPOINT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HPOINT` reader - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
pub type HPOINT_R = crate::FieldReader<u32, u32>;
#[doc = "Field `HPOINT` writer - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
pub type HPOINT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, HSCH_HPOINT_SPEC, u32, u32, 20, O>;
impl R {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint(&self) -> HPOINT_R {
        HPOINT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - The output value changes to high when htimerx(x=\\[0 3\\]) selected by high speed channel0 has reached reg_hpoint_hsch0\\[19:0\\]"]
    #[inline(always)]
    pub fn hpoint(&mut self) -> HPOINT_W<0> {
        HPOINT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hsch_hpoint](index.html) module"]
pub struct HSCH_HPOINT_SPEC;
impl crate::RegisterSpec for HSCH_HPOINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hsch_hpoint::R](R) reader structure"]
impl crate::Readable for HSCH_HPOINT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hsch_hpoint::W](W) writer structure"]
impl crate::Writable for HSCH_HPOINT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets HSCH%s_HPOINT to value 0"]
impl crate::Resettable for HSCH_HPOINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
