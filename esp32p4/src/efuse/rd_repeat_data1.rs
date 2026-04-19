#[doc = "Register `RD_REPEAT_DATA1` reader"]
pub type R = crate::R<RD_REPEAT_DATA1_SPEC>;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_8_10` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RECOVERY_BOOTLOADER_FLASH_SECTOR_8_10_R = crate::FieldReader;
#[doc = "Field `RECOVERY_BOOTLOADER_FLASH_SECTOR_11_11` reader - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
pub type RECOVERY_BOOTLOADER_FLASH_SECTOR_11_11_R = crate::BitReader;
#[doc = "Field `KM_RND_SWITCH_CYCLE` reader - Set the bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles"]
pub type KM_RND_SWITCH_CYCLE_R = crate::BitReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE` reader - EFUSE_KM_DEPLOY_ONLY_ONCE and EFUSE_KM_DEPLOY_ONLY_ONCE_H together form one field: {EFUSE_KM_DEPLOY_ONLY_ONCE_H, EFUSE_KM_DEPLOY_ONLY_ONCE\\[3:0\\]}. Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type KM_DEPLOY_ONLY_ONCE_R = crate::FieldReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY` reader - EFUSE_FORCE_USE_KEY_MANAGER_KEY and EFUSE_FORCE_USE_KEY_MANAGER_KEY_H together form one field: {EFUSE_FORCE_USE_KEY_MANAGER_KEY_H, EFUSE_FORCE_USE_KEY_MANAGER_KEY\\[3:0\\]}. Set each bit to control whether corresponding key must come from key manager. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
pub type FORCE_USE_KEY_MANAGER_KEY_R = crate::FieldReader;
#[doc = "Field `FORCE_DISABLE_SW_INIT_KEY` reader - Set this bit to disable software written init key, and force use efuse_init_key."]
pub type FORCE_DISABLE_SW_INIT_KEY_R = crate::BitReader;
#[doc = "Field `KM_XTS_KEY_LENGTH_256` reader - Set this bit to config flash encryption xts-512 key, else use xts-256 key when using the key manager"]
pub type KM_XTS_KEY_LENGTH_256_R = crate::BitReader;
#[doc = "Field `ECC_FORCE_CONST_TIME` reader - Set this bit to permanently turn on ECC const-time mode."]
pub type ECC_FORCE_CONST_TIME_R = crate::BitReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Select lp wdt timeout threshold at startup = initial timeout value * (2 ^ (EFUSE_WDT_DELAY_SEL + 1))"]
pub type WDT_DELAY_SEL_R = crate::BitReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - Set this bit to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
pub type SPI_BOOT_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - Set this bit to enable revoking first secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - Set this bit to enable revoking second secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - Set this bit to enable revoking third secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0` reader - Purpose of Key0."]
pub type KEY_PURPOSE_0_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1` reader - Purpose of Key1."]
pub type KEY_PURPOSE_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_8_10(&self) -> RECOVERY_BOOTLOADER_FLASH_SECTOR_8_10_R {
        RECOVERY_BOOTLOADER_FLASH_SECTOR_8_10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - Represents the starting flash sector (flash sector size is 0x1000) of the recovery bootloader used by the ROM bootloader If the primary bootloader fails. 0 and 0xFFF - this feature is disabled."]
    #[inline(always)]
    pub fn recovery_bootloader_flash_sector_11_11(
        &self,
    ) -> RECOVERY_BOOTLOADER_FLASH_SECTOR_11_11_R {
        RECOVERY_BOOTLOADER_FLASH_SECTOR_11_11_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set the bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles"]
    #[inline(always)]
    pub fn km_rnd_switch_cycle(&self) -> KM_RND_SWITCH_CYCLE_R {
        KM_RND_SWITCH_CYCLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:8 - EFUSE_KM_DEPLOY_ONLY_ONCE and EFUSE_KM_DEPLOY_ONLY_ONCE_H together form one field: {EFUSE_KM_DEPLOY_ONLY_ONCE_H, EFUSE_KM_DEPLOY_ONLY_ONCE\\[3:0\\]}. Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn km_deploy_only_once(&self) -> KM_DEPLOY_ONLY_ONCE_R {
        KM_DEPLOY_ONLY_ONCE_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - EFUSE_FORCE_USE_KEY_MANAGER_KEY and EFUSE_FORCE_USE_KEY_MANAGER_KEY_H together form one field: {EFUSE_FORCE_USE_KEY_MANAGER_KEY_H, EFUSE_FORCE_USE_KEY_MANAGER_KEY\\[3:0\\]}. Set each bit to control whether corresponding key must come from key manager. 1 is true, 0 is false. bit 0: ecsda, bit 1: xts, bit2: hmac, bit3: ds, bit4:psram"]
    #[inline(always)]
    pub fn force_use_key_manager_key(&self) -> FORCE_USE_KEY_MANAGER_KEY_R {
        FORCE_USE_KEY_MANAGER_KEY_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Set this bit to disable software written init key, and force use efuse_init_key."]
    #[inline(always)]
    pub fn force_disable_sw_init_key(&self) -> FORCE_DISABLE_SW_INIT_KEY_R {
        FORCE_DISABLE_SW_INIT_KEY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to config flash encryption xts-512 key, else use xts-256 key when using the key manager"]
    #[inline(always)]
    pub fn km_xts_key_length_256(&self) -> KM_XTS_KEY_LENGTH_256_R {
        KM_XTS_KEY_LENGTH_256_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to permanently turn on ECC const-time mode."]
    #[inline(always)]
    pub fn ecc_force_const_time(&self) -> ECC_FORCE_CONST_TIME_R {
        ECC_FORCE_CONST_TIME_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Select lp wdt timeout threshold at startup = initial timeout value * (2 ^ (EFUSE_WDT_DELAY_SEL + 1))"]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:20 - Set this bit to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Set this bit to enable revoking first secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable revoking second secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable revoking third secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Purpose of Key0."]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Purpose of Key1."]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA1")
            .field(
                "recovery_bootloader_flash_sector_8_10",
                &self.recovery_bootloader_flash_sector_8_10(),
            )
            .field(
                "recovery_bootloader_flash_sector_11_11",
                &self.recovery_bootloader_flash_sector_11_11(),
            )
            .field("km_rnd_switch_cycle", &self.km_rnd_switch_cycle())
            .field("km_deploy_only_once", &self.km_deploy_only_once())
            .field(
                "force_use_key_manager_key",
                &self.force_use_key_manager_key(),
            )
            .field(
                "force_disable_sw_init_key",
                &self.force_disable_sw_init_key(),
            )
            .field("km_xts_key_length_256", &self.km_xts_key_length_256())
            .field("ecc_force_const_time", &self.ecc_force_const_time())
            .field("wdt_delay_sel", &self.wdt_delay_sel())
            .field("spi_boot_crypt_cnt", &self.spi_boot_crypt_cnt())
            .field("secure_boot_key_revoke0", &self.secure_boot_key_revoke0())
            .field("secure_boot_key_revoke1", &self.secure_boot_key_revoke1())
            .field("secure_boot_key_revoke2", &self.secure_boot_key_revoke2())
            .field("key_purpose_0", &self.key_purpose_0())
            .field("key_purpose_1", &self.key_purpose_1())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data1::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA1_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA1_SPEC {}
