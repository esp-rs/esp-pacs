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
#[doc = "Field `KEY_PURPOSE_2_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type KEY_PURPOSE_2_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_3_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type KEY_PURPOSE_3_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_4_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type KEY_PURPOSE_4_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_5_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type KEY_PURPOSE_5_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RPT4_RESERVED0_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type RPT4_RESERVED0_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SECURE_BOOT_EN_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_USB_JTAG_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_USB_JTAG_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_USB_DEVICE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type DIS_USB_DEVICE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `STRAP_JTAG_SEL_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type STRAP_JTAG_SEL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `USB_PHY_SEL_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type USB_PHY_SEL_ERR_R = crate::BitReader<bool>;
#[doc = "Field `POWER_GLITCH_DSENSE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type POWER_GLITCH_DSENSE_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FLASH_TPUW_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type FLASH_TPUW_ERR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:3 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_2_err(&self) -> KEY_PURPOSE_2_ERR_R {
        KEY_PURPOSE_2_ERR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_3_err(&self) -> KEY_PURPOSE_3_ERR_R {
        KEY_PURPOSE_3_ERR_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_4_err(&self) -> KEY_PURPOSE_4_ERR_R {
        KEY_PURPOSE_4_ERR_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_5_err(&self) -> KEY_PURPOSE_5_ERR_R {
        KEY_PURPOSE_5_ERR_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn rpt4_reserved0_err(&self) -> RPT4_RESERVED0_ERR_R {
        RPT4_RESERVED0_ERR_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SECURE_BOOT_EN_ERR_R {
        SECURE_BOOT_EN_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_usb_jtag_err(&self) -> DIS_USB_JTAG_ERR_R {
        DIS_USB_JTAG_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_usb_device_err(&self) -> DIS_USB_DEVICE_ERR_R {
        DIS_USB_DEVICE_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn strap_jtag_sel_err(&self) -> STRAP_JTAG_SEL_ERR_R {
        STRAP_JTAG_SEL_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn usb_phy_sel_err(&self) -> USB_PHY_SEL_ERR_R {
        USB_PHY_SEL_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn power_glitch_dsense_err(&self) -> POWER_GLITCH_DSENSE_ERR_R {
        POWER_GLITCH_DSENSE_ERR_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FLASH_TPUW_ERR_R {
        FLASH_TPUW_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
