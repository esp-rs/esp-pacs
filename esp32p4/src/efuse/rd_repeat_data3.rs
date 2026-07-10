#[doc = "Register `RD_REPEAT_DATA3` reader"]
pub type R = crate::R<RD_REPEAT_DATA3_SPEC>;
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - Set this bit to disable download mode (boot_mode\\[3:0\\] = 0, 1, 2, 4, 5, 6, 7)."]
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `DIS_DIRECT_BOOT` reader - Set this bit to disable direct boot mode"]
pub type DIS_DIRECT_BOOT_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_ROM_PRINT` reader - Set this bit to disable USB-Serial-JTAG print during rom boot."]
pub type DIS_USB_SERIAL_JTAG_ROM_PRINT_R = crate::BitReader;
#[doc = "Field `LOCK_KM_KEY` reader - set this bit to lock the key manager key after deploy"]
pub type LOCK_KM_KEY_R = crate::BitReader;
#[doc = "Field `DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE` reader - Set this bit to disable the USB-Serial-JTAG download function."]
pub type DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` reader - Set this bit to enable security download mode."]
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL` reader - Set the type of UART printing, 00: force enable printing, 01: enable printing when GPIO8 is reset at low level, 10: enable printing when GPIO8 is reset at high level, 11: force disable printing"]
pub type UART_PRINT_CONTROL_R = crate::FieldReader;
#[doc = "Field `FORCE_SEND_RESUME` reader - Set this bit to force ROM code to send a resume command during SPI boot."]
pub type FORCE_SEND_RESUME_R = crate::BitReader;
#[doc = "Field `SECURE_VERSION` reader - Secure version used by ESP-IDF anti-rollback feature."]
pub type SECURE_VERSION_R = crate::FieldReader<u16>;
#[doc = "Field `SECURE_BOOT_DISABLE_FAST_WAKE` reader - Represents whether secure boot do fast verification on wake is disabled. 0: enabled 1: disabled"]
pub type SECURE_BOOT_DISABLE_FAST_WAKE_R = crate::BitReader;
#[doc = "Field `HYS_EN_PAD` reader - Set bits to enable hysteresis function of PAD0~27"]
pub type HYS_EN_PAD_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0_H` reader - Purpose of Key0. The 5-th bit."]
pub type KEY_PURPOSE_0_H_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_1_H` reader - Purpose of Key1. The 5-th bit."]
pub type KEY_PURPOSE_1_H_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_2_H` reader - Purpose of Key2. The 5-th bit."]
pub type KEY_PURPOSE_2_H_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_3_H` reader - Purpose of Key3. The 5-th bit."]
pub type KEY_PURPOSE_3_H_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_4_H` reader - Purpose of Key4. The 5-th bit."]
pub type KEY_PURPOSE_4_H_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set this bit to disable download mode (boot_mode\\[3:0\\] = 0, 1, 2, 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to disable direct boot mode"]
    #[inline(always)]
    pub fn dis_direct_boot(&self) -> DIS_DIRECT_BOOT_R {
        DIS_DIRECT_BOOT_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to disable USB-Serial-JTAG print during rom boot."]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_rom_print(&self) -> DIS_USB_SERIAL_JTAG_ROM_PRINT_R {
        DIS_USB_SERIAL_JTAG_ROM_PRINT_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - set this bit to lock the key manager key after deploy"]
    #[inline(always)]
    pub fn lock_km_key(&self) -> LOCK_KM_KEY_R {
        LOCK_KM_KEY_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to disable the USB-Serial-JTAG download function."]
    #[inline(always)]
    pub fn dis_usb_serial_jtag_download_mode(&self) -> DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R {
        DIS_USB_SERIAL_JTAG_DOWNLOAD_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable security download mode."]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Set the type of UART printing, 00: force enable printing, 01: enable printing when GPIO8 is reset at low level, 10: enable printing when GPIO8 is reset at high level, 11: force disable printing"]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - Set this bit to force ROM code to send a resume command during SPI boot."]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:24 - Secure version used by ESP-IDF anti-rollback feature."]
    #[inline(always)]
    pub fn secure_version(&self) -> SECURE_VERSION_R {
        SECURE_VERSION_R::new(((self.bits >> 9) & 0xffff) as u16)
    }
    #[doc = "Bit 25 - Represents whether secure boot do fast verification on wake is disabled. 0: enabled 1: disabled"]
    #[inline(always)]
    pub fn secure_boot_disable_fast_wake(&self) -> SECURE_BOOT_DISABLE_FAST_WAKE_R {
        SECURE_BOOT_DISABLE_FAST_WAKE_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Set bits to enable hysteresis function of PAD0~27"]
    #[inline(always)]
    pub fn hys_en_pad(&self) -> HYS_EN_PAD_R {
        HYS_EN_PAD_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Purpose of Key0. The 5-th bit."]
    #[inline(always)]
    pub fn key_purpose_0_h(&self) -> KEY_PURPOSE_0_H_R {
        KEY_PURPOSE_0_H_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - Purpose of Key1. The 5-th bit."]
    #[inline(always)]
    pub fn key_purpose_1_h(&self) -> KEY_PURPOSE_1_H_R {
        KEY_PURPOSE_1_H_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Purpose of Key2. The 5-th bit."]
    #[inline(always)]
    pub fn key_purpose_2_h(&self) -> KEY_PURPOSE_2_H_R {
        KEY_PURPOSE_2_H_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Purpose of Key3. The 5-th bit."]
    #[inline(always)]
    pub fn key_purpose_3_h(&self) -> KEY_PURPOSE_3_H_R {
        KEY_PURPOSE_3_H_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Purpose of Key4. The 5-th bit."]
    #[inline(always)]
    pub fn key_purpose_4_h(&self) -> KEY_PURPOSE_4_H_R {
        KEY_PURPOSE_4_H_R::new(((self.bits >> 31) & 1) != 0)
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
            .field("lock_km_key", &self.lock_km_key())
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
            .field("hys_en_pad", &self.hys_en_pad())
            .field("key_purpose_0_h", &self.key_purpose_0_h())
            .field("key_purpose_1_h", &self.key_purpose_1_h())
            .field("key_purpose_2_h", &self.key_purpose_2_h())
            .field("key_purpose_3_h", &self.key_purpose_3_h())
            .field("key_purpose_4_h", &self.key_purpose_4_h())
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
