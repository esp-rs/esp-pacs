///Register `RD_REPEAT_ERR1` reader
pub type R = crate::R<RD_REPEAT_ERR1_SPEC>;
///Field `RPT4_RESERVED2_ERR` reader - Reserved.
pub type RPT4_RESERVED2_ERR_R = crate::FieldReader<u16>;
///Field `WDT_DELAY_SEL_ERR` reader - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error.
pub type WDT_DELAY_SEL_ERR_R = crate::FieldReader;
///Field `SPI_BOOT_CRYPT_CNT_ERR` reader - If any bit in SPI_BOOT_CRYPT_CNT is 1, then it indicates a programming error.
pub type SPI_BOOT_CRYPT_CNT_ERR_R = crate::FieldReader;
///Field `SECURE_BOOT_KEY_REVOKE0_ERR` reader - If SECURE_BOOT_KEY_REVOKE0 is 1, then it indicates a programming error.
pub type SECURE_BOOT_KEY_REVOKE0_ERR_R = crate::BitReader;
///Field `SECURE_BOOT_KEY_REVOKE1_ERR` reader - If SECURE_BOOT_KEY_REVOKE1 is 1, then it indicates a programming error.
pub type SECURE_BOOT_KEY_REVOKE1_ERR_R = crate::BitReader;
///Field `SECURE_BOOT_KEY_REVOKE2_ERR` reader - If SECURE_BOOT_KEY_REVOKE2 is 1, then it indicates a programming error.
pub type SECURE_BOOT_KEY_REVOKE2_ERR_R = crate::BitReader;
///Field `KEY_PURPOSE_0_ERR` reader - If any bit in KEY_PURPOSE_0 is 1, then it indicates a programming error.
pub type KEY_PURPOSE_0_ERR_R = crate::FieldReader;
///Field `KEY_PURPOSE_1_ERR` reader - If any bit in KEY_PURPOSE_1 is 1, then it indicates a programming error.
pub type KEY_PURPOSE_1_ERR_R = crate::FieldReader;
impl R {
    ///Bits 0:15 - Reserved.
    #[inline(always)]
    pub fn rpt4_reserved2_err(&self) -> RPT4_RESERVED2_ERR_R {
        RPT4_RESERVED2_ERR_R::new((self.bits & 0xffff) as u16)
    }
    ///Bits 16:17 - If any bit in WDT_DELAY_SEL is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> WDT_DELAY_SEL_ERR_R {
        WDT_DELAY_SEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - If any bit in SPI_BOOT_CRYPT_CNT is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn spi_boot_crypt_cnt_err(&self) -> SPI_BOOT_CRYPT_CNT_ERR_R {
        SPI_BOOT_CRYPT_CNT_ERR_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21 - If SECURE_BOOT_KEY_REVOKE0 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn secure_boot_key_revoke0_err(&self) -> SECURE_BOOT_KEY_REVOKE0_ERR_R {
        SECURE_BOOT_KEY_REVOKE0_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - If SECURE_BOOT_KEY_REVOKE1 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn secure_boot_key_revoke1_err(&self) -> SECURE_BOOT_KEY_REVOKE1_ERR_R {
        SECURE_BOOT_KEY_REVOKE1_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - If SECURE_BOOT_KEY_REVOKE2 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn secure_boot_key_revoke2_err(&self) -> SECURE_BOOT_KEY_REVOKE2_ERR_R {
        SECURE_BOOT_KEY_REVOKE2_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:27 - If any bit in KEY_PURPOSE_0 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> KEY_PURPOSE_0_ERR_R {
        KEY_PURPOSE_0_ERR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - If any bit in KEY_PURPOSE_1 is 1, then it indicates a programming error.
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> KEY_PURPOSE_1_ERR_R {
        KEY_PURPOSE_1_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR1")
            .field("rpt4_reserved2_err", &self.rpt4_reserved2_err())
            .field("wdt_delay_sel_err", &self.wdt_delay_sel_err())
            .field("spi_boot_crypt_cnt_err", &self.spi_boot_crypt_cnt_err())
            .field(
                "secure_boot_key_revoke0_err",
                &self.secure_boot_key_revoke0_err(),
            )
            .field(
                "secure_boot_key_revoke1_err",
                &self.secure_boot_key_revoke1_err(),
            )
            .field(
                "secure_boot_key_revoke2_err",
                &self.secure_boot_key_revoke2_err(),
            )
            .field("key_purpose_0_err", &self.key_purpose_0_err())
            .field("key_purpose_1_err", &self.key_purpose_1_err())
            .finish()
    }
}
/**Programming error record register 1 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_REPEAT_ERR1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_repeat_err1::R`](R) reader structure
impl crate::Readable for RD_REPEAT_ERR1_SPEC {}
///`reset()` method sets RD_REPEAT_ERR1 to value 0
impl crate::Resettable for RD_REPEAT_ERR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
