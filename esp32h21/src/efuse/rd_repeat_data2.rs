#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub type R = crate::R<RD_REPEAT_DATA2_SPEC>;
#[doc = "Field `KEY_PURPOSE_2` reader - Represents the purpose of Key2. See Table tab:efuse-key-purpose.\\\\"]
pub type KEY_PURPOSE_2_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3` reader - Represents the purpose of Key3. See Table tab:efuse-key-purpose.\\\\"]
pub type KEY_PURPOSE_3_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4` reader - Represents the purpose of Key4. See Table tab:efuse-key-purpose.\\\\"]
pub type KEY_PURPOSE_4_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5` reader - Represents the purpose of Key5. See Table tab:efuse-key-purpose.\\\\"]
pub type KEY_PURPOSE_5_R = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL` reader - Represents the security level of anti-DPA attack. The level is adjusted by configuring the clock random frequency division mode.\\\\ 0: Security level is SEC_DPA_OFF\\\\ 1: Security level is SEC_DPA_LOW\\\\ 2: Security level is SEC_DPA_MIDDLE\\\\ 3: Security level is SEC_DPA_HIGH\\\\ For more information, please refer to Chapter mod:sysreg > Section sec:sysreg-anti-dpa-attack-security-control.\\\\"]
pub type SEC_DPA_LEVEL_R = crate::FieldReader;
#[doc = "Field `IO_LDO_1P8` reader - Represents select IO LDO voltage to 1.8V or 3.3V.\\\\ 1: 1.8V\\\\ 0: 3.3V\\\\"]
pub type IO_LDO_1P8_R = crate::BitReader;
#[doc = "Field `CRYPT_DPA_ENABLE` reader - Represents whether defense against DPA attack is enabled.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type CRYPT_DPA_ENABLE_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Represents whether Secure Boot is enabled.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Represents whether aggressive revocation of Secure Boot is enabled.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
#[doc = "Field `POWERGLITCH_EN1` reader - Represents whether to enable power glitch function when chip power on.\\\\"]
pub type POWERGLITCH_EN1_R = crate::FieldReader;
#[doc = "Field `DCDC_CCM_EN` reader - Represents whether change DCDC to CCM mode."]
pub type DCDC_CCM_EN_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW` reader - Represents the flash waiting time after power-up. Measurement unit: ms.\\\\ When the value is less than 15, the waiting time is the programmed value. Otherwise, the waiting time is a fixed value, i.e. 30 ms.\\\\"]
pub type FLASH_TPUW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents the purpose of Key2. See Table tab:efuse-key-purpose.\\\\"]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Represents the purpose of Key3. See Table tab:efuse-key-purpose.\\\\"]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Represents the purpose of Key4. See Table tab:efuse-key-purpose.\\\\"]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Represents the purpose of Key5. See Table tab:efuse-key-purpose.\\\\"]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents the security level of anti-DPA attack. The level is adjusted by configuring the clock random frequency division mode.\\\\ 0: Security level is SEC_DPA_OFF\\\\ 1: Security level is SEC_DPA_LOW\\\\ 2: Security level is SEC_DPA_MIDDLE\\\\ 3: Security level is SEC_DPA_HIGH\\\\ For more information, please refer to Chapter mod:sysreg > Section sec:sysreg-anti-dpa-attack-security-control.\\\\"]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SEC_DPA_LEVEL_R {
        SEC_DPA_LEVEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Represents select IO LDO voltage to 1.8V or 3.3V.\\\\ 1: 1.8V\\\\ 0: 3.3V\\\\"]
    #[inline(always)]
    pub fn io_ldo_1p8(&self) -> IO_LDO_1P8_R {
        IO_LDO_1P8_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents whether defense against DPA attack is enabled.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn crypt_dpa_enable(&self) -> CRYPT_DPA_ENABLE_R {
        CRYPT_DPA_ENABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether Secure Boot is enabled.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents whether aggressive revocation of Secure Boot is enabled.\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:26 - Represents whether to enable power glitch function when chip power on.\\\\"]
    #[inline(always)]
    pub fn powerglitch_en1(&self) -> POWERGLITCH_EN1_R {
        POWERGLITCH_EN1_R::new(((self.bits >> 22) & 0x1f) as u8)
    }
    #[doc = "Bit 27 - Represents whether change DCDC to CCM mode."]
    #[inline(always)]
    pub fn dcdc_ccm_en(&self) -> DCDC_CCM_EN_R {
        DCDC_CCM_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Represents the flash waiting time after power-up. Measurement unit: ms.\\\\ When the value is less than 15, the waiting time is the programmed value. Otherwise, the waiting time is a fixed value, i.e. 30 ms.\\\\"]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA2")
            .field("key_purpose_2", &self.key_purpose_2())
            .field("key_purpose_3", &self.key_purpose_3())
            .field("key_purpose_4", &self.key_purpose_4())
            .field("key_purpose_5", &self.key_purpose_5())
            .field("sec_dpa_level", &self.sec_dpa_level())
            .field("io_ldo_1p8", &self.io_ldo_1p8())
            .field("crypt_dpa_enable", &self.crypt_dpa_enable())
            .field("secure_boot_en", &self.secure_boot_en())
            .field(
                "secure_boot_aggressive_revoke",
                &self.secure_boot_aggressive_revoke(),
            )
            .field("powerglitch_en1", &self.powerglitch_en1())
            .field("dcdc_ccm_en", &self.dcdc_ccm_en())
            .field("flash_tpuw", &self.flash_tpuw())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {}
