#[doc = "Register `KEY_%s` reader"]
pub struct R(crate::R<KEY__SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY__SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY__SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY__SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_%s` writer"]
pub struct W(crate::W<KEY__SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY__SPEC>;
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
impl From<crate::W<KEY__SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY__SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY` reader - AES key material register."]
pub type KEY_R = crate::FieldReader;
#[doc = "Field `KEY` writer - AES key material register."]
pub type KEY_W<'a, const O: u8> = crate::FieldWriter<'a, KEY__SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - AES key material register."]
    #[inline(always)]
    pub fn key(&self) -> KEY_R {
        KEY_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_")
            .field("key", &format_args!("{}", self.key().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - AES key material register."]
    #[inline(always)]
    #[must_use]
    pub fn key(&mut self) -> KEY_W<0> {
        KEY_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_](index.html) module"]
pub struct KEY__SPEC;
impl crate::RegisterSpec for KEY__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_::R](R) reader structure"]
impl crate::Readable for KEY__SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_::W](W) writer structure"]
impl crate::Writable for KEY__SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_%s to value 0"]
impl crate::Resettable for KEY__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
