#[doc = "Register `TEXT_%s` reader"]
pub struct R(crate::R<TEXT__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEXT__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEXT__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEXT__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEXT_%s` writer"]
pub struct W(crate::W<TEXT__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXT__SPEC>;
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
impl From<crate::W<TEXT__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXT__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXT` reader - SHA Message block and hash result register."]
pub type TEXT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TEXT` writer - SHA Message block and hash result register."]
pub type TEXT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, TEXT__SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - SHA Message block and hash result register."]
    #[inline(always)]
    pub fn text(&self) -> TEXT_R {
        TEXT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SHA Message block and hash result register."]
    #[inline(always)]
    pub fn text(&mut self) -> TEXT_W<0> {
        TEXT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [text_](index.html) module"]
pub struct TEXT__SPEC;
impl crate::RegisterSpec for TEXT__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [text_::R](R) reader structure"]
impl crate::Readable for TEXT__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [text_::W](W) writer structure"]
impl crate::Writable for TEXT__SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets TEXT_%s to value 0"]
impl crate::Resettable for TEXT__SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
