#[doc = "Register `RD_REPEAT_DATA0` reader"]
pub struct R(crate::R<RD_REPEAT_DATA0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_DIS` reader - The bit be set to disable software read high/low 128-bit of BLK3."]
pub type RD_DIS_R = crate::FieldReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Selects RTC watchdog timeout threshold, in unit of slow clock cycle. 0: 40000. 1: 80000. 2: 160000. 3:320000."]
pub type WDT_DELAY_SEL_R = crate::FieldReader;
#[doc = "Field `DIS_PAD_JTAG` reader - Set this bit to disable pad jtag."]
pub type DIS_PAD_JTAG_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_ICACHE` reader - The bit be set to disable icache in download mode."]
pub type DIS_DOWNLOAD_ICACHE_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT` reader - The bit be set to disable manual encryption."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_R = crate::BitReader;
#[doc = "Field `SPI_BOOT_ENCRYPT_DECRYPT_CNT` reader - These bits be set to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
pub type SPI_BOOT_ENCRYPT_DECRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `XTS_KEY_LENGTH_256` reader - The bit be set means XTS_AES use the whole 256-bit efuse data in BLOCK3. Otherwise, XTS_AES use 128-bit eFuse data in BLOCK3."]
pub type XTS_KEY_LENGTH_256_R = crate::BitReader;
#[doc = "Field `UART_PRINT_CONTROL` reader - Set this bit to disable usb printing."]
pub type UART_PRINT_CONTROL_R = crate::FieldReader;
#[doc = "Field `FORCE_SEND_RESUME` reader - Set this bit to force ROM code to send a resume command during SPI boot."]
pub type FORCE_SEND_RESUME_R = crate::BitReader;
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - Set this bit to disable download mode (boot_mode\\[3:0\\] = 0, 1, 2, 4, 5, 6, 7)."]
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
#[doc = "Field `DIS_DIRECT_BOOT` reader - This bit set means disable direct_boot mode."]
pub type DIS_DIRECT_BOOT_R = crate::BitReader;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD` reader - Set this bit to enable secure UART download mode."]
pub type ENABLE_SECURITY_DOWNLOAD_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW` reader - Configures flash waiting time after power-up, in unit of ms. If the value is less than 15, the waiting time is the configurable value. Otherwise, the waiting time is twice the configurable value."]
pub type FLASH_TPUW_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_EN` reader - The bit be set to enable secure boot."]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `RPT4_RESERVED` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:1 - The bit be set to disable software read high/low 128-bit of BLK3."]
    #[inline(always)]
    pub fn rd_dis(&self) -> RD_DIS_R {
        RD_DIS_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Selects RTC watchdog timeout threshold, in unit of slow clock cycle. 0: 40000. 1: 80000. 2: 160000. 3:320000."]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Set this bit to disable pad jtag."]
    #[inline(always)]
    pub fn dis_pad_jtag(&self) -> DIS_PAD_JTAG_R {
        DIS_PAD_JTAG_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The bit be set to disable icache in download mode."]
    #[inline(always)]
    pub fn dis_download_icache(&self) -> DIS_DOWNLOAD_ICACHE_R {
        DIS_DOWNLOAD_ICACHE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The bit be set to disable manual encryption."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - These bits be set to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
    #[inline(always)]
    pub fn spi_boot_encrypt_decrypt_cnt(&self) -> SPI_BOOT_ENCRYPT_DECRYPT_CNT_R {
        SPI_BOOT_ENCRYPT_DECRYPT_CNT_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - The bit be set means XTS_AES use the whole 256-bit efuse data in BLOCK3. Otherwise, XTS_AES use 128-bit eFuse data in BLOCK3."]
    #[inline(always)]
    pub fn xts_key_length_256(&self) -> XTS_KEY_LENGTH_256_R {
        XTS_KEY_LENGTH_256_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Set this bit to disable usb printing."]
    #[inline(always)]
    pub fn uart_print_control(&self) -> UART_PRINT_CONTROL_R {
        UART_PRINT_CONTROL_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Set this bit to force ROM code to send a resume command during SPI boot."]
    #[inline(always)]
    pub fn force_send_resume(&self) -> FORCE_SEND_RESUME_R {
        FORCE_SEND_RESUME_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to disable download mode (boot_mode\\[3:0\\] = 0, 1, 2, 4, 5, 6, 7)."]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - This bit set means disable direct_boot mode."]
    #[inline(always)]
    pub fn dis_direct_boot(&self) -> DIS_DIRECT_BOOT_R {
        DIS_DIRECT_BOOT_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable secure UART download mode."]
    #[inline(always)]
    pub fn enable_security_download(&self) -> ENABLE_SECURITY_DOWNLOAD_R {
        ENABLE_SECURITY_DOWNLOAD_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - Configures flash waiting time after power-up, in unit of ms. If the value is less than 15, the waiting time is the configurable value. Otherwise, the waiting time is twice the configurable value."]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - The bit be set to enable secure boot."]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved(&self) -> RPT4_RESERVED_R {
        RPT4_RESERVED_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA0")
            .field("rd_dis", &format_args!("{}", self.rd_dis().bits()))
            .field(
                "wdt_delay_sel",
                &format_args!("{}", self.wdt_delay_sel().bits()),
            )
            .field(
                "dis_pad_jtag",
                &format_args!("{}", self.dis_pad_jtag().bit()),
            )
            .field(
                "dis_download_icache",
                &format_args!("{}", self.dis_download_icache().bit()),
            )
            .field(
                "dis_download_manual_encrypt",
                &format_args!("{}", self.dis_download_manual_encrypt().bit()),
            )
            .field(
                "spi_boot_encrypt_decrypt_cnt",
                &format_args!("{}", self.spi_boot_encrypt_decrypt_cnt().bits()),
            )
            .field(
                "xts_key_length_256",
                &format_args!("{}", self.xts_key_length_256().bit()),
            )
            .field(
                "uart_print_control",
                &format_args!("{}", self.uart_print_control().bits()),
            )
            .field(
                "force_send_resume",
                &format_args!("{}", self.force_send_resume().bit()),
            )
            .field(
                "dis_download_mode",
                &format_args!("{}", self.dis_download_mode().bit()),
            )
            .field(
                "dis_direct_boot",
                &format_args!("{}", self.dis_direct_boot().bit()),
            )
            .field(
                "enable_security_download",
                &format_args!("{}", self.enable_security_download().bit()),
            )
            .field("flash_tpuw", &format_args!("{}", self.flash_tpuw().bits()))
            .field(
                "secure_boot_en",
                &format_args!("{}", self.secure_boot_en().bit()),
            )
            .field(
                "rpt4_reserved",
                &format_args!("{}", self.rpt4_reserved().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_DATA0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "BLOCK0 data register 1.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data0](index.html) module"]
pub struct RD_REPEAT_DATA0_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data0::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA0 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
