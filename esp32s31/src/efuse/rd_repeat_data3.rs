#[doc = "Register `RD_REPEAT_DATA3` reader"]
pub type R = crate::R<RD_REPEAT_DATA3_SPEC>;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL` reader - Represents the control of the xts pseudo-round anti-dpa attack function.\\\\ 0: controlled by register.\\\\ 1-3: the higer the value is, the more pseudo-rounds are inserted to the xts-aes calculation\\\\"]
pub type XTS_DPA_PSEUDO_LEVEL_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Represents whether secure boot is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Represents whether revoking aggressive secure boot is enabled or disabled.\\\\ 1: enabled.\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
#[doc = "Field `FLASH_TYPE` reader - flash type: 0: nor flash, 1: nand flash"]
pub type FLASH_TYPE_R = crate::BitReader;
#[doc = "Field `DIS_USB_OTG_DOWNLOAD_MODE` reader - Set this bit to disable download via USB-OTG."]
pub type DIS_USB_OTG_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW` reader - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
pub type FLASH_TPUW_R = crate::FieldReader;
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - Represents whether Download mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `DIS_DIRECT_BOOT` reader - Represents whether direct boot mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_DIRECT_BOOT_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_ROM_PRINT` reader - Represents whether print from USB-Serial-JTAG is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_USB_SERIAL_JTAG_ROM_PRINT_R = crate::BitReader;
#[doc = "Field `LOCK_KM_KEY` reader - Represetns whether to lock the efuse xts key.\\\\ 1. Lock\\\\ 0: Unlock\\\\"]
pub type LOCK_KM_KEY_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE` reader - Represents whether the USB-Serial-JTAG download function is disabled or enabled.\\\\ 1: Disable\\\\ 0: Enable\\\\"]
pub type DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` reader - Represents whether security download is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL` reader - Represents the type of UART printing.\\\\ 00: force enable printing\\\\ 01: enable printing when GPIO8 is reset at low level\\\\ 10: enable printing when GPIO8 is reset at high level\\\\ 11: force disable printing\\\\"]
pub type UART_PRINT_CONTROL_R = crate::FieldReader;
#[doc = "Field `FORCE_SEND_RESUME` reader - Represents whether ROM code is forced to send a resume command during SPI boot.\\\\ 1: forced\\\\ 0:not forced\\\\"]
pub type FORCE_SEND_RESUME_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Represents the control of the xts pseudo-round anti-dpa attack function.\\\\ 0: controlled by register.\\\\ 1-3: the higer the value is, the more pseudo-rounds are inserted to the xts-aes calculation\\\\"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level(&self) -> XTS_DPA_PSEUDO_LEVEL_R {
        XTS_DPA_PSEUDO_LEVEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents whether secure boot is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents whether revoking aggressive secure boot is enabled or disabled.\\\\ 1: enabled.\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - flash type: 0: nor flash, 1: nand flash"]
    #[inline(always)]
    pub fn flash_type(&self) -> FLASH_TYPE_R {
        FLASH_TYPE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to disable download via USB-OTG."]
    #[inline(always)]
    pub fn dis_usb_otg_download_mode(&self) -> DIS_USB_OTG_DOWNLOAD_MODE_R {
        DIS_USB_OTG_DOWNLOAD_MODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:15 - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Represents whether Download mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Represents whether direct boot mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_direct_boot(&self) -> DIS_DIRECT_BOOT_R {
        DIS_DIRECT_BOOT_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents whether print from USB-Serial-JTAG is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print(&self) -> DIS_USB_SERIAL_JTAG_ROM_PRINT_R {
        DIS_USB_SERIAL_JTAG_ROM_PRINT_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represetns whether to lock the efuse xts key.\\\\ 1. Lock\\\\ 0: Unlock\\\\"]
    #[inline(always)]
    pub fn lock_km_key(&self) -> LOCK_KM_KEY_R {
        LOCK_KM_KEY_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether the USB-Serial-JTAG download function is disabled or enabled.\\\\ 1: Disable\\\\ 0: Enable\\\\"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode(&self) -> DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R {
        DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents whether security download is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Represents the type of UART printing.\\\\ 00: force enable printing\\\\ 01: enable printing when GPIO8 is reset at low level\\\\ 10: enable printing when GPIO8 is reset at high level\\\\ 11: force disable printing\\\\"]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24 - Represents whether ROM code is forced to send a resume command during SPI boot.\\\\ 1: forced\\\\ 0:not forced\\\\"]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA3")
            .field("xts_dpa_pseudo_level", &self.xts_dpa_pseudo_level())
            .field("secure_boot_en", &self.secure_boot_en())
            .field(
                "secure_boot_aggressive_revoke",
                &self.secure_boot_aggressive_revoke(),
            )
            .field("flash_type", &self.flash_type())
            .field(
                "dis_usb_otg_download_mode",
                &self.dis_usb_otg_download_mode(),
            )
            .field("flash_tpuw", &self.flash_tpuw())
            .field("dis_download_mode", &self.dis_download_mode())
            .field("dis_direct_boot", &self.dis_direct_boot())
            .field(
                "dis_usb_serial_jtag_rom_print",
                &self.dis_usb_serial_jtag_rom_print(),
            )
            .field("lock_km_key", &self.lock_km_key())
            .field(
                "dis_usb_serial_jtag_download_mode",
                &self.dis_usb_serial_jtag_download_mode(),
            )
            .field("enable_security_download", &self.enable_security_download())
            .field("uart_print_control", &self.uart_print_control())
            .field("force_send_resume", &self.force_send_resume())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data3::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data3::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA3_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA3 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA3_SPEC {}
