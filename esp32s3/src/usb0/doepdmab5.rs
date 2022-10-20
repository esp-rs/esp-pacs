#[doc = "Register `DOEPDMAB5` reader"]
pub struct R(crate::R<DOEPDMAB5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEPDMAB5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEPDMAB5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEPDMAB5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEPDMAB5` writer"]
pub struct W(crate::W<DOEPDMAB5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEPDMAB5_SPEC>;
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
impl From<crate::W<DOEPDMAB5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEPDMAB5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DMABUFFERADDR5` reader - "]
pub type DMABUFFERADDR5_R = crate::FieldReader<u32, u32>;
#[doc = "Field `DMABUFFERADDR5` writer - "]
pub type DMABUFFERADDR5_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, DOEPDMAB5_SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr5(&self) -> DMABUFFERADDR5_R {
        DMABUFFERADDR5_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn dmabufferaddr5(&mut self) -> DMABUFFERADDR5_W<0> {
        DMABUFFERADDR5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doepdmab5](index.html) module"]
pub struct DOEPDMAB5_SPEC;
impl crate::RegisterSpec for DOEPDMAB5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doepdmab5::R](R) reader structure"]
impl crate::Readable for DOEPDMAB5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doepdmab5::W](W) writer structure"]
impl crate::Writable for DOEPDMAB5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEPDMAB5 to value 0"]
impl crate::Resettable for DOEPDMAB5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
