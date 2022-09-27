#[doc = "Register `TEXT_OUT_%s` reader"]
pub struct R(crate::R<TEXT_OUT__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEXT_OUT__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEXT_OUT__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEXT_OUT__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEXT_OUT_%s` writer"]
pub struct W(crate::W<TEXT_OUT__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXT_OUT__SPEC>;
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
impl From<crate::W<TEXT_OUT__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXT_OUT__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXT_OUT_0` reader - Stores the result data when the AES accelerator operates in the Typical AES working mode."]
pub type TEXT_OUT_0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `TEXT_OUT_0` writer - Stores the result data when the AES accelerator operates in the Typical AES working mode."]
pub type TEXT_OUT_0_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, TEXT_OUT__SPEC, u32, u32, 32, O>;
impl R {
    #[doc = "Bits 0:31 - Stores the result data when the AES accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    pub fn text_out_0(&self) -> TEXT_OUT_0_R {
        TEXT_OUT_0_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Stores the result data when the AES accelerator operates in the Typical AES working mode."]
    #[inline(always)]
    pub fn text_out_0(&mut self) -> TEXT_OUT_0_W<0> {
        TEXT_OUT_0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Result data register %s\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [text_out_](index.html) module"]
pub struct TEXT_OUT__SPEC;
impl crate::RegisterSpec for TEXT_OUT__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [text_out_::R](R) reader structure"]
impl crate::Readable for TEXT_OUT__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [text_out_::W](W) writer structure"]
impl crate::Writable for TEXT_OUT__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEXT_OUT_%s to value 0"]
impl crate::Resettable for TEXT_OUT__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
