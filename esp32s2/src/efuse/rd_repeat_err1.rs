///Register `RD_REPEAT_ERR1` reader
pub type R = crate::R<RD_REPEAT_ERR1_SPEC>;
///Field `VDD_SPI_DREFM_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFM.
pub type VDD_SPI_DREFM_ERR_R = crate::FieldReader;
///Field `VDD_SPI_DREFL_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFL.
pub type VDD_SPI_DREFL_ERR_R = crate::FieldReader;
///Field `VDD_SPI_XPD_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_XPD.
pub type VDD_SPI_XPD_ERR_R = crate::BitReader;
///Field `VDD_SPI_TIEH_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_TIEH.
pub type VDD_SPI_TIEH_ERR_R = crate::BitReader;
///Field `VDD_SPI_FORCE_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_FORCE.
pub type VDD_SPI_FORCE_ERR_R = crate::BitReader;
///Field `VDD_SPI_EN_INIT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_EN_INIT.
pub type VDD_SPI_EN_INIT_ERR_R = crate::BitReader;
///Field `VDD_SPI_ENCURLIM_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_ENCURLIM.
pub type VDD_SPI_ENCURLIM_ERR_R = crate::BitReader;
///Field `VDD_SPI_DCURLIM_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DCURLIM.
pub type VDD_SPI_DCURLIM_ERR_R = crate::FieldReader;
///Field `VDD_SPI_INIT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_INIT.
pub type VDD_SPI_INIT_ERR_R = crate::FieldReader;
///Field `VDD_SPI_DCAP_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DCAP.
pub type VDD_SPI_DCAP_ERR_R = crate::FieldReader;
///Field `WDT_DELAY_SEL_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_WDT_DELAY_SEL.
pub type WDT_DELAY_SEL_ERR_R = crate::FieldReader;
///Field `SPI_BOOT_CRYPT_CNT_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_SPI_BOOT_CRYPT_CNT.
pub type SPI_BOOT_CRYPT_CNT_ERR_R = crate::FieldReader;
///Field `SECURE_BOOT_KEY_REVOKE0_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_BOOT_KEY_REVOKE0.
pub type SECURE_BOOT_KEY_REVOKE0_ERR_R = crate::BitReader;
///Field `SECURE_BOOT_KEY_REVOKE1_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_BOOT_KEY_REVOKE1.
pub type SECURE_BOOT_KEY_REVOKE1_ERR_R = crate::BitReader;
///Field `SECURE_BOOT_KEY_REVOKE2_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_BOOT_KEY_REVOKE2.
pub type SECURE_BOOT_KEY_REVOKE2_ERR_R = crate::BitReader;
///Field `KEY_PURPOSE_0_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_KEY_PURPOSE_0.
pub type KEY_PURPOSE_0_ERR_R = crate::FieldReader;
///Field `KEY_PURPOSE_1_ERR` reader - Any bit equal to 1 denotes a programming error in EFUSE_KEY_PURPOSE_1.
pub type KEY_PURPOSE_1_ERR_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFM.
    #[inline(always)]
    pub fn vdd_spi_drefm_err(&self) -> VDD_SPI_DREFM_ERR_R {
        VDD_SPI_DREFM_ERR_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:3 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DREFL.
    #[inline(always)]
    pub fn vdd_spi_drefl_err(&self) -> VDD_SPI_DREFL_ERR_R {
        VDD_SPI_DREFL_ERR_R::new(((self.bits >> 2) & 3) as u8)
    }
    ///Bit 4 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_XPD.
    #[inline(always)]
    pub fn vdd_spi_xpd_err(&self) -> VDD_SPI_XPD_ERR_R {
        VDD_SPI_XPD_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    ///Bit 5 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_TIEH.
    #[inline(always)]
    pub fn vdd_spi_tieh_err(&self) -> VDD_SPI_TIEH_ERR_R {
        VDD_SPI_TIEH_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_FORCE.
    #[inline(always)]
    pub fn vdd_spi_force_err(&self) -> VDD_SPI_FORCE_ERR_R {
        VDD_SPI_FORCE_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    ///Bit 7 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_EN_INIT.
    #[inline(always)]
    pub fn vdd_spi_en_init_err(&self) -> VDD_SPI_EN_INIT_ERR_R {
        VDD_SPI_EN_INIT_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    ///Bit 8 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_ENCURLIM.
    #[inline(always)]
    pub fn vdd_spi_encurlim_err(&self) -> VDD_SPI_ENCURLIM_ERR_R {
        VDD_SPI_ENCURLIM_ERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    ///Bits 9:11 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DCURLIM.
    #[inline(always)]
    pub fn vdd_spi_dcurlim_err(&self) -> VDD_SPI_DCURLIM_ERR_R {
        VDD_SPI_DCURLIM_ERR_R::new(((self.bits >> 9) & 7) as u8)
    }
    ///Bits 12:13 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_INIT.
    #[inline(always)]
    pub fn vdd_spi_init_err(&self) -> VDD_SPI_INIT_ERR_R {
        VDD_SPI_INIT_ERR_R::new(((self.bits >> 12) & 3) as u8)
    }
    ///Bits 14:15 - Any bit equal to 1 denotes a programming error in EFUSE_VDD_SPI_DCAP.
    #[inline(always)]
    pub fn vdd_spi_dcap_err(&self) -> VDD_SPI_DCAP_ERR_R {
        VDD_SPI_DCAP_ERR_R::new(((self.bits >> 14) & 3) as u8)
    }
    ///Bits 16:17 - Any bit equal to 1 denotes a programming error in EFUSE_WDT_DELAY_SEL.
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> WDT_DELAY_SEL_ERR_R {
        WDT_DELAY_SEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    ///Bits 18:20 - Any bit equal to 1 denotes a programming error in EFUSE_SPI_BOOT_CRYPT_CNT.
    #[inline(always)]
    pub fn spi_boot_crypt_cnt_err(&self) -> SPI_BOOT_CRYPT_CNT_ERR_R {
        SPI_BOOT_CRYPT_CNT_ERR_R::new(((self.bits >> 18) & 7) as u8)
    }
    ///Bit 21 - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_BOOT_KEY_REVOKE0.
    #[inline(always)]
    pub fn secure_boot_key_revoke0_err(&self) -> SECURE_BOOT_KEY_REVOKE0_ERR_R {
        SECURE_BOOT_KEY_REVOKE0_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    ///Bit 22 - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_BOOT_KEY_REVOKE1.
    #[inline(always)]
    pub fn secure_boot_key_revoke1_err(&self) -> SECURE_BOOT_KEY_REVOKE1_ERR_R {
        SECURE_BOOT_KEY_REVOKE1_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    ///Bit 23 - Any bit equal to 1 denotes a programming error in EFUSE_SECURE_BOOT_KEY_REVOKE2.
    #[inline(always)]
    pub fn secure_boot_key_revoke2_err(&self) -> SECURE_BOOT_KEY_REVOKE2_ERR_R {
        SECURE_BOOT_KEY_REVOKE2_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    ///Bits 24:27 - Any bit equal to 1 denotes a programming error in EFUSE_KEY_PURPOSE_0.
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> KEY_PURPOSE_0_ERR_R {
        KEY_PURPOSE_0_ERR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    ///Bits 28:31 - Any bit equal to 1 denotes a programming error in EFUSE_KEY_PURPOSE_1.
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> KEY_PURPOSE_1_ERR_R {
        KEY_PURPOSE_1_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR1")
            .field("vdd_spi_drefm_err", &self.vdd_spi_drefm_err())
            .field("vdd_spi_drefl_err", &self.vdd_spi_drefl_err())
            .field("vdd_spi_xpd_err", &self.vdd_spi_xpd_err())
            .field("vdd_spi_tieh_err", &self.vdd_spi_tieh_err())
            .field("vdd_spi_force_err", &self.vdd_spi_force_err())
            .field("vdd_spi_en_init_err", &self.vdd_spi_en_init_err())
            .field("vdd_spi_encurlim_err", &self.vdd_spi_encurlim_err())
            .field("vdd_spi_dcurlim_err", &self.vdd_spi_dcurlim_err())
            .field("vdd_spi_init_err", &self.vdd_spi_init_err())
            .field("vdd_spi_dcap_err", &self.vdd_spi_dcap_err())
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
