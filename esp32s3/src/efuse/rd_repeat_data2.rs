#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub type R = crate::R<RD_REPEAT_DATA2_SPEC>;
#[doc = "Field `KEY_PURPOSE_2` reader - Purpose of Key2."]
pub type KEY_PURPOSE_2_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_3` reader - Purpose of Key3."]
pub type KEY_PURPOSE_3_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_4` reader - Purpose of Key4."]
pub type KEY_PURPOSE_4_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_5` reader - Purpose of Key5."]
pub type KEY_PURPOSE_5_R = crate::FieldReader;
#[doc = "Field `RPT4_RESERVED0` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED0_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Set this bit to enable secure boot."]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Set this bit to enable revoking aggressive secure boot."]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
#[doc = "Field `DIS_USB_JTAG` reader - Set this bit to disable function of usb switch to jtag in module of usb device."]
pub type DIS_USB_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_USB_DEVICE` reader - Set this bit to disable usb device."]
pub type DIS_USB_DEVICE_R = crate::BitReader;
#[doc = "Field `STRAP_JTAG_SEL` reader - Set this bit to enable selection between usb_to_jtag and pad_to_jtag through strapping gpio10 when both reg_dis_usb_jtag and reg_dis_pad_jtag are equal to 0."]
pub type STRAP_JTAG_SEL_R = crate::BitReader;
#[doc = "Field `USB_PHY_SEL` reader - This bit is used to switch internal PHY and external PHY for USB OTG and USB Device. 0: internal PHY is assigned to USB Device while external PHY is assigned to USB OTG. 1: internal PHY is assigned to USB OTG while external PHY is assigned to USB Device."]
pub type USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_DSENSE` reader - Sample delay configuration of power glitch."]
pub type POWER_GLITCH_DSENSE_R = crate::FieldReader;
#[doc = "Field `FLASH_TPUW` reader - Configures flash waiting time after power-up, in unit of ms. If the value is less than 15, the waiting time is the configurable value. Otherwise, the waiting time is twice the configurable value."]
pub type FLASH_TPUW_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Purpose of Key2."]
    #[inline(always)]
    pub fn key_purpose_2(&self) -> KEY_PURPOSE_2_R {
        KEY_PURPOSE_2_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - Purpose of Key3."]
    #[inline(always)]
    pub fn key_purpose_3(&self) -> KEY_PURPOSE_3_R {
        KEY_PURPOSE_3_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - Purpose of Key4."]
    #[inline(always)]
    pub fn key_purpose_4(&self) -> KEY_PURPOSE_4_R {
        KEY_PURPOSE_4_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Purpose of Key5."]
    #[inline(always)]
    pub fn key_purpose_5(&self) -> KEY_PURPOSE_5_R {
        KEY_PURPOSE_5_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved0(&self) -> RPT4_RESERVED0_R {
        RPT4_RESERVED0_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Set this bit to enable secure boot."]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to enable revoking aggressive secure boot."]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to disable function of usb switch to jtag in module of usb device."]
    #[inline(always)]
    pub fn dis_usb_jtag(&self) -> DIS_USB_JTAG_R {
        DIS_USB_JTAG_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to disable usb device."]
    #[inline(always)]
    pub fn dis_usb_device(&self) -> DIS_USB_DEVICE_R {
        DIS_USB_DEVICE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to enable selection between usb_to_jtag and pad_to_jtag through strapping gpio10 when both reg_dis_usb_jtag and reg_dis_pad_jtag are equal to 0."]
    #[inline(always)]
    pub fn strap_jtag_sel(&self) -> STRAP_JTAG_SEL_R {
        STRAP_JTAG_SEL_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - This bit is used to switch internal PHY and external PHY for USB OTG and USB Device. 0: internal PHY is assigned to USB Device while external PHY is assigned to USB OTG. 1: internal PHY is assigned to USB OTG while external PHY is assigned to USB Device."]
    #[inline(always)]
    pub fn usb_phy_sel(&self) -> USB_PHY_SEL_R {
        USB_PHY_SEL_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Sample delay configuration of power glitch."]
    #[inline(always)]
    pub fn power_glitch_dsense(&self) -> POWER_GLITCH_DSENSE_R {
        POWER_GLITCH_DSENSE_R::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:31 - Configures flash waiting time after power-up, in unit of ms. If the value is less than 15, the waiting time is the configurable value. Otherwise, the waiting time is twice the configurable value."]
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
            .field("rpt4_reserved0", &self.rpt4_reserved0())
            .field("secure_boot_en", &self.secure_boot_en())
            .field(
                "secure_boot_aggressive_revoke",
                &self.secure_boot_aggressive_revoke(),
            )
            .field("dis_usb_jtag", &self.dis_usb_jtag())
            .field("dis_usb_device", &self.dis_usb_device())
            .field("strap_jtag_sel", &self.strap_jtag_sel())
            .field("usb_phy_sel", &self.usb_phy_sel())
            .field("power_glitch_dsense", &self.power_glitch_dsense())
            .field("flash_tpuw", &self.flash_tpuw())
            .finish()
    }
}
#[doc = "BLOCK0 data register 3.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {}
