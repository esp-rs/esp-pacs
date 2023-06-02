#[doc = "Register `RD_REPEAT_ERR1` reader"]
pub struct R(crate::R<RD_REPEAT_ERR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED2_ERR` reader - Reserved."]
pub type RPT4_RESERVED2_ERR_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_DELAY_SEL_ERR` reader - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error."]
pub type WDT_DELAY_SEL_ERR_R = crate::FieldReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT_ERR` reader - If any bit in SPI_BOOT_CRYPT_CNT is 1, then it indicates a programming error."]
pub type SPI_BOOT_CRYPT_CNT_ERR_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0_ERR` reader - If SECURE_BOOT_KEY_REVOKE0 is 1, then it indicates a programming error."]
pub type SECURE_BOOT_KEY_REVOKE0_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1_ERR` reader - If SECURE_BOOT_KEY_REVOKE1 is 1, then it indicates a programming error."]
pub type SECURE_BOOT_KEY_REVOKE1_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2_ERR` reader - If SECURE_BOOT_KEY_REVOKE2 is 1, then it indicates a programming error."]
pub type SECURE_BOOT_KEY_REVOKE2_ERR_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0_ERR` reader - If any bit in KEY_PURPOSE_0 is 1, then it indicates a programming error."]
pub type KEY_PURPOSE_0_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1_ERR` reader - If any bit in KEY_PURPOSE_1 is 1, then it indicates a programming error."]
pub type KEY_PURPOSE_1_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved2_err(&self) -> RPT4_RESERVED2_ERR_R {
        RPT4_RESERVED2_ERR_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> WDT_DELAY_SEL_ERR_R {
        WDT_DELAY_SEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - If any bit in SPI_BOOT_CRYPT_CNT is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt_err(&self) -> SPI_BOOT_CRYPT_CNT_ERR_R {
        SPI_BOOT_CRYPT_CNT_ERR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - If SECURE_BOOT_KEY_REVOKE0 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0_err(&self) -> SECURE_BOOT_KEY_REVOKE0_ERR_R {
        SECURE_BOOT_KEY_REVOKE0_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - If SECURE_BOOT_KEY_REVOKE1 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1_err(&self) -> SECURE_BOOT_KEY_REVOKE1_ERR_R {
        SECURE_BOOT_KEY_REVOKE1_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If SECURE_BOOT_KEY_REVOKE2 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2_err(&self) -> SECURE_BOOT_KEY_REVOKE2_ERR_R {
        SECURE_BOOT_KEY_REVOKE2_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - If any bit in KEY_PURPOSE_0 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> KEY_PURPOSE_0_ERR_R {
        KEY_PURPOSE_0_ERR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - If any bit in KEY_PURPOSE_1 is 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> KEY_PURPOSE_1_ERR_R {
        KEY_PURPOSE_1_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR1")
            .field(
                "rpt4_reserved2_err",
                &format_args!("{}", self.rpt4_reserved2_err().bits()),
            )
            .field(
                "wdt_delay_sel_err",
                &format_args!("{}", self.wdt_delay_sel_err().bits()),
            )
            .field(
                "spi_boot_crypt_cnt_err",
                &format_args!("{}", self.spi_boot_crypt_cnt_err().bits()),
            )
            .field(
                "secure_boot_key_revoke0_err",
                &format_args!("{}", self.secure_boot_key_revoke0_err().bit()),
            )
            .field(
                "secure_boot_key_revoke1_err",
                &format_args!("{}", self.secure_boot_key_revoke1_err().bit()),
            )
            .field(
                "secure_boot_key_revoke2_err",
                &format_args!("{}", self.secure_boot_key_revoke2_err().bit()),
            )
            .field(
                "key_purpose_0_err",
                &format_args!("{}", self.key_purpose_0_err().bits()),
            )
            .field(
                "key_purpose_1_err",
                &format_args!("{}", self.key_purpose_1_err().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_ERR1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 1 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err1](index.html) module"]
pub struct RD_REPEAT_ERR1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err1::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR1 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
