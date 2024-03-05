#[doc = "Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` reader"]
pub type R = crate::R<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
#[doc = "Register `EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL` writer"]
pub type W = crate::W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC>;
#[doc = "Field `ENABLE_SPI_MANUAL_ENCRYPT` reader - reg_enable_spi_manual_encrypt"]
pub type ENABLE_SPI_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_SPI_MANUAL_ENCRYPT` writer - reg_enable_spi_manual_encrypt"]
pub type ENABLE_SPI_MANUAL_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DOWNLOAD_DB_ENCRYPT` reader - reg_enable_download_db_encrypt"]
pub type ENABLE_DOWNLOAD_DB_ENCRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_DOWNLOAD_DB_ENCRYPT` writer - reg_enable_download_db_encrypt"]
pub type ENABLE_DOWNLOAD_DB_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` reader - reg_enable_download_g0cb_decrypt"]
pub type ENABLE_DOWNLOAD_G0CB_DECRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_DOWNLOAD_G0CB_DECRYPT` writer - reg_enable_download_g0cb_decrypt"]
pub type ENABLE_DOWNLOAD_G0CB_DECRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` reader - reg_enable_download_manual_encrypt"]
pub type ENABLE_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `ENABLE_DOWNLOAD_MANUAL_ENCRYPT` writer - reg_enable_download_manual_encrypt"]
pub type ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - reg_enable_spi_manual_encrypt"]
    #[inline(always)]
    pub fn enable_spi_manual_encrypt(&self) -> ENABLE_SPI_MANUAL_ENCRYPT_R {
        ENABLE_SPI_MANUAL_ENCRYPT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_enable_download_db_encrypt"]
    #[inline(always)]
    pub fn enable_download_db_encrypt(&self) -> ENABLE_DOWNLOAD_DB_ENCRYPT_R {
        ENABLE_DOWNLOAD_DB_ENCRYPT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - reg_enable_download_g0cb_decrypt"]
    #[inline(always)]
    pub fn enable_download_g0cb_decrypt(&self) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_R {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - reg_enable_download_manual_encrypt"]
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_enable_spi_manual_encrypt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_spi_manual_encrypt(
        &mut self,
    ) -> ENABLE_SPI_MANUAL_ENCRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_SPI_MANUAL_ENCRYPT_W::new(self, 0)
    }
    #[doc = "Bit 1 - reg_enable_download_db_encrypt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_download_db_encrypt(
        &mut self,
    ) -> ENABLE_DOWNLOAD_DB_ENCRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_DOWNLOAD_DB_ENCRYPT_W::new(self, 1)
    }
    #[doc = "Bit 2 - reg_enable_download_g0cb_decrypt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_download_g0cb_decrypt(
        &mut self,
    ) -> ENABLE_DOWNLOAD_G0CB_DECRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_DOWNLOAD_G0CB_DECRYPT_W::new(self, 2)
    }
    #[doc = "Bit 3 - reg_enable_download_manual_encrypt"]
    #[inline(always)]
    #[must_use]
    pub fn enable_download_manual_encrypt(
        &mut self,
    ) -> ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W<EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC> {
        ENABLE_DOWNLOAD_MANUAL_ENCRYPT_W::new(self, 3)
    }
}
#[doc = "SYSTEM_EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`external_device_encrypt_decrypt_control::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`external_device_encrypt_decrypt_control::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC;
impl crate::RegisterSpec for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`external_device_encrypt_decrypt_control::R`](R) reader structure"]
impl crate::Readable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`external_device_encrypt_decrypt_control::W`](W) writer structure"]
impl crate::Writable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL to value 0"]
impl crate::Resettable for EXTERNAL_DEVICE_ENCRYPT_DECRYPT_CONTROL_SPEC {
    const RESET_VALUE: u32 = 0;
}
