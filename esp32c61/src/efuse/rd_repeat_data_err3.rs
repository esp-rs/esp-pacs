#[doc = "Register `RD_REPEAT_DATA_ERR3` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR3_SPEC>;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR` reader - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION"]
pub type BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR_R = crate::FieldReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_EN_ERR` reader - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_EN"]
pub type BOOTLOADER_ANTI_ROLLBACK_EN_ERR_R = crate::BitReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR` reader - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM"]
pub type BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR_R = crate::BitReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR` reader - Represents the programming error of EFUSE_RECOVERY_BOOTLOADER_FLASH_SECTOR"]
pub type RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:3 - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_secure_version_err(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR_R {
        BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bit 4 - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_EN"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_en_err(&self) -> BOOTLOADER_ANTI_ROLLBACK_EN_ERR_R {
        BOOTLOADER_ANTI_ROLLBACK_EN_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_update_in_rom_err(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR_R {
        BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:17 - Represents the programming error of EFUSE_RECOVERY_BOOTLOADER_FLASH_SECTOR"]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_err(&self) -> RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR_R {
        RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR_R::new(((self.bits >> 6) & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR3")
            .field(
                "bootloader_anti_rollback_secure_version_err",
                &self.bootloader_anti_rollback_secure_version_err(),
            )
            .field(
                "bootloader_anti_rollback_en_err",
                &self.bootloader_anti_rollback_en_err(),
            )
            .field(
                "bootloader_anti_rollback_update_in_rom_err",
                &self.bootloader_anti_rollback_update_in_rom_err(),
            )
            .field(
                "recovery_bootloader_flash_sector_err",
                &self.recovery_bootloader_flash_sector_err(),
            )
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err3::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR3_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR3 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR3_SPEC {}
