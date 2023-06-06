#[doc = "Register `RD_REPEAT_DATA3` reader"]
pub struct R(crate::R<RD_REPEAT_DATA3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - Set this bit to disable all download boot modes."]
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `DIS_LEGACY_SPI_BOOT` reader - Set this bit to disable Legacy SPI boot mode."]
pub type DIS_LEGACY_SPI_BOOT_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CHANNEL` reader - Selects the default UART for printing boot messages. 0: UART0. 1: UART1."]
pub type UART_PRINT_CHANNEL_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED3` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED3_R = crate::BitReader;
#[doc = "Field `DIS_USB_DOWNLOAD_MODE` reader - Set this bit to disable use of USB OTG in UART download boot mode."]
pub type DIS_USB_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` reader - Set this bit to enable secure UART download mode (read/write flash only)."]
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL` reader - Set the default UART boot message output mode. 00: Enabled. 01: Enable when GPIO46 is low at reset. 10: Enable when GPIO46 is high at reset. 11: Disabled."]
pub type UART_PRINT_CONTROL_R = crate::FieldReader;
#[doc = "Field `PIN_POWER_SELECTION` reader - Set default power supply for GPIO33-GPIO37, set when SPI flash is initialized. 0: VDD3P3_CPU. 1: VDD_SPI."]
pub type PIN_POWER_SELECTION_R = crate::BitReader;
#[doc = "Field `FLASH_TYPE` reader - SPI flash type. 0: maximum four data lines, 1: eight data lines."]
pub type FLASH_TYPE_R = crate::BitReader;
#[doc = "Field `FORCE_SEND_RESUME` reader - If set, forces ROM code to send an SPI flash resume command during SPI boot."]
pub type FORCE_SEND_RESUME_R = crate::BitReader;
#[doc = "Field `SECURE_VERSION` reader - Secure version (used by ESP-IDF anti-rollback feature)."]
pub type SECURE_VERSION_R = crate::FieldReader<u16>;
#[doc = "Field `RPT4_RESERVED2` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED2_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Set this bit to disable all download boot modes."]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to disable Legacy SPI boot mode."]
    #[inline(always)]
    pub fn dis_legacy_spi_boot(&self) -> DIS_LEGACY_SPI_BOOT_R {
        DIS_LEGACY_SPI_BOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Selects the default UART for printing boot messages. 0: UART0. 1: UART1."]
    #[inline(always)]
    pub fn uart_print_channel(&self) -> UART_PRINT_CHANNEL_R {
        UART_PRINT_CHANNEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved3(&self) -> RPT4_RESERVED3_R {
        RPT4_RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to disable use of USB OTG in UART download boot mode."]
    #[inline(always)]
    pub fn dis_usb_download_mode(&self) -> DIS_USB_DOWNLOAD_MODE_R {
        DIS_USB_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable secure UART download mode (read/write flash only)."]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set the default UART boot message output mode. 00: Enabled. 01: Enable when GPIO46 is low at reset. 10: Enable when GPIO46 is high at reset. 11: Disabled."]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Set default power supply for GPIO33-GPIO37, set when SPI flash is initialized. 0: VDD3P3_CPU. 1: VDD_SPI."]
    #[inline(always)]
    pub fn pin_power_selection(&self) -> PIN_POWER_SELECTION_R {
        PIN_POWER_SELECTION_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - SPI flash type. 0: maximum four data lines, 1: eight data lines."]
    #[inline(always)]
    pub fn flash_type(&self) -> FLASH_TYPE_R {
        FLASH_TYPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - If set, forces ROM code to send an SPI flash resume command during SPI boot."]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:26 - Secure version (used by ESP-IDF anti-rollback feature)."]
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 11) & 0xffff) as u16)
    }
    #[doc = "Bits 27:31 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved2(&self) -> RPT4_RESERVED2_R {
        RPT4_RESERVED2_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA3")
            .field(
                "dis_download_mode",
                &format_args!("{}", self.dis_download_mode().bit()),
            )
            .field(
                "dis_legacy_spi_boot",
                &format_args!("{}", self.dis_legacy_spi_boot().bit()),
            )
            .field(
                "uart_print_channel",
                &format_args!("{}", self.uart_print_channel().bit()),
            )
            .field(
                "rpt4_reserved3",
                &format_args!("{}", self.rpt4_reserved3().bit()),
            )
            .field(
                "dis_usb_download_mode",
                &format_args!("{}", self.dis_usb_download_mode().bit()),
            )
            .field(
                "enable_security_download",
                &format_args!("{}", self.enable_security_download().bit()),
            )
            .field(
                "uart_print_control",
                &format_args!("{}", self.uart_print_control().bits()),
            )
            .field(
                "pin_power_selection",
                &format_args!("{}", self.pin_power_selection().bit()),
            )
            .field("flash_type", &format_args!("{}", self.flash_type().bit()))
            .field(
                "force_send_resume",
                &format_args!("{}", self.force_send_resume().bit()),
            )
            .field(
                "secure_version",
                &format_args!("{}", self.secure_version().bits()),
            )
            .field(
                "rpt4_reserved2",
                &format_args!("{}", self.rpt4_reserved2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_DATA3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Register 4 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data3](index.html) module"]
pub struct RD_REPEAT_DATA3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data3::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA3 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
