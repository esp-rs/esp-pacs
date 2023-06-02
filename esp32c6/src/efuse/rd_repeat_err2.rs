#[doc = "Register `RD_REPEAT_ERR2` reader"]
pub struct R(crate::R<RD_REPEAT_ERR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `KEY_PURPOSE_2_ERR` reader - Indicates a programming error of KEY_PURPOSE_2."]
pub type KEY_PURPOSE_2_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3_ERR` reader - Indicates a programming error of KEY_PURPOSE_3."]
pub type KEY_PURPOSE_3_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4_ERR` reader - Indicates a programming error of KEY_PURPOSE_4."]
pub type KEY_PURPOSE_4_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5_ERR` reader - Indicates a programming error of KEY_PURPOSE_5."]
pub type KEY_PURPOSE_5_ERR_R = crate::FieldReader;
#[doc = "Field `SEC_DPA_LEVEL_ERR` reader - Indicates a programming error of SEC_DPA_LEVEL."]
pub type SEC_DPA_LEVEL_ERR_R = crate::FieldReader;
#[doc = "Field `RPT4_RESERVED2_ERR_1` reader - Reserved."]
pub type RPT4_RESERVED2_ERR_1_R = crate::BitReader;
#[doc = "Field `CRYPT_DPA_ENABLE_ERR` reader - Indicates a programming error of CRYPT_DPA_ENABLE."]
pub type CRYPT_DPA_ENABLE_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - Indicates a programming error of SECURE_BOOT_EN."]
pub type SECURE_BOOT_EN_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - Indicates a programming error of SECURE_BOOT_AGGRESSIVE_REVOKE."]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED2_ERR_0` reader - Reserved."]
pub type RPT4_RESERVED2_ERR_0_R = crate::FieldReader;
#[doc = "Field `FLASH_TPUW_ERR` reader - Indicates a programming error of FLASH_TPUW."]
pub type FLASH_TPUW_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Indicates a programming error of KEY_PURPOSE_2."]
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> KEY_PURPOSE_2_ERR_R {
        KEY_PURPOSE_2_ERR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Indicates a programming error of KEY_PURPOSE_3."]
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> KEY_PURPOSE_3_ERR_R {
        KEY_PURPOSE_3_ERR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Indicates a programming error of KEY_PURPOSE_4."]
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> KEY_PURPOSE_4_ERR_R {
        KEY_PURPOSE_4_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Indicates a programming error of KEY_PURPOSE_5."]
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> KEY_PURPOSE_5_ERR_R {
        KEY_PURPOSE_5_ERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:17 - Indicates a programming error of SEC_DPA_LEVEL."]
    #[inline(always)]
    pub fn sec_dpa_level_err(&self) -> SEC_DPA_LEVEL_ERR_R {
        SEC_DPA_LEVEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved2_err_1(&self) -> RPT4_RESERVED2_ERR_1_R {
        RPT4_RESERVED2_ERR_1_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Indicates a programming error of CRYPT_DPA_ENABLE."]
    #[inline(always)]
    pub fn crypt_dpa_enable_err(&self) -> CRYPT_DPA_ENABLE_ERR_R {
        CRYPT_DPA_ENABLE_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Indicates a programming error of SECURE_BOOT_EN."]
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SECURE_BOOT_EN_ERR_R {
        SECURE_BOOT_EN_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Indicates a programming error of SECURE_BOOT_AGGRESSIVE_REVOKE."]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:27 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved2_err_0(&self) -> RPT4_RESERVED2_ERR_0_R {
        RPT4_RESERVED2_ERR_0_R::new(((self.bits >> 22) & 0x3f) as u8)
    }
    #[doc = "Bits 28:31 - Indicates a programming error of FLASH_TPUW."]
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FLASH_TPUW_ERR_R {
        FLASH_TPUW_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR2")
            .field(
                "key_purpose_2_err",
                &format_args!("{}", self.key_purpose_2_err().bits()),
            )
            .field(
                "key_purpose_3_err",
                &format_args!("{}", self.key_purpose_3_err().bits()),
            )
            .field(
                "key_purpose_4_err",
                &format_args!("{}", self.key_purpose_4_err().bits()),
            )
            .field(
                "key_purpose_5_err",
                &format_args!("{}", self.key_purpose_5_err().bits()),
            )
            .field(
                "sec_dpa_level_err",
                &format_args!("{}", self.sec_dpa_level_err().bits()),
            )
            .field(
                "rpt4_reserved2_err_1",
                &format_args!("{}", self.rpt4_reserved2_err_1().bit()),
            )
            .field(
                "crypt_dpa_enable_err",
                &format_args!("{}", self.crypt_dpa_enable_err().bit()),
            )
            .field(
                "secure_boot_en_err",
                &format_args!("{}", self.secure_boot_en_err().bit()),
            )
            .field(
                "secure_boot_aggressive_revoke_err",
                &format_args!("{}", self.secure_boot_aggressive_revoke_err().bit()),
            )
            .field(
                "rpt4_reserved2_err_0",
                &format_args!("{}", self.rpt4_reserved2_err_0().bits()),
            )
            .field(
                "flash_tpuw_err",
                &format_args!("{}", self.flash_tpuw_err().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_ERR2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 2 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err2](index.html) module"]
pub struct RD_REPEAT_ERR2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err2::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR2 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
