#[doc = "Register `XTS_AES_KEY_UPDATE` reader"]
pub struct R(crate::R<XTS_AES_KEY_UPDATE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XTS_AES_KEY_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XTS_AES_KEY_UPDATE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XTS_AES_KEY_UPDATE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XTS_AES_KEY_UPDATE` writer"]
pub struct W(crate::W<XTS_AES_KEY_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XTS_AES_KEY_UPDATE_SPEC>;
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
impl From<crate::W<XTS_AES_KEY_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XTS_AES_KEY_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XTS_AES_KEY_UPDATE` reader - Set this bit to update xts_aes key"]
pub type XTS_AES_KEY_UPDATE_R = crate::BitReader;
#[doc = "Field `XTS_AES_KEY_UPDATE` writer - Set this bit to update xts_aes key"]
pub type XTS_AES_KEY_UPDATE_W<'a, const O: u8> = crate::BitWriter<'a, XTS_AES_KEY_UPDATE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit to update xts_aes key"]
    #[inline(always)]
    pub fn xts_aes_key_update(&self) -> XTS_AES_KEY_UPDATE_R {
        XTS_AES_KEY_UPDATE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("XTS_AES_KEY_UPDATE")
            .field(
                "xts_aes_key_update",
                &format_args!("{}", self.xts_aes_key_update().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<XTS_AES_KEY_UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to update xts_aes key"]
    #[inline(always)]
    #[must_use]
    pub fn xts_aes_key_update(&mut self) -> XTS_AES_KEY_UPDATE_W<0> {
        XTS_AES_KEY_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "register description\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xts_aes_key_update](index.html) module"]
pub struct XTS_AES_KEY_UPDATE_SPEC;
impl crate::RegisterSpec for XTS_AES_KEY_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xts_aes_key_update::R](R) reader structure"]
impl crate::Readable for XTS_AES_KEY_UPDATE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xts_aes_key_update::W](W) writer structure"]
impl crate::Writable for XTS_AES_KEY_UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTS_AES_KEY_UPDATE to value 0"]
impl crate::Resettable for XTS_AES_KEY_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
