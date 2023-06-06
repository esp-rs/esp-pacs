#[doc = "Register `KEY_3` reader"]
pub struct R(crate::R<KEY_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_3` writer"]
pub struct W(crate::W<KEY_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_3_SPEC>;
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
impl From<crate::W<KEY_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_3` reader - This bits stores key_3 that is a part of key material."]
pub type KEY_3_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_3` writer - This bits stores key_3 that is a part of key material."]
pub type KEY_3_W<'a, const O: u8> = crate::FieldWriter<'a, KEY_3_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_3 that is a part of key material."]
    #[inline(always)]
    pub fn key_3(&self) -> KEY_3_R {
        KEY_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_3")
            .field("key_3", &format_args!("{}", self.key_3().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_3 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_3(&mut self) -> KEY_3_W<0> {
        KEY_3_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key material key_3 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_3](index.html) module"]
pub struct KEY_3_SPEC;
impl crate::RegisterSpec for KEY_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_3::R](R) reader structure"]
impl crate::Readable for KEY_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_3::W](W) writer structure"]
impl crate::Writable for KEY_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_3 to value 0"]
impl crate::Resettable for KEY_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
