#[doc = "Register `KEY_5` reader"]
pub struct R(crate::R<KEY_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<KEY_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<KEY_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<KEY_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `KEY_5` writer"]
pub struct W(crate::W<KEY_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<KEY_5_SPEC>;
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
impl From<crate::W<KEY_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<KEY_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `KEY_5` reader - This bits stores key_5 that is a part of key material."]
pub type KEY_5_R = crate::FieldReader<u32>;
#[doc = "Field `KEY_5` writer - This bits stores key_5 that is a part of key material."]
pub type KEY_5_W<'a, const O: u8> = crate::FieldWriter<'a, KEY_5_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - This bits stores key_5 that is a part of key material."]
    #[inline(always)]
    pub fn key_5(&self) -> KEY_5_R {
        KEY_5_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_5")
            .field("key_5", &format_args!("{}", self.key_5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<KEY_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - This bits stores key_5 that is a part of key material."]
    #[inline(always)]
    #[must_use]
    pub fn key_5(&mut self) -> KEY_5_W<0> {
        KEY_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Key material key_5 configure register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [key_5](index.html) module"]
pub struct KEY_5_SPEC;
impl crate::RegisterSpec for KEY_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [key_5::R](R) reader structure"]
impl crate::Readable for KEY_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [key_5::W](W) writer structure"]
impl crate::Writable for KEY_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets KEY_5 to value 0"]
impl crate::Resettable for KEY_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
