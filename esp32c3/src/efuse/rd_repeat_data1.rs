#[doc = "Register `RD_REPEAT_DATA1` reader"]
pub struct R(crate::R<RD_REPEAT_DATA1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_DATA1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_DATA1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_DATA1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RPT4_RESERVED2` reader - Reserved (used for four backups method)."]
pub type RPT4_RESERVED2_R = crate::FieldReader<u16, u16>;
#[doc = "Field `WDT_DELAY_SEL` reader - Selects RTC watchdog timeout threshold, in unit of slow clock cycle. 0: 40000. 1: 80000. 2: 160000. 3:320000."]
pub type WDT_DELAY_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - Set this bit to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
pub type SPI_BOOT_CRYPT_CNT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - Set this bit to enable revoking first secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - Set this bit to enable revoking second secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::BitReader<bool>;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - Set this bit to enable revoking third secure boot key."]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::BitReader<bool>;
#[doc = "Field `KEY_PURPOSE_0` reader - Purpose of Key0."]
pub type KEY_PURPOSE_0_R = crate::FieldReader<u8, u8>;
#[doc = "Field `KEY_PURPOSE_1` reader - Purpose of Key1."]
pub type KEY_PURPOSE_1_R = crate::FieldReader<u8, u8>;
impl R {
    #[doc = "Bits 0:15 - Reserved (used for four backups method)."]
    #[inline(always)]
    pub fn rpt4_reserved2(&self) -> RPT4_RESERVED2_R {
        RPT4_RESERVED2_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Selects RTC watchdog timeout threshold, in unit of slow clock cycle. 0: 40000. 1: 80000. 2: 160000. 3:320000."]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Set this bit to enable SPI boot encrypt/decrypt. Odd number of 1: enable. even number of 1: disable."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Set this bit to enable revoking first secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to enable revoking second secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to enable revoking third secure boot key."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Purpose of Key0."]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Purpose of Key1."]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[doc = "BLOCK0 data register 2.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_data1](index.html) module"]
pub struct RD_REPEAT_DATA1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_data1::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
