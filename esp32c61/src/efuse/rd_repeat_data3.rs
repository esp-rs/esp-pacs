#[doc = "Register `RD_REPEAT_DATA3` reader"]
pub type R = crate::R<RD_REPEAT_DATA3_SPEC>;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION` reader - Represents the anti-rollback secure version of the 2nd stage bootloader used by the ROM bootloader."]
pub type BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_R = crate::FieldReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_EN` reader - Represents whether the ani-rollback check for the 2nd stage bootloader is enabled.\\\\1: Enabled\\\\0: Disabled\\\\"]
pub type BOOTLOADER_ANTI_ROLLBACK_EN_R = crate::BitReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM` reader - Represents whether the ani-rollback SECURE_VERSION will be updated from the ROM bootloader.\\\\1: Enable\\\\0: Disable\\\\"]
pub type BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_R = crate::BitReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RECOVERY_BOOTLOADER_FLASH_SECTOR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Represents the anti-rollback secure version of the 2nd stage bootloader used by the ROM bootloader."]
    #[inline(always)]
    pub fn bootloader_anti_rollback_secure_version(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_R {
        BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Represents whether the ani-rollback check for the 2nd stage bootloader is enabled.\\\\1: Enabled\\\\0: Disabled\\\\"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_en(&self) -> BOOTLOADER_ANTI_ROLLBACK_EN_R {
        BOOTLOADER_ANTI_ROLLBACK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents whether the ani-rollback SECURE_VERSION will be updated from the ROM bootloader.\\\\1: Enable\\\\0: Disable\\\\"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_update_in_rom(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_R {
        BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:17 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector(&self) -> RECOVERY_BOOTLOADER_FLASH_SECTOR_R {
        RECOVERY_BOOTLOADER_FLASH_SECTOR_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA3")
            .field(
                "bootloader_anti_rollback_secure_version",
                &self.bootloader_anti_rollback_secure_version(),
            )
            .field(
                "bootloader_anti_rollback_en",
                &self.bootloader_anti_rollback_en(),
            )
            .field(
                "bootloader_anti_rollback_update_in_rom",
                &self.bootloader_anti_rollback_update_in_rom(),
            )
            .field(
                "recovery_bootloader_flash_sector",
                &self.recovery_bootloader_flash_sector(),
            )
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data3::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA3_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA3 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA3_SPEC {}
