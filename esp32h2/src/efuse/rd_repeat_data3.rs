///Register `RD_REPEAT_DATA3` reader
pub type R = crate::R<RD_REPEAT_DATA3_SPEC>;
///Field `DIS_DOWNLOAD_MODE` reader - Represents whether Download mode is disabled or enabled. 1: disabled. 0: enabled.
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
///Field `DIS_DIRECT_BOOT` reader - Represents whether direct boot mode is disabled or enabled. 1: disabled. 0: enabled.
pub type DIS_DIRECT_BOOT_R = crate::BitReader;
///Field `DIS_USB_PRINT` reader - Represents whether print from USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
pub type DIS_USB_PRINT_R = crate::BitReader;
///Field `RPT4_RESERVED3_5` reader - Reserved.
pub type RPT4_RESERVED3_5_R = crate::BitReader;
///Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE` reader - Represents whether the USB-Serial-JTAG download function is disabled or enabled. 1: disabled. 0: enabled.
pub type DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R = crate::BitReader;
///Field `ENABLE_SECURITY_DOWNLOAD` reader - Represents whether security download is enabled or disabled. 1: enabled. 0: disabled.
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::BitReader;
///Field `UART_PRINT_CONTROL` reader - Represents the type of UART printing. 00: force enable printing. 01: enable printing when GPIO8 is reset at low level. 10: enable printing when GPIO8 is reset at high level. 11: force disable printing.
pub type UART_PRINT_CONTROL_R = crate::FieldReader;
///Field `FORCE_SEND_RESUME` reader - Represents whether ROM code is forced to send a resume command during SPI boot. 1: forced. 0:not forced.
pub type FORCE_SEND_RESUME_R = crate::BitReader;
///Field `SECURE_VERSION` reader - Represents the version used by ESP-IDF anti-rollback feature.
pub type SECURE_VERSION_R = crate::FieldReader<u16>;
///Field `SECURE_BOOT_DISABLE_FAST_WAKE` reader - Represents whether FAST VERIFY ON WAKE is disabled or enabled when Secure Boot is enabled. 1: disabled. 0: enabled.
pub type SECURE_BOOT_DISABLE_FAST_WAKE_R = crate::BitReader;
///Field `HYS_EN_PAD0` reader - Represents whether the hysteresis function of corresponding PAD is enabled. 1: enabled. 0:disabled.
pub type HYS_EN_PAD0_R = crate::FieldReader;
impl R {
    ///Bit 0 - Represents whether Download mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Represents whether direct boot mode is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_direct_boot(&self) -> DIS_DIRECT_BOOT_R {
        DIS_DIRECT_BOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Represents whether print from USB-Serial-JTAG is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_usb_print(&self) -> DIS_USB_PRINT_R {
        DIS_USB_PRINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reserved.
    #[inline(always)]
    pub fn rpt4_reserved3_5(&self) -> RPT4_RESERVED3_5_R {
        RPT4_RESERVED3_5_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Represents whether the USB-Serial-JTAG download function is disabled or enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode(&self) -> DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R {
        DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Represents whether security download is enabled or disabled. 1: enabled. 0: disabled.
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Represents the type of UART printing. 00: force enable printing. 01: enable printing when GPIO8 is reset at low level. 10: enable printing when GPIO8 is reset at high level. 11: force disable printing.
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Represents whether ROM code is forced to send a resume command during SPI boot. 1: forced. 0:not forced.
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:24 - Represents the version used by ESP-IDF anti-rollback feature.
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    ///Bit 25 - Represents whether FAST VERIFY ON WAKE is disabled or enabled when Secure Boot is enabled. 1: disabled. 0: enabled.
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake(&self) -> SECURE_BOOT_DISABLE_FAST_WAKE_R {
        SECURE_BOOT_DISABLE_FAST_WAKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bits 26:31 - Represents whether the hysteresis function of corresponding PAD is enabled. 1: enabled. 0:disabled.
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
            .field("dis_usb_print", &self.dis_usb_print())
            .field("rpt4_reserved3_5", &self.rpt4_reserved3_5())
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
/**BLOCK0 data register 4.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_data3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_REPEAT_DATA3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_repeat_data3::R`](R) reader structure
impl crate::Readable for RD_REPEAT_DATA3_SPEC {}
///`reset()` method sets RD_REPEAT_DATA3 to value 0
impl crate::Resettable for RD_REPEAT_DATA3_SPEC {
    const RESET_VALUE: u32 = 0;
}
