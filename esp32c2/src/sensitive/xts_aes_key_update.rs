#[doc = "Register `XTS_AES_KEY_UPDATE` reader"]
pub type R = crate::R<XTS_AES_KEY_UPDATE_SPEC>;
#[doc = "Register `XTS_AES_KEY_UPDATE` writer"]
pub type W = crate::W<XTS_AES_KEY_UPDATE_SPEC>;
#[doc = "Field `XTS_AES_KEY_UPDATE` reader - Set this bit to update xts_aes key"]
pub type XTS_AES_KEY_UPDATE_R = crate::BitReader;
#[doc = "Field `XTS_AES_KEY_UPDATE` writer - Set this bit to update xts_aes key"]
pub type XTS_AES_KEY_UPDATE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub fn xts_aes_key_update(&mut self) -> XTS_AES_KEY_UPDATE_W<XTS_AES_KEY_UPDATE_SPEC, 0> {
        XTS_AES_KEY_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`xts_aes_key_update::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`xts_aes_key_update::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XTS_AES_KEY_UPDATE_SPEC;
impl crate::RegisterSpec for XTS_AES_KEY_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`xts_aes_key_update::R`](R) reader structure"]
impl crate::Readable for XTS_AES_KEY_UPDATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`xts_aes_key_update::W`](W) writer structure"]
impl crate::Writable for XTS_AES_KEY_UPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets XTS_AES_KEY_UPDATE to value 0"]
impl crate::Resettable for XTS_AES_KEY_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
