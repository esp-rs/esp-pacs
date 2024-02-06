#[doc = "Register `RD_REPEAT_ERR2` reader"]
pub type R = crate::R<RD_REPEAT_ERR2_SPEC>;
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
#[doc = "Field `ECDSA_ENABLE_SOFT_K_ERR` reader - Indicates a programming error of ECDSA_FORCE_USE_HARDWARE_K."]
pub type ECDSA_ENABLE_SOFT_K_ERR_R = crate::BitReader;
#[doc = "Field `CRYPT_DPA_ENABLE_ERR` reader - Indicates a programming error of CRYPT_DPA_ENABLE."]
pub type CRYPT_DPA_ENABLE_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - Indicates a programming error of SECURE_BOOT_EN."]
pub type SECURE_BOOT_EN_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - Indicates a programming error of SECURE_BOOT_AGGRESSIVE_REVOKE."]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R = crate::BitReader;
#[doc = "Field `FLASH_TYPE_ERR` reader - Indicates a programming error of FLASH_TYPE."]
pub type FLASH_TYPE_ERR_R = crate::BitReader;
#[doc = "Field `FLASH_PAGE_SIZE_ERR` reader - Indicates a programming error of FLASH_PAGE_SIZE."]
pub type FLASH_PAGE_SIZE_ERR_R = crate::FieldReader;
#[doc = "Field `FLASH_ECC_EN_ERR` reader - Indicates a programming error of FLASH_ECC_EN."]
pub type FLASH_ECC_EN_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_OTG_DOWNLOAD_MODE_ERR` reader - Indicates a programming error of DIS_USB_OTG_DOWNLOAD_MODE."]
pub type DIS_USB_OTG_DOWNLOAD_MODE_ERR_R = crate::BitReader;
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
    #[doc = "Bit 18 - Indicates a programming error of ECDSA_FORCE_USE_HARDWARE_K."]
    #[inline(always)]
    pub fn ecdsa_enable_soft_k_err(&self) -> ECDSA_ENABLE_SOFT_K_ERR_R {
        ECDSA_ENABLE_SOFT_K_ERR_R::new(((self.bits >> 18) & 1) != 0)
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
    #[doc = "Bit 23 - Indicates a programming error of FLASH_TYPE."]
    #[inline(always)]
    pub fn flash_type_err(&self) -> FLASH_TYPE_ERR_R {
        FLASH_TYPE_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:25 - Indicates a programming error of FLASH_PAGE_SIZE."]
    #[inline(always)]
    pub fn flash_page_size_err(&self) -> FLASH_PAGE_SIZE_ERR_R {
        FLASH_PAGE_SIZE_ERR_R::new(((self.bits >> 24) & 3) as u8)
    }
    #[doc = "Bit 26 - Indicates a programming error of FLASH_ECC_EN."]
    #[inline(always)]
    pub fn flash_ecc_en_err(&self) -> FLASH_ECC_EN_ERR_R {
        FLASH_ECC_EN_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Indicates a programming error of DIS_USB_OTG_DOWNLOAD_MODE."]
    #[inline(always)]
    pub fn dis_usb_otg_download_mode_err(&self) -> DIS_USB_OTG_DOWNLOAD_MODE_ERR_R {
        DIS_USB_OTG_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 27) & 1) != 0)
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
                "ecdsa_enable_soft_k_err",
                &format_args!("{}", self.ecdsa_enable_soft_k_err().bit()),
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
                "flash_type_err",
                &format_args!("{}", self.flash_type_err().bit()),
            )
            .field(
                "flash_page_size_err",
                &format_args!("{}", self.flash_page_size_err().bits()),
            )
            .field(
                "flash_ecc_en_err",
                &format_args!("{}", self.flash_ecc_en_err().bit()),
            )
            .field(
                "dis_usb_otg_download_mode_err",
                &format_args!("{}", self.dis_usb_otg_download_mode_err().bit()),
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
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Programming error record register 2 of BLOCK0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_ERR2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_err2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_ERR2 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR2_SPEC {
    const RESET_VALUE: u32 = 0;
}
