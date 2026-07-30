#[doc = "Register `RD_REPEAT_DATA1` reader"]
pub type R = crate::R<RD_REPEAT_DATA1_SPEC>;
#[doc = "Field `IO_LDO_ADJUST` reader - Represents configuration of IO LDO mode and voltage.\\\\"]
pub type IO_LDO_ADJUST_R = crate::FieldReader;
#[doc = "Field `VDD_SPI_LDO_ADJUST` reader - Represents configuration of FLASH LDO mode and voltage.\\\\"]
pub type VDD_SPI_LDO_ADJUST_R = crate::FieldReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Represents RTC watchdog timeout threshold.\\\\ 0：The originally configured STG0 threshold × 2\\\\ 1：The originally configured STG0 threshold × 4\\\\ 2：The originally configured STG0 threshold × 8\\\\ 3：The originally configured STG0 threshold × 16\\\\"]
pub type WDT_DELAY_SEL_R = crate::FieldReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - Represents whether SPI boot encryption/decryption is enabled. \\\\ Odd count of bits with a value of 1: Enabled\\\\ Even count of bits with a value of 1: Disabled\\\\"]
pub type SPI_BOOT_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - Represents whether revoking Secure Boot key 0 is enabled. \\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - Represents whether revoking Secure Boot key 1 is enabled. \\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - Represents whether revoking Secure Boot key 2 is enabled. \\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0` reader - Represents the purpose of Key0. See Table tab:efuse-key-purpose.\\\\"]
pub type KEY_PURPOSE_0_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1` reader - Represents the purpose of Key1. See Table tab:efuse-key-purpose.\\\\"]
pub type KEY_PURPOSE_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Represents configuration of IO LDO mode and voltage.\\\\"]
    #[inline(always)]
    pub fn io_ldo_adjust(&self) -> IO_LDO_ADJUST_R {
        IO_LDO_ADJUST_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Represents configuration of FLASH LDO mode and voltage.\\\\"]
    #[inline(always)]
    pub fn vdd_spi_ldo_adjust(&self) -> VDD_SPI_LDO_ADJUST_R {
        VDD_SPI_LDO_ADJUST_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Represents RTC watchdog timeout threshold.\\\\ 0：The originally configured STG0 threshold × 2\\\\ 1：The originally configured STG0 threshold × 4\\\\ 2：The originally configured STG0 threshold × 8\\\\ 3：The originally configured STG0 threshold × 16\\\\"]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Represents whether SPI boot encryption/decryption is enabled. \\\\ Odd count of bits with a value of 1: Enabled\\\\ Even count of bits with a value of 1: Disabled\\\\"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Represents whether revoking Secure Boot key 0 is enabled. \\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents whether revoking Secure Boot key 1 is enabled. \\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents whether revoking Secure Boot key 2 is enabled. \\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Represents the purpose of Key0. See Table tab:efuse-key-purpose.\\\\"]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Represents the purpose of Key1. See Table tab:efuse-key-purpose.\\\\"]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA1")
            .field("io_ldo_adjust", &self.io_ldo_adjust())
            .field("vdd_spi_ldo_adjust", &self.vdd_spi_ldo_adjust())
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
