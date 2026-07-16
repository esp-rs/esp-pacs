#[doc = "Register `RD_REPEAT_DATA_ERR6` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR6_SPEC>;
#[doc = "Field `RMA_SESSION_COUNTER_ERR` reader - Represents the programming error of EFUSE_RMA_SESSION_COUNTER"]
pub type RMA_SESSION_COUNTER_ERR_R = crate::FieldReader;
#[doc = "Field `RMA_NONCE_ENA_ERR` reader - Represents the programming error of EFUSE_RMA_NONCE_ENA"]
pub type RMA_NONCE_ENA_ERR_R = crate::FieldReader;
#[doc = "Field `RMA_CHIP_INFO_SOURCE_ERR` reader - Represents the programming error of EFUSE_RMA_CHIP_INFO_SOURCE"]
pub type RMA_CHIP_INFO_SOURCE_ERR_R = crate::BitReader;
#[doc = "Field `RMA_DISABLE_FAST_VEF_ERR` reader - Represents the programming error of EFUSE_RMA_DISABLE_FAST_VEF"]
pub type RMA_DISABLE_FAST_VEF_ERR_R = crate::BitReader;
#[doc = "Field `PVT_0_GLITCH_EN_ERR` reader - Represents the programming error of EFUSE_PVT_0_GLITCH_EN"]
pub type PVT_0_GLITCH_EN_ERR_R = crate::BitReader;
#[doc = "Field `PVT_0_GLITCH_MODE_ERR` reader - Represents the programming error of EFUSE_PVT_0_GLITCH_MODE"]
pub type PVT_0_GLITCH_MODE_ERR_R = crate::FieldReader;
#[doc = "Field `PVT_1_GLITCH_EN_ERR` reader - Represents the programming error of EFUSE_PVT_1_GLITCH_EN"]
pub type PVT_1_GLITCH_EN_ERR_R = crate::BitReader;
#[doc = "Field `PVT_1_GLITCH_MODE_ERR` reader - Represents the programming error of EFUSE_PVT_1_GLITCH_MODE"]
pub type PVT_1_GLITCH_MODE_ERR_R = crate::FieldReader;
#[doc = "Field `PMU_FLASH_POWER_SEL_ERR` reader - Represents the programming error of EFUSE_PMU_FLASH_POWER_SEL"]
pub type PMU_FLASH_POWER_SEL_ERR_R = crate::BitReader;
#[doc = "Field `PMU_FLASH_POWER_SEL_EN_ERR` reader - Represents the programming error of EFUSE_PMU_FLASH_POWER_SEL_EN"]
pub type PMU_FLASH_POWER_SEL_EN_ERR_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_EN_ERR` reader - Represents the programming error of EFUSE_POWER_GLITCH_EN"]
pub type POWER_GLITCH_EN_ERR_R = crate::FieldReader;
#[doc = "Field `ENA_XTS_SHADOW_ERR` reader - Represents the programming error of EFUSE_ENA_XTS_SHADOW"]
pub type ENA_XTS_SHADOW_ERR_R = crate::BitReader;
#[doc = "Field `ENA_SPI_BOOT_CRYPT_SCRAMBLER_ERR` reader - Represents the programming error of EFUSE_ENA_SPI_BOOT_CRYPT_SCRAMBLER"]
pub type ENA_SPI_BOOT_CRYPT_SCRAMBLER_ERR_R = crate::BitReader;
#[doc = "Field `RE_ENABLE_JTAG_SOURCE_ERR` reader - Represents the programming error of EFUSE_RE_ENABLE_JTAG_SOURCE"]
pub type RE_ENABLE_JTAG_SOURCE_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Represents the programming error of EFUSE_RMA_SESSION_COUNTER"]
    #[inline(always)]
    pub fn rma_session_counter_err(&self) -> RMA_SESSION_COUNTER_ERR_R {
        RMA_SESSION_COUNTER_ERR_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Represents the programming error of EFUSE_RMA_NONCE_ENA"]
    #[inline(always)]
    pub fn rma_nonce_ena_err(&self) -> RMA_NONCE_ENA_ERR_R {
        RMA_NONCE_ENA_ERR_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Represents the programming error of EFUSE_RMA_CHIP_INFO_SOURCE"]
    #[inline(always)]
    pub fn rma_chip_info_source_err(&self) -> RMA_CHIP_INFO_SOURCE_ERR_R {
        RMA_CHIP_INFO_SOURCE_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents the programming error of EFUSE_RMA_DISABLE_FAST_VEF"]
    #[inline(always)]
    pub fn rma_disable_fast_vef_err(&self) -> RMA_DISABLE_FAST_VEF_ERR_R {
        RMA_DISABLE_FAST_VEF_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents the programming error of EFUSE_PVT_0_GLITCH_EN"]
    #[inline(always)]
    pub fn pvt_0_glitch_en_err(&self) -> PVT_0_GLITCH_EN_ERR_R {
        PVT_0_GLITCH_EN_ERR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Represents the programming error of EFUSE_PVT_0_GLITCH_MODE"]
    #[inline(always)]
    pub fn pvt_0_glitch_mode_err(&self) -> PVT_0_GLITCH_MODE_ERR_R {
        PVT_0_GLITCH_MODE_ERR_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Represents the programming error of EFUSE_PVT_1_GLITCH_EN"]
    #[inline(always)]
    pub fn pvt_1_glitch_en_err(&self) -> PVT_1_GLITCH_EN_ERR_R {
        PVT_1_GLITCH_EN_ERR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Represents the programming error of EFUSE_PVT_1_GLITCH_MODE"]
    #[inline(always)]
    pub fn pvt_1_glitch_mode_err(&self) -> PVT_1_GLITCH_MODE_ERR_R {
        PVT_1_GLITCH_MODE_ERR_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - Represents the programming error of EFUSE_PMU_FLASH_POWER_SEL"]
    #[inline(always)]
    pub fn pmu_flash_power_sel_err(&self) -> PMU_FLASH_POWER_SEL_ERR_R {
        PMU_FLASH_POWER_SEL_ERR_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Represents the programming error of EFUSE_PMU_FLASH_POWER_SEL_EN"]
    #[inline(always)]
    pub fn pmu_flash_power_sel_en_err(&self) -> PMU_FLASH_POWER_SEL_EN_ERR_R {
        PMU_FLASH_POWER_SEL_EN_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - Represents the programming error of EFUSE_POWER_GLITCH_EN"]
    #[inline(always)]
    pub fn power_glitch_en_err(&self) -> POWER_GLITCH_EN_ERR_R {
        POWER_GLITCH_EN_ERR_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - Represents the programming error of EFUSE_ENA_XTS_SHADOW"]
    #[inline(always)]
    pub fn ena_xts_shadow_err(&self) -> ENA_XTS_SHADOW_ERR_R {
        ENA_XTS_SHADOW_ERR_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents the programming error of EFUSE_ENA_SPI_BOOT_CRYPT_SCRAMBLER"]
    #[inline(always)]
    pub fn ena_spi_boot_crypt_scrambler_err(&self) -> ENA_SPI_BOOT_CRYPT_SCRAMBLER_ERR_R {
        ENA_SPI_BOOT_CRYPT_SCRAMBLER_ERR_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents the programming error of EFUSE_RE_ENABLE_JTAG_SOURCE"]
    #[inline(always)]
    pub fn re_enable_jtag_source_err(&self) -> RE_ENABLE_JTAG_SOURCE_ERR_R {
        RE_ENABLE_JTAG_SOURCE_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR6")
            .field("rma_session_counter_err", &self.rma_session_counter_err())
            .field("rma_nonce_ena_err", &self.rma_nonce_ena_err())
            .field("rma_chip_info_source_err", &self.rma_chip_info_source_err())
            .field("rma_disable_fast_vef_err", &self.rma_disable_fast_vef_err())
            .field("pvt_0_glitch_en_err", &self.pvt_0_glitch_en_err())
            .field("pvt_0_glitch_mode_err", &self.pvt_0_glitch_mode_err())
            .field("pvt_1_glitch_en_err", &self.pvt_1_glitch_en_err())
            .field("pvt_1_glitch_mode_err", &self.pvt_1_glitch_mode_err())
            .field("pmu_flash_power_sel_err", &self.pmu_flash_power_sel_err())
            .field(
                "pmu_flash_power_sel_en_err",
                &self.pmu_flash_power_sel_en_err(),
            )
            .field("power_glitch_en_err", &self.power_glitch_en_err())
            .field("ena_xts_shadow_err", &self.ena_xts_shadow_err())
            .field(
                "ena_spi_boot_crypt_scrambler_err",
                &self.ena_spi_boot_crypt_scrambler_err(),
            )
            .field(
                "re_enable_jtag_source_err",
                &self.re_enable_jtag_source_err(),
            )
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR6_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err6::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR6_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR6 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR6_SPEC {}
