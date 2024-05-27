///Register `RD_REPEAT_ERR3` reader
pub type R = crate::R<RD_REPEAT_ERR3_SPEC>;
///Field `DIS_DOWNLOAD_MODE_ERR` reader - Indicates a programming error of DIS_DOWNLOAD_MODE.
pub type DIS_DOWNLOAD_MODE_ERR_R = crate::BitReader;
///Field `DIS_DIRECT_BOOT_ERR` reader - Indicates a programming error of DIS_DIRECT_BOOT.
pub type DIS_DIRECT_BOOT_ERR_R = crate::BitReader;
///Field `DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR` reader - Indicates a programming error of DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR.
pub type DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_R = crate::BitReader;
///Field `LOCK_KM_KEY_ERR` reader - TBD
pub type LOCK_KM_KEY_ERR_R = crate::BitReader;
///Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR` reader - Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE.
pub type DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R = crate::BitReader;
///Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - Indicates a programming error of ENABLE_SECURITY_DOWNLOAD.
pub type ENABLE_SECURITY_DOWNLOAD_ERR_R = crate::BitReader;
///Field `UART_PRINT_CONTROL_ERR` reader - Indicates a programming error of UART_PRINT_CONTROL.
pub type UART_PRINT_CONTROL_ERR_R = crate::FieldReader;
///Field `FORCE_SEND_RESUME_ERR` reader - Indicates a programming error of FORCE_SEND_RESUME.
pub type FORCE_SEND_RESUME_ERR_R = crate::BitReader;
///Field `SECURE_VERSION_ERR` reader - Indicates a programming error of SECURE VERSION.
pub type SECURE_VERSION_ERR_R = crate::FieldReader<u16>;
///Field `SECURE_BOOT_DISABLE_FAST_WAKE_ERR` reader - Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE.
pub type SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R = crate::BitReader;
///Field `HYS_EN_PAD_ERR` reader - Indicates a programming error of HYS_EN_PAD.
pub type HYS_EN_PAD_ERR_R = crate::BitReader;
///Field `DCDC_VSET_ERR` reader - Indicates a programming error of DCDC_VSET.
pub type DCDC_VSET_ERR_R = crate::FieldReader;
impl R {
    ///Bit 0 - Indicates a programming error of DIS_DOWNLOAD_MODE.
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - Indicates a programming error of DIS_DIRECT_BOOT.
    #[inline(always)]
    pub fn dis_direct_boot_err(&self) -> DIS_DIRECT_BOOT_ERR_R {
        DIS_DIRECT_BOOT_ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - Indicates a programming error of DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print_err(&self) -> DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_R {
        DIS_USB_SERIAL_JTAG_ROM_PRINT_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - TBD
    #[inline(always)]
    pub fn lock_km_key_err(&self) -> LOCK_KM_KEY_ERR_R {
        LOCK_KM_KEY_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - Indicates a programming error of DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE.
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode_err(&self) -> DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R {
        DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Indicates a programming error of ENABLE_SECURITY_DOWNLOAD.
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> ENABLE_SECURITY_DOWNLOAD_ERR_R {
        ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bits 6:7 - Indicates a programming error of UART_PRINT_CONTROL.
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UART_PRINT_CONTROL_ERR_R {
        UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 6) & 3) as u8)
    }
    ///Bit 8 - Indicates a programming error of FORCE_SEND_RESUME.
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> FORCE_SEND_RESUME_ERR_R {
        FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:24 - Indicates a programming error of SECURE VERSION.
    #[inline(always)]
    pub fn secure_version_err(&self) -> SECURE_VERSION_ERR_R {
        SECURE_VERSION_ERR_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    ///Bit 25 - Indicates a programming error of SECURE_BOOT_DISABLE_FAST_WAKE.
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake_err(&self) -> SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R {
        SECURE_BOOT_DISABLE_FAST_WAKE_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    ///Bit 26 - Indicates a programming error of HYS_EN_PAD.
    #[inline(always)]
    pub fn hys_en_pad_err(&self) -> HYS_EN_PAD_ERR_R {
        HYS_EN_PAD_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bits 27:31 - Indicates a programming error of DCDC_VSET.
    #[inline(always)]
    pub fn dcdc_vset_err(&self) -> DCDC_VSET_ERR_R {
        DCDC_VSET_ERR_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR3")
            .field("dis_download_mode_err", &self.dis_download_mode_err())
            .field("dis_direct_boot_err", &self.dis_direct_boot_err())
            .field(
                "dis_usb_serial_jtag_rom_print_err",
                &self.dis_usb_serial_jtag_rom_print_err(),
            )
            .field("lock_km_key_err", &self.lock_km_key_err())
            .field(
                "dis_usb_serial_jtag_download_mode_err",
                &self.dis_usb_serial_jtag_download_mode_err(),
            )
            .field(
                "enable_security_download_err",
                &self.enable_security_download_err(),
            )
            .field("uart_print_control_err", &self.uart_print_control_err())
            .field("force_send_resume_err", &self.force_send_resume_err())
            .field("secure_version_err", &self.secure_version_err())
            .field(
                "secure_boot_disable_fast_wake_err",
                &self.secure_boot_disable_fast_wake_err(),
            )
            .field("hys_en_pad_err", &self.hys_en_pad_err())
            .field("dcdc_vset_err", &self.dcdc_vset_err())
            .finish()
    }
}
/**Programming error record register 3 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_REPEAT_ERR3_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR3_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_repeat_err3::R`](R) reader structure
impl crate::Readable for RD_REPEAT_ERR3_SPEC {}
///`reset()` method sets RD_REPEAT_ERR3 to value 0
impl crate::Resettable for RD_REPEAT_ERR3_SPEC {
    const RESET_VALUE: u32 = 0;
}
