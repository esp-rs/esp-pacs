#[doc = "Register `RD_REPEAT_DATA3` reader"]
pub type R = crate::R<RD_REPEAT_DATA3_SPEC>;
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - Represents whether all download modes are disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `DIS_DIRECT_BOOT` reader - Represents whether direct boot mode is disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_DIRECT_BOOT_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_ROM_PRINT` reader - Represents whether print from USB-Serial-JTAG during ROM boot is disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_USB_SERIAL_JTAG_ROM_PRINT_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE` reader - Represents whether the USB-Serial-JTAG download function is disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` reader - Represents whether security download is enabled. Only UART is supported for download. Reading/writing RAM or registers is not supported (i.e. Stub download is not supported).\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL` reader - Represents the type of UART printing.\\\\ 0: Force enable printing.\\\\ 1: Enable printing when GPIO8 is reset at low level.\\\\ 2: Enable printing when GPIO8 is reset at high level.\\\\ 3: Force disable printing.\\\\"]
pub type UART_PRINT_CONTROL_R = crate::FieldReader;
#[doc = "Field `FORCE_SEND_RESUME` reader - Represents whether ROM code is forced to send a resume command during SPI boot.\\\\ 1: Forced\\\\ 0: Not forced\\\\"]
pub type FORCE_SEND_RESUME_R = crate::BitReader;
#[doc = "Field `SECURE_VERSION` reader - Represents the security version used by ESP-IDF anti-rollback feature.\\\\"]
pub type SECURE_VERSION_R = crate::FieldReader<u16>;
#[doc = "Field `SECURE_BOOT_DISABLE_FAST_WAKE` reader - Represents whether FAST VERIFY ON WAKE is disabled when Secure Boot is enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
pub type SECURE_BOOT_DISABLE_FAST_WAKE_R = crate::BitReader;
#[doc = "Field `HYS_EN_PAD0` reader - Represents whether to enable the hysteresis function of pad 0-5.\\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
pub type HYS_EN_PAD0_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Represents whether all download modes are disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Represents whether direct boot mode is disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_direct_boot(&self) -> DIS_DIRECT_BOOT_R {
        DIS_DIRECT_BOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Represents whether print from USB-Serial-JTAG during ROM boot is disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print(&self) -> DIS_USB_SERIAL_JTAG_ROM_PRINT_R {
        DIS_USB_SERIAL_JTAG_ROM_PRINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents whether the USB-Serial-JTAG download function is disabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode(&self) -> DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R {
        DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents whether security download is enabled. Only UART is supported for download. Reading/writing RAM or registers is not supported (i.e. Stub download is not supported).\\\\ 1: Enabled\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Represents the type of UART printing.\\\\ 0: Force enable printing.\\\\ 1: Enable printing when GPIO8 is reset at low level.\\\\ 2: Enable printing when GPIO8 is reset at high level.\\\\ 3: Force disable printing.\\\\"]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Represents whether ROM code is forced to send a resume command during SPI boot.\\\\ 1: Forced\\\\ 0: Not forced\\\\"]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:24 - Represents the security version used by ESP-IDF anti-rollback feature.\\\\"]
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - Represents whether FAST VERIFY ON WAKE is disabled when Secure Boot is enabled.\\\\ 1: Disabled\\\\ 0: Enabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake(&self) -> SECURE_BOOT_DISABLE_FAST_WAKE_R {
        SECURE_BOOT_DISABLE_FAST_WAKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:31 - Represents whether to enable the hysteresis function of pad 0-5.\\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
    #[inline(always)]
    pub fn hys_en_pad0(&self) -> HYS_EN_PAD0_R {
        HYS_EN_PAD0_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA3")
            .field("dis_download_mode", &self.dis_download_mode())
            .field("dis_direct_boot", &self.dis_direct_boot())
            .field(
                "dis_usb_serial_jtag_rom_print",
                &self.dis_usb_serial_jtag_rom_print(),
            )
            .field(
                "dis_usb_serial_jtag_download_mode",
                &self.dis_usb_serial_jtag_download_mode(),
            )
            .field("enable_security_download", &self.enable_security_download())
            .field("uart_print_control", &self.uart_print_control())
            .field("force_send_resume", &self.force_send_resume())
            .field("secure_version", &self.secure_version())
            .field(
                "secure_boot_disable_fast_wake",
                &self.secure_boot_disable_fast_wake(),
            )
            .field("hys_en_pad0", &self.hys_en_pad0())
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
