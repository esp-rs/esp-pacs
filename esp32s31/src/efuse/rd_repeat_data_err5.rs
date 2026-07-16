#[doc = "Register `RD_REPEAT_DATA_ERR5` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR5_SPEC>;
#[doc = "Field `DCDC_VSET_EN_ERR` reader - Represents the programming error of EFUSE_DCDC_VSET_EN"]
pub type DCDC_VSET_EN_ERR_R = crate::BitReader;
#[doc = "Field `DIS_WDT_ERR` reader - Represents the programming error of EFUSE_DIS_WDT"]
pub type DIS_WDT_ERR_R = crate::BitReader;
#[doc = "Field `DIS_SWD_ERR` reader - Represents the programming error of EFUSE_DIS_SWD"]
pub type DIS_SWD_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_SHA384_EN_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_SHA384_EN"]
pub type SECURE_BOOT_SHA384_EN_ERR_R = crate::BitReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR` reader - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION"]
pub type BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR_R = crate::FieldReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_EN_ERR` reader - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_EN"]
pub type BOOTLOADER_ANTI_ROLLBACK_EN_ERR_R = crate::BitReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR` reader - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM"]
pub type BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR_R = crate::BitReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR` reader - Represents the programming error of EFUSE_RECOVERY_BOOTLOADER_FLASH_SECTOR"]
pub type RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR_R = crate::FieldReader<u16>;
#[doc = "Field `RMA_ENA_ERR` reader - Represents the programming error of EFUSE_RMA_ENA"]
pub type RMA_ENA_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 2 - Represents the programming error of EFUSE_DCDC_VSET_EN"]
    #[inline(always)]
    pub fn dcdc_vset_en_err(&self) -> DCDC_VSET_EN_ERR_R {
        DCDC_VSET_EN_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents the programming error of EFUSE_DIS_WDT"]
    #[inline(always)]
    pub fn dis_wdt_err(&self) -> DIS_WDT_ERR_R {
        DIS_WDT_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the programming error of EFUSE_DIS_SWD"]
    #[inline(always)]
    pub fn dis_swd_err(&self) -> DIS_SWD_ERR_R {
        DIS_SWD_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 11 - Represents the programming error of EFUSE_SECURE_BOOT_SHA384_EN"]
    #[inline(always)]
    pub fn secure_boot_sha384_en_err(&self) -> SECURE_BOOT_SHA384_EN_ERR_R {
        SECURE_BOOT_SHA384_EN_ERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_secure_version_err(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR_R {
        BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_ERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_EN"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_en_err(&self) -> BOOTLOADER_ANTI_ROLLBACK_EN_ERR_R {
        BOOTLOADER_ANTI_ROLLBACK_EN_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents the programming error of EFUSE_BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_update_in_rom_err(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR_R {
        BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_ERR_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:29 - Represents the programming error of EFUSE_RECOVERY_BOOTLOADER_FLASH_SECTOR"]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_err(&self) -> RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR_R {
        RECOVERY_BOOTLOADER_FLASH_SECTOR_ERR_R::new(((self.bits >> 18) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - Represents the programming error of EFUSE_RMA_ENA"]
    #[inline(always)]
    pub fn rma_ena_err(&self) -> RMA_ENA_ERR_R {
        RMA_ENA_ERR_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR5")
            .field("dcdc_vset_en_err", &self.dcdc_vset_en_err())
            .field("dis_wdt_err", &self.dis_wdt_err())
            .field("dis_swd_err", &self.dis_swd_err())
            .field(
                "secure_boot_sha384_en_err",
                &self.secure_boot_sha384_en_err(),
            )
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
            .field("rma_ena_err", &self.rma_ena_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR5_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err5::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR5_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR5 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR5_SPEC {}
