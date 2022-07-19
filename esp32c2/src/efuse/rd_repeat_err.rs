#[doc = "Register `RD_REPEAT_ERR` reader"]
pub struct R(crate::R<RD_REPEAT_ERR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RD_DIS_ERR` reader - If any bit in RD_DIS is 1, then it indicates a programming error."]
pub type RD_DIS_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_DELAY_SEL_ERR` reader - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error."]
pub type WDT_DELAY_SEL_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `DIS_PAD_JTAG_ERR` reader - If any bit in DIS_PAD_JTAG is 1, then it indicates a programming error."]
pub type DIS_PAD_JTAG_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DOWNLOAD_ICACHE_ERR` reader - If any bit in this filed is 1, then it indicates a programming error."]
pub type DIS_DOWNLOAD_ICACHE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR` reader - If any bit in DIS_DOWNLOAD_MANUAL_ENCRYPT is 1, then it indicates a programming error."]
pub type DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SPI_BOOT_ENCRYPT_DECRYPT_CNT_ERR` reader - If any bit in SPI_BOOT_ENCRYPT_DECRYPT_CNT is 1, then it indicates a programming error."]
pub type SPI_BOOT_ENCRYPT_DECRYPT_CNT_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `XTS_KEY_LENGTH_256_ERR` reader - If any bit in XTS_KEY_LENGTH_256 is 1, then it indicates a programming error."]
pub type XTS_KEY_LENGTH_256_ERR_R = crate::BitReader<bool>;
#[doc = "Field `UART_PRINT_CONTROL_ERR` reader - If any bit in UART_PRINT_CONTROL is 1, then it indicates a programming error."]
pub type UART_PRINT_CONTROL_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FORCE_SEND_RESUME_ERR` reader - If any bit in FORCE_SEND_RESUME is 1, then it indicates a programming error."]
pub type FORCE_SEND_RESUME_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - If any bit in this filed is 1, then it indicates a programming error."]
pub type DIS_DOWNLOAD_MODE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `DIS_DIRECT_BOOT_ERR` reader - If any bit in this filed is 1, then it indicates a programming error."]
pub type DIS_DIRECT_BOOT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `ENABLE_SECURITY_DOWNLOAD_ERR` reader - If any bit in this filed is 1, then it indicates a programming error."]
pub type ENABLE_SECURITY_DOWNLOAD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `FLASH_TPUW_ERR` reader - If any bit in this filed is 1, then it indicates a programming error."]
pub type FLASH_TPUW_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - If any bit in this filed is 1, then it indicates a programming error."]
pub type SECURE_BOOT_EN_ERR_R = crate::BitReader<bool>;
#[doc = "Field `RPT4_RESERVED_ERR` reader - Reserved."]
pub type RPT4_RESERVED_ERR_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:1 - If any bit in RD_DIS is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn rd_dis_err(&self) -> RD_DIS_ERR_R {
        RD_DIS_ERR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> WDT_DELAY_SEL_ERR_R {
        WDT_DELAY_SEL_ERR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - If any bit in DIS_PAD_JTAG is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_pad_jtag_err(&self) -> DIS_PAD_JTAG_ERR_R {
        DIS_PAD_JTAG_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If any bit in this filed is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_icache_err(&self) -> DIS_DOWNLOAD_ICACHE_ERR_R {
        DIS_DOWNLOAD_ICACHE_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If any bit in DIS_DOWNLOAD_MANUAL_ENCRYPT is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_manual_encrypt_err(&self) -> DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R {
        DIS_DOWNLOAD_MANUAL_ENCRYPT_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:9 - If any bit in SPI_BOOT_ENCRYPT_DECRYPT_CNT is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn spi_boot_encrypt_decrypt_cnt_err(&self) -> SPI_BOOT_ENCRYPT_DECRYPT_CNT_ERR_R {
        SPI_BOOT_ENCRYPT_DECRYPT_CNT_ERR_R::new(((self.bits >> 7) & 7) as u8)
    }
    #[doc = "Bit 10 - If any bit in XTS_KEY_LENGTH_256 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn xts_key_length_256_err(&self) -> XTS_KEY_LENGTH_256_ERR_R {
        XTS_KEY_LENGTH_256_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - If any bit in UART_PRINT_CONTROL is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn uart_print_control_err(&self) -> UART_PRINT_CONTROL_ERR_R {
        UART_PRINT_CONTROL_ERR_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - If any bit in FORCE_SEND_RESUME is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn force_send_resume_err(&self) -> FORCE_SEND_RESUME_ERR_R {
        FORCE_SEND_RESUME_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - If any bit in this filed is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - If any bit in this filed is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn dis_direct_boot_err(&self) -> DIS_DIRECT_BOOT_ERR_R {
        DIS_DIRECT_BOOT_ERR_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - If any bit in this filed is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn enable_security_download_err(&self) -> ENABLE_SECURITY_DOWNLOAD_ERR_R {
        ENABLE_SECURITY_DOWNLOAD_ERR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:20 - If any bit in this filed is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FLASH_TPUW_ERR_R {
        FLASH_TPUW_ERR_R::new(((self.bits >> 17) & 0x0f) as u8)
    }
    #[doc = "Bit 21 - If any bit in this filed is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SECURE_BOOT_EN_ERR_R {
        SECURE_BOOT_EN_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:31 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved_err(&self) -> RPT4_RESERVED_ERR_R {
        RPT4_RESERVED_ERR_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[doc = "Programming error record register 0 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err](index.html) module"]
pub struct RD_REPEAT_ERR_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR to value 0"]
impl crate::Resettable for RD_REPEAT_ERR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
