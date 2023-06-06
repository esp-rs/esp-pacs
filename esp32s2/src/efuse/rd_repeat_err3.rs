#[doc = "Register `RD_REPEAT_ERR3` reader"]
pub struct R(crate::R<RD_REPEAT_ERR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_MODE."]
pub type DIS_DOWNLOAD_MODE_ERR_R = crate::BitReader;
#[doc = "Field `DIS_LEGACY_SPI_BOOT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_LEGACY_SPI_BOOT."]
pub type DIS_LEGACY_SPI_BOOT_ERR_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CHANNEL_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_UART_PRINT_CHANNEL."]
pub type UART_PRINT_CHANNEL_ERR_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED3_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED3."]
pub type RPT4_RESERVED3_ERR_R = crate::BitReader;
#[doc = "Field `DIS_USB_DOWNLOAD_MODE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_DIS_USB_DOWNLOAD_MODE."]
pub type DIS_USB_DOWNLOAD_MODE_ERR_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_ENABLE_SECURITY_DOWNLOAD."]
pub type ENABLE_SECURITY_DOWNLOAD_ERR_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_UART_PRINT_CONTROL."]
pub type UART_PRINT_CONTROL_ERR_R = crate::FieldReader;
#[doc = "Field `PIN_POWER_SELECTION_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_PIN_POWER_SELECTION."]
pub type PIN_POWER_SELECTION_ERR_R = crate::BitReader;
#[doc = "Field `FLASH_TYPE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_FLASH_TYPE."]
pub type FLASH_TYPE_ERR_R = crate::BitReader;
#[doc = "Field `FORCE_SEND_RESUME_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_FORCE_SEND_RESUME."]
pub type FORCE_SEND_RESUME_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_VERSION_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_VERSION."]
pub type SECURE_VERSION_ERR_R = crate::FieldReader<u16>;
#[doc = "Field `RPT4_RESERVED2_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED2."]
pub type RPT4_RESERVED2_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_DOWNLOAD_MODE."]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_LEGACY_SPI_BOOT."]
    #[inline(always)]
    pub fn dis_legacy_spi_boot_err(&self) -> DIS_LEGACY_SPI_BOOT_ERR_R {
        DIS_LEGACY_SPI_BOOT_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Any bit equal to 1 denotes a programming error in EFUSE_UART_PRINT_CHANNEL."]
    #[inline(always)]
    pub fn uart_print_channel_err(&self) -> UART_PRINT_CHANNEL_ERR_R {
        UART_PRINT_CHANNEL_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED3."]
    #[inline(always)]
    pub fn rpt4_reserved3_err(&self) -> RPT4_RESERVED3_ERR_R {
        RPT4_RESERVED3_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Any bit equal to 1 denotes a programming error in EFUSE_DIS_USB_DOWNLOAD_MODE."]
    #[inline(always)]
    pub fn dis_usb_download_mode_err(&self) -> DIS_USB_DOWNLOAD_MODE_ERR_R {
        DIS_USB_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Any bit equal to 1 denotes a programming error in EFUSE_ENABLE_SECURITY_DOWNLOAD."]
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> ENABLE_SECURITY_DOWNLOAD_ERR_R {
        ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Any bit equal to 1 denotes a programming error in EFUSE_UART_PRINT_CONTROL."]
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UART_PRINT_CONTROL_ERR_R {
        UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Any bit equal to 1 denotes a programming error in EFUSE_PIN_POWER_SELECTION."]
    #[inline(always)]
    pub fn pin_power_selection_err(&self) -> PIN_POWER_SELECTION_ERR_R {
        PIN_POWER_SELECTION_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Any bit equal to 1 denotes a programming error in EFUSE_FLASH_TYPE."]
    #[inline(always)]
    pub fn flash_type_err(&self) -> FLASH_TYPE_ERR_R {
        FLASH_TYPE_ERR_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Any bit equal to 1 denotes a programming error in EFUSE_FORCE_SEND_RESUME."]
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> FORCE_SEND_RESUME_ERR_R {
        FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:26 - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_VERSION."]
    #[inline(always)]
    pub fn secure_version_err(&self) -> SECURE_VERSION_ERR_R {
        SECURE_VERSION_ERR_R::new(((self.bits >> 11) & 0xffff) as u16)
    }
    #[doc = "Bits 27:31 - Any bit equal to 1 denotes a programming error in EFUSE_RPT4_RESERVED2."]
    #[inline(always)]
    pub fn rpt4_reserved2_err(&self) -> RPT4_RESERVED2_ERR_R {
        RPT4_RESERVED2_ERR_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR3")
            .field(
                "dis_download_mode_err",
                &format_args!("{}", self.dis_download_mode_err().bit()),
            )
            .field(
                "dis_legacy_spi_boot_err",
                &format_args!("{}", self.dis_legacy_spi_boot_err().bit()),
            )
            .field(
                "uart_print_channel_err",
                &format_args!("{}", self.uart_print_channel_err().bit()),
            )
            .field(
                "rpt4_reserved3_err",
                &format_args!("{}", self.rpt4_reserved3_err().bit()),
            )
            .field(
                "dis_usb_download_mode_err",
                &format_args!("{}", self.dis_usb_download_mode_err().bit()),
            )
            .field(
                "enable_security_download_err",
                &format_args!("{}", self.enable_security_download_err().bit()),
            )
            .field(
                "uart_print_control_err",
                &format_args!("{}", self.uart_print_control_err().bits()),
            )
            .field(
                "pin_power_selection_err",
                &format_args!("{}", self.pin_power_selection_err().bit()),
            )
            .field(
                "flash_type_err",
                &format_args!("{}", self.flash_type_err().bit()),
            )
            .field(
                "force_send_resume_err",
                &format_args!("{}", self.force_send_resume_err().bit()),
            )
            .field(
                "secure_version_err",
                &format_args!("{}", self.secure_version_err().bits()),
            )
            .field(
                "rpt4_reserved2_err",
                &format_args!("{}", self.rpt4_reserved2_err().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_ERR3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 3 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err3](index.html) module"]
pub struct RD_REPEAT_ERR3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err3::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR3_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR3 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
