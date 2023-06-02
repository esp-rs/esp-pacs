#[doc = "Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` reader"]
pub struct R(crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` writer"]
pub struct W(crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
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
impl From<crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ENABLE_SPI_MANUAL_ENCRYPT` reader - Set this bit as 1 to enable mspi xts manual encrypt in spi boot mode."]
pub type ENABLE_SPI_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_SPI_MANUAL_ENCRYPT` writer - Set this bit as 1 to enable mspi xts manual encrypt in spi boot mode."]
pub type ENABLE_SPI_MANUAL_ENCRYPT_W<'a, const O: u8> =
    crate::BitWriter<'a, EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC, O>;
#[doc = "Field `ENABLE_DOWNLOAD_DB_ENCRYPT` reader - reserved"]
pub type ENABLE_DOWNLOAD_DB_ENCRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` reader - Set this bit as 1 to enable mspi xts auto decrypt in download boot mode."]
pub type ENABLE_DOWNLOAD_G0CB_DECRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` writer - Set this bit as 1 to enable mspi xts auto decrypt in download boot mode."]
pub type ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a, const O: u8> =
    crate::BitWriter<'a, EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC, O>;
#[doc = "Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` reader - Set this bit as 1 to enable mspi xts manual encrypt in download boot mode."]
pub type ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` writer - Set this bit as 1 to enable mspi xts manual encrypt in download boot mode."]
pub type ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a, const O: u8> =
    crate::BitWriter<'a, EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Set this bit as 1 to enable mspi xts manual encrypt in spi boot mode."]
    #[inline(always)]
    pub fn enable_spi_manual_encrypt(&self) -> ENABLE_SPI_MANUAL_ENCRYPT_R {
        ENABLE_SPI_MANUAL_ENCRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reserved"]
    #[inline(always)]
    pub fn enable_download_db_encrypt(&self) -> ENABLE_DOWNLOAD_DB_ENCRYPT_R {
        ENABLE_DOWNLOAD_DB_ENCRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit as 1 to enable mspi xts auto decrypt in download boot mode."]
    #[inline(always)]
    pub fn enable_download_g0cb_decrypt(&self) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit as 1 to enable mspi xts manual encrypt in download boot mode."]
    #[inline(always)]
    pub fn enable_download_manual_encrypt(&self) -> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL")
            .field(
                "enable_spi_manual_encrypt",
                &format_args!("{}", self.enable_spi_manual_encrypt().bit()),
            )
            .field(
                "enable_download_db_encrypt",
                &format_args!("{}", self.enable_download_db_encrypt().bit()),
            )
            .field(
                "enable_download_g0cb_decrypt",
                &format_args!("{}", self.enable_download_g0cb_decrypt().bit()),
            )
            .field(
                "enable_download_manual_encrypt",
                &format_args!("{}", self.enable_download_manual_encrypt().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit as 1 to enable mspi xts manual encrypt in spi boot mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable_spi_manual_encrypt(&mut self) -> ENABLE_SPI_MANUAL_ENCRYPT_W<0> {
        ENABLE_SPI_MANUAL_ENCRYPT_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit as 1 to enable mspi xts auto decrypt in download boot mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable_download_g0cb_decrypt(&mut self) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_W<2> {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit as 1 to enable mspi xts manual encrypt in download boot mode."]
    #[inline(always)]
    #[must_use]
    pub fn enable_download_manual_encrypt(&mut self) -> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<3> {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "EXTERNAL DEVICE ENCRYPTION/DECRYPTION configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [external_device_encrypt_decrypt_control](index.html) module"]
pub struct EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC;
impl crate::RegisterSpec for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [external_device_encrypt_decrypt_control::R](R) reader structure"]
impl crate::Readable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [external_device_encrypt_decrypt_control::W](W) writer structure"]
impl crate::Writable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL to value 0"]
impl crate::Resettable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
