///Register `RD_REPEAT_DATA3` reader
pub type R = crate::R<RD_REPEAT_DATA3_SPEC>;
///Field `DIS_DOWNLOAD_MODE` reader - Set this bit to disable all download boot modes.
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
///Field `DIS_LEGACY_SPI_BOOT` reader - Set this bit to disable Legacy SPI boot mode.
pub type DIS_LEGACY_SPI_BOOT_R = crate::BitReader;
///Field `UART_PRINT_CHANNEL` reader - Selects the default UART for printing boot messages. 0: UART0. 1: UART1.
pub type UART_PRINT_CHANNEL_R = crate::BitReader;
///Field `RPT4_RESERVED3` reader - Reserved (used for four backups method).
pub type RPT4_RESERVED3_R = crate::BitReader;
///Field `DIS_USB_DOWNLOAD_MODE` reader - Set this bit to disable use of USB OTG in UART download boot mode.
pub type DIS_USB_DOWNLOAD_MODE_R = crate::BitReader;
///Field `ENABLE_SECURITY_DOWNLOAD` reader - Set this bit to enable secure UART download mode (read/write flash only).
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::BitReader;
///Field `UART_PRINT_CONTROL` reader - Set the default UART boot message output mode. 00: Enabled. 01: Enable when GPIO46 is low at reset. 10: Enable when GPIO46 is high at reset. 11: Disabled.
pub type UART_PRINT_CONTROL_R = crate::FieldReader;
///Field `PIN_POWER_SELECTION` reader - Set default power supply for GPIO33-GPIO37, set when SPI flash is initialized. 0: VDD3P3_CPU. 1: VDD_SPI.
pub type PIN_POWER_SELECTION_R = crate::BitReader;
///Field `FLASH_TYPE` reader - SPI flash type. 0: maximum four data lines, 1: eight data lines.
pub type FLASH_TYPE_R = crate::BitReader;
///Field `FORCE_SEND_RESUME` reader - If set, forces ROM code to send an SPI flash resume command during SPI boot.
pub type FORCE_SEND_RESUME_R = crate::BitReader;
///Field `SECURE_VERSION` reader - Secure version (used by ESP-IDF anti-rollback feature).
pub type SECURE_VERSION_R = crate::FieldReader<u16>;
///Field `RPT4_RESERVED2` reader - Reserved (used for four backups method).
pub type RPT4_RESERVED2_R = crate::FieldReader;
impl R {
    ///Bit 0 - Set this bit to disable all download boot modes.
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Set this bit to disable Legacy SPI boot mode.
    #[inline(always)]
    pub fn dis_legacy_spi_boot(&self) -> DIS_LEGACY_SPI_BOOT_R {
        DIS_LEGACY_SPI_BOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Selects the default UART for printing boot messages. 0: UART0. 1: UART1.
    #[inline(always)]
    pub fn uart_print_channel(&self) -> UART_PRINT_CHANNEL_R {
        UART_PRINT_CHANNEL_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - Reserved (used for four backups method).
    #[inline(always)]
    pub fn rpt4_reserved3(&self) -> RPT4_RESERVED3_R {
        RPT4_RESERVED3_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Set this bit to disable use of USB OTG in UART download boot mode.
    #[inline(always)]
    pub fn dis_usb_download_mode(&self) -> DIS_USB_DOWNLOAD_MODE_R {
        DIS_USB_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Set this bit to enable secure UART download mode (read/write flash only).
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Set the default UART boot message output mode. 00: Enabled. 01: Enable when GPIO46 is low at reset. 10: Enable when GPIO46 is high at reset. 11: Disabled.
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Set default power supply for GPIO33-GPIO37, set when SPI flash is initialized. 0: VDD3P3_CPU. 1: VDD_SPI.
    #[inline(always)]
    pub fn pin_power_selection(&self) -> PIN_POWER_SELECTION_R {
        PIN_POWER_SELECTION_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bit 9 - SPI flash type. 0: maximum four data lines, 1: eight data lines.
    #[inline(always)]
    pub fn flash_type(&self) -> FLASH_TYPE_R {
        FLASH_TYPE_R::new(((self.bits >> 9) & 1) != 0)
    }
    ///Bit 10 - If set, forces ROM code to send an SPI flash resume command during SPI boot.
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 10) & 1) != 0)
    }
    ///Bits 11:26 - Secure version (used by ESP-IDF anti-rollback feature).
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 11) & 0xffff) as u16)
    }
    ///Bits 27:31 - Reserved (used for four backups method).
    #[inline(always)]
    pub fn rpt4_reserved2(&self) -> RPT4_RESERVED2_R {
        RPT4_RESERVED2_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA3")
            .field("dis_download_mode", &self.dis_download_mode())
            .field("dis_legacy_spi_boot", &self.dis_legacy_spi_boot())
            .field("uart_print_channel", &self.uart_print_channel())
            .field("rpt4_reserved3", &self.rpt4_reserved3())
            .field("dis_usb_download_mode", &self.dis_usb_download_mode())
            .field("enable_security_download", &self.enable_security_download())
            .field("uart_print_control", &self.uart_print_control())
            .field("pin_power_selection", &self.pin_power_selection())
            .field("flash_type", &self.flash_type())
            .field("force_send_resume", &self.force_send_resume())
            .field("secure_version", &self.secure_version())
            .field("rpt4_reserved2", &self.rpt4_reserved2())
            .finish()
    }
}
/**Register 4 of BLOCK0.

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
