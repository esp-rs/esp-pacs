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
#[doc = "Field `VDD_SPI_DREFM_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_DREFM_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDD_SPI_DREFL_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_DREFL_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDD_SPI_XPD_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_XPD_ERR_R = crate::BitReader<bool>;
#[doc = "Field `VDD_SPI_TIEH_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_TIEH_ERR_R = crate::BitReader<bool>;
#[doc = "Field `VDD_SPI_FORCE_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_FORCE_ERR_R = crate::BitReader<bool>;
#[doc = "Field `VDD_SPI_EN_INIT_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_EN_INIT_ERR_R = crate::BitReader<bool>;
#[doc = "Field `VDD_SPI_ENCURLIM_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_ENCURLIM_ERR_R = crate::BitReader<bool>;
#[doc = "Field `VDD_SPI_DCURLIM_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_DCURLIM_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDD_SPI_INIT_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_INIT_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VDD_SPI_DCAP_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type VDD_SPI_DCAP_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `WDT_DELAY_SEL_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type WDT_DELAY_SEL_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_BOOT_CRYPT_CNT_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SPI_BOOT_CRYPT_CNT_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SECURE_BOOT_KEY_REVOKE0_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SECURE_BOOT_KEY_REVOKE1_ERR_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type SECURE_BOOT_KEY_REVOKE2_ERR_R = crate::BitReader<bool>;
#[doc = "Field `KEY_PURPOSE_0_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type KEY_PURPOSE_0_ERR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_1_ERR` reader - If any bits in this filed are 1, then it indicates a programming error."]
pub type KEY_PURPOSE_1_ERR_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:1 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_drefm_err(&self) -> VDD_SPI_DREFM_ERR_R {
        VDD_SPI_DREFM_ERR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_drefl_err(&self) -> VDD_SPI_DREFL_ERR_R {
        VDD_SPI_DREFL_ERR_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_xpd_err(&self) -> VDD_SPI_XPD_ERR_R {
        VDD_SPI_XPD_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_tieh_err(&self) -> VDD_SPI_TIEH_ERR_R {
        VDD_SPI_TIEH_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_force_err(&self) -> VDD_SPI_FORCE_ERR_R {
        VDD_SPI_FORCE_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_en_init_err(&self) -> VDD_SPI_EN_INIT_ERR_R {
        VDD_SPI_EN_INIT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_encurlim_err(&self) -> VDD_SPI_ENCURLIM_ERR_R {
        VDD_SPI_ENCURLIM_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_dcurlim_err(&self) -> VDD_SPI_DCURLIM_ERR_R {
        VDD_SPI_DCURLIM_ERR_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:13 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_init_err(&self) -> VDD_SPI_INIT_ERR_R {
        VDD_SPI_INIT_ERR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn vdd_spi_dcap_err(&self) -> VDD_SPI_DCAP_ERR_R {
        VDD_SPI_DCAP_ERR_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> WDT_DELAY_SEL_ERR_R {
        WDT_DELAY_SEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt_err(&self) -> SPI_BOOT_CRYPT_CNT_ERR_R {
        SPI_BOOT_CRYPT_CNT_ERR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0_err(&self) -> SECURE_BOOT_KEY_REVOKE0_ERR_R {
        SECURE_BOOT_KEY_REVOKE0_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1_err(&self) -> SECURE_BOOT_KEY_REVOKE1_ERR_R {
        SECURE_BOOT_KEY_REVOKE1_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2_err(&self) -> SECURE_BOOT_KEY_REVOKE2_ERR_R {
        SECURE_BOOT_KEY_REVOKE2_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> KEY_PURPOSE_0_ERR_R {
        KEY_PURPOSE_0_ERR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - If any bits in this filed are 1, then it indicates a programming error."]
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> KEY_PURPOSE_1_ERR_R {
        KEY_PURPOSE_1_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
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
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
