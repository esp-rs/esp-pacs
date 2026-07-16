#[doc = "Register `RD_REPEAT_DATA5` reader"]
pub type R = crate::R<RD_REPEAT_DATA5_SPEC>;
#[doc = "Register `RD_REPEAT_DATA5` writer"]
pub type W = crate::W<RD_REPEAT_DATA5_SPEC>;
#[doc = "Field `RD_RESERVE_0_192` reader - "]
pub type RD_RESERVE_0_192_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_192` writer - "]
pub type RD_RESERVE_0_192_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DCDC_VSET_EN` reader - Select dcdc vset use efuse_dcdc_vset."]
pub type DCDC_VSET_EN_R = crate::BitReader;
#[doc = "Field `DIS_WDT` reader - Set this bit to disable watch dog."]
pub type DIS_WDT_R = crate::BitReader;
#[doc = "Field `DIS_SWD` reader - Set this bit to disable super-watchdog."]
pub type DIS_SWD_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_197` reader - "]
pub type RD_RESERVE_0_197_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_197` writer - "]
pub type RD_RESERVE_0_197_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `SECURE_BOOT_SHA384_EN` reader - Represents whether secure boot using SHA-384 is enabled. 0: Disable 1: Enable"]
pub type SECURE_BOOT_SHA384_EN_R = crate::BitReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION` reader - Represents the anti-rollback secure version of the 2nd stage bootloader used by the ROM bootloader."]
pub type BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_R = crate::FieldReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_EN` reader - Represents whether the ani-rollback check for the 2nd stage bootloader is enabled.\\\\1: Enabled\\\\0: Disabled\\\\"]
pub type BOOTLOADER_ANTI_ROLLBACK_EN_R = crate::BitReader;
#[doc = "Field `BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM` reader - Represents whether the ani-rollback SECURE_VERSION will be updated from the ROM bootloader.\\\\1: Enable\\\\0: Disable\\\\"]
pub type BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_R = crate::BitReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RECOVERY_BOOTLOADER_FLASH_SECTOR_R = crate::FieldReader<u16>;
#[doc = "Field `RMA_ENA` reader - Represents whether rma function is supported in download mode.\\\\ 2'b01/2'b10: enabled\\\\2'b00/2'b11: disabled\\\\"]
pub type RMA_ENA_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rd_reserve_0_192(&self) -> RD_RESERVE_0_192_R {
        RD_RESERVE_0_192_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Select dcdc vset use efuse_dcdc_vset."]
    #[inline(always)]
    pub fn dcdc_vset_en(&self) -> DCDC_VSET_EN_R {
        DCDC_VSET_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to disable watch dog."]
    #[inline(always)]
    pub fn dis_wdt(&self) -> DIS_WDT_R {
        DIS_WDT_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to disable super-watchdog."]
    #[inline(always)]
    pub fn dis_swd(&self) -> DIS_SWD_R {
        DIS_SWD_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:10"]
    #[inline(always)]
    pub fn rd_reserve_0_197(&self) -> RD_RESERVE_0_197_R {
        RD_RESERVE_0_197_R::new(((self.bits >> 5) & 0x3f) as u8)
    }
    #[doc = "Bit 11 - Represents whether secure boot using SHA-384 is enabled. 0: Disable 1: Enable"]
    #[inline(always)]
    pub fn secure_boot_sha384_en(&self) -> SECURE_BOOT_SHA384_EN_R {
        SECURE_BOOT_SHA384_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Represents the anti-rollback secure version of the 2nd stage bootloader used by the ROM bootloader."]
    #[inline(always)]
    pub fn bootloader_anti_rollback_secure_version(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_R {
        BOOTLOADER_ANTI_ROLLBACK_SECURE_VERSION_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Represents whether the ani-rollback check for the 2nd stage bootloader is enabled.\\\\1: Enabled\\\\0: Disabled\\\\"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_en(&self) -> BOOTLOADER_ANTI_ROLLBACK_EN_R {
        BOOTLOADER_ANTI_ROLLBACK_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents whether the ani-rollback SECURE_VERSION will be updated from the ROM bootloader.\\\\1: Enable\\\\0: Disable\\\\"]
    #[inline(always)]
    pub fn bootloader_anti_rollback_update_in_rom(
        &self,
    ) -> BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_R {
        BOOTLOADER_ANTI_ROLLBACK_UPDATE_IN_ROM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:29 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector(&self) -> RECOVERY_BOOTLOADER_FLASH_SECTOR_R {
        RECOVERY_BOOTLOADER_FLASH_SECTOR_R::new(((self.bits >> 18) & 0x0fff) as u16)
    }
    #[doc = "Bits 30:31 - Represents whether rma function is supported in download mode.\\\\ 2'b01/2'b10: enabled\\\\2'b00/2'b11: disabled\\\\"]
    #[inline(always)]
    pub fn rma_ena(&self) -> RMA_ENA_R {
        RMA_ENA_R::new(((self.bits >> 30) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA5")
            .field("dcdc_vset_en", &self.dcdc_vset_en())
            .field("dis_wdt", &self.dis_wdt())
            .field("dis_swd", &self.dis_swd())
            .field("secure_boot_sha384_en", &self.secure_boot_sha384_en())
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
            .field("rma_ena", &self.rma_ena())
            .field("rd_reserve_0_192", &self.rd_reserve_0_192())
            .field("rd_reserve_0_197", &self.rd_reserve_0_197())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn rd_reserve_0_192(&mut self) -> RD_RESERVE_0_192_W<'_, RD_REPEAT_DATA5_SPEC> {
        RD_RESERVE_0_192_W::new(self, 0)
    }
    #[doc = "Bits 5:10"]
    #[inline(always)]
    pub fn rd_reserve_0_197(&mut self) -> RD_RESERVE_0_197_W<'_, RD_REPEAT_DATA5_SPEC> {
        RD_RESERVE_0_197_W::new(self, 5)
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_repeat_data5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA5_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data5::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA5_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_repeat_data5::W`](W) writer structure"]
impl crate::Writable for RD_REPEAT_DATA5_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA5 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA5_SPEC {}
