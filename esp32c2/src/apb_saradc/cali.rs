#[doc = "Register `CALI` reader"]
pub struct R(crate::R<CALI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALI` writer"]
pub struct W(crate::W<CALI_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALI_SPEC>;
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
impl From<crate::W<CALI_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALI_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CFG` reader - Need add description"]
pub type CFG_R = crate::FieldReader<u32, u32>;
#[doc = "Field `CFG` writer - Need add description"]
pub type CFG_W<'a> = crate::FieldWriter<'a, u32, CALI_SPEC, u32, u32, 17, 0>;
impl R {
    #[doc = "Bits 0:16 - Need add description"]
    #[inline(always)]
    pub fn cfg(&self) -> CFG_R {
        CFG_R::new((self.bits & 0x0001_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:16 - Need add description"]
    #[inline(always)]
    pub fn cfg(&mut self) -> CFG_W {
        CFG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cali](index.html) module"]
pub struct CALI_SPEC;
impl crate::RegisterSpec for CALI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cali::R](R) reader structure"]
impl crate::Readable for CALI_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cali::W](W) writer structure"]
impl crate::Writable for CALI_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALI to value 0x8000"]
impl crate::Resettable for CALI_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x8000
    }
}
