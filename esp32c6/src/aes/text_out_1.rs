#[doc = "Register `TEXT_OUT_1` reader"]
pub struct R(crate::R<TEXT_OUT_1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TEXT_OUT_1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TEXT_OUT_1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TEXT_OUT_1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `TEXT_OUT_1` writer"]
pub struct W(crate::W<TEXT_OUT_1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<TEXT_OUT_1_SPEC>;
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
impl From<crate::W<TEXT_OUT_1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<TEXT_OUT_1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TEXT_OUT_1` reader - This bits stores text_out_1 that is a part of result text material."]
pub type TEXT_OUT_1_R = crate::FieldReader<u32>;
#[doc = "Field `TEXT_OUT_1` writer - This bits stores text_out_1 that is a part of result text material."]
pub type TEXT_OUT_1_W<'a, const O: u8> = crate::FieldWriter<'a, TEXT_OUT_1_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores text_out_1 that is a part of result text material."]
    #[inline(always)]
    pub fn text_out_1(&self) -> TEXT_OUT_1_R {
        TEXT_OUT_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TEXT_OUT_1")
            .field("text_out_1", &format_args!("{}", self.text_out_1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TEXT_OUT_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores text_out_1 that is a part of result text material."]
    #[inline(always)]
    #[must_use]
    pub fn text_out_1(&mut self) -> TEXT_OUT_1_W<0> {
        TEXT_OUT_1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "result text material text_out_1 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [text_out_1](index.html) module"]
pub struct TEXT_OUT_1_SPEC;
impl crate::RegisterSpec for TEXT_OUT_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [text_out_1::R](R) reader structure"]
impl crate::Readable for TEXT_OUT_1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [text_out_1::W](W) writer structure"]
impl crate::Writable for TEXT_OUT_1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TEXT_OUT_1 to value 0"]
impl crate::Resettable for TEXT_OUT_1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
