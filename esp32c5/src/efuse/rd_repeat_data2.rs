#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub type R = crate::R<RD_REPEAT_DATA2_SPEC>;
#[doc = "Field `KEY_PURPOSE_2` reader - Represents the purpose of Key2."]
pub type KEY_PURPOSE_2_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3` reader - Represents the purpose of Key3."]
pub type KEY_PURPOSE_3_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4` reader - Represents the purpose of Key4."]
pub type KEY_PURPOSE_4_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5` reader - Represents the purpose of Key5."]
pub type KEY_PURPOSE_5_R = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL` reader - Represents the spa secure level by configuring the clock random divide mode."]
pub type SEC_DPA_LEVEL_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Represents whether secure boot is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Represents whether revoking aggressive secure boot is enabled or disabled.\\\\ 1: enabled.\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
#[doc = "Field `KM_XTS_KEY_LENGTH_256` reader - Set this bitto configure flash encryption use xts-128 key. else use xts-256 key."]
pub type KM_XTS_KEY_LENGTH_256_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW` reader - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
pub type FLASH_TPUW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents the purpose of Key2."]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Represents the purpose of Key3."]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Represents the purpose of Key4."]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Represents the purpose of Key5."]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Represents the spa secure level by configuring the clock random divide mode."]
    #[inline(always)]
    pub fn sec_dpa_level(&self) -> SEC_DPA_LEVEL_R {
        SEC_DPA_LEVEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 20 - Represents whether secure boot is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents whether revoking aggressive secure boot is enabled or disabled.\\\\ 1: enabled.\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 27 - Set this bitto configure flash encryption use xts-128 key. else use xts-256 key."]
    #[inline(always)]
    pub fn km_xts_key_length_256(&self) -> KM_XTS_KEY_LENGTH_256_R {
        KM_XTS_KEY_LENGTH_256_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
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
            .field("secure_boot_en", &self.secure_boot_en())
            .field(
                "secure_boot_aggressive_revoke",
                &self.secure_boot_aggressive_revoke(),
            )
            .field("km_xts_key_length_256", &self.km_xts_key_length_256())
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
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {
    const RESET_VALUE: u32 = 0;
}
