#[doc = "Register `CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE` reader"]
pub struct R(crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE` writer"]
pub struct W(crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>;
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
impl From<crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RECORD_DISABLE_DB_ENCRYPT` reader - Reserved."]
pub type RECORD_DISABLE_DB_ENCRYPT_R = crate::BitReader;
#[doc = "Field `RECORD_DISABLE_DB_ENCRYPT` writer - Reserved."]
pub type RECORD_DISABLE_DB_ENCRYPT_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC, O>;
#[doc = "Field `RECORD_DISABLE_G0CB_DECRYPT` reader - Reserved."]
pub type RECORD_DISABLE_G0CB_DECRYPT_R = crate::BitReader;
#[doc = "Field `RECORD_DISABLE_G0CB_DECRYPT` writer - Reserved."]
pub type RECORD_DISABLE_G0CB_DECRYPT_W<'a, const O: u8> =
    crate::BitWriter<'a, CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn record_disable_db_encrypt(&self) -> RECORD_DISABLE_DB_ENCRYPT_R {
        RECORD_DISABLE_DB_ENCRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn record_disable_g0cb_decrypt(&self) -> RECORD_DISABLE_G0CB_DECRYPT_R {
        RECORD_DISABLE_G0CB_DECRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE")
            .field(
                "record_disable_db_encrypt",
                &format_args!("{}", self.record_disable_db_encrypt().bit()),
            )
            .field(
                "record_disable_g0cb_decrypt",
                &format_args!("{}", self.record_disable_g0cb_decrypt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn record_disable_db_encrypt(&mut self) -> RECORD_DISABLE_DB_ENCRYPT_W<0> {
        RECORD_DISABLE_DB_ENCRYPT_W::new(self)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn record_disable_g0cb_decrypt(&mut self) -> RECORD_DISABLE_G0CB_DECRYPT_W<1> {
        RECORD_DISABLE_G0CB_DECRYPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cache_encrypt_decrypt_record_disable](index.html) module"]
pub struct CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC;
impl crate::RegisterSpec for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cache_encrypt_decrypt_record_disable::R](R) reader structure"]
impl crate::Readable for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cache_encrypt_decrypt_record_disable::W](W) writer structure"]
impl crate::Writable for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE to value 0"]
impl crate::Resettable for CACHE_ENCRYPT_DECRYPT_RECORD_DISABLE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
