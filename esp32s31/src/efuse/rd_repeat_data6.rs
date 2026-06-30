#[doc = "Register `RD_REPEAT_DATA6` reader"]
pub type R = crate::R<RD_REPEAT_DATA6_SPEC>;
#[doc = "Field `RMA_SESSION_COUNTER` reader - Represents the number of times the RMA session has been entered."]
pub type RMA_SESSION_COUNTER_R = crate::FieldReader;
#[doc = "Field `RMA_NONCE_ENA` reader - Represents whether random number NONCE is used in RMA and whether the KM module is used to generate the NONCE\\\\. 2'bx0: No NONCE\\\\ 2'b1x: Use KM generate NONCE.\\\\"]
pub type RMA_NONCE_ENA_R = crate::FieldReader;
#[doc = "Field `RMA_CHIP_INFO_SOURCE` reader - Represents whether HUK_info is selected as the source for calculating CHIP_info in RMA.\\\\1: use HUK_info\\\\ 0: use UNIQ_id\\\\"]
pub type RMA_CHIP_INFO_SOURCE_R = crate::BitReader;
#[doc = "Field `RMA_DISABLE_FAST_VEF` reader - Represents whether disable FAST_VEF in RMA session.\\\\1: disable\\\\0: enable\\\\"]
pub type RMA_DISABLE_FAST_VEF_R = crate::BitReader;
#[doc = "Field `PVT_0_GLITCH_EN` reader - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
pub type PVT_0_GLITCH_EN_R = crate::BitReader;
#[doc = "Field `PVT_0_GLITCH_MODE` reader - Use to configure glitch mode"]
pub type PVT_0_GLITCH_MODE_R = crate::FieldReader;
#[doc = "Field `PVT_1_GLITCH_EN` reader - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
pub type PVT_1_GLITCH_EN_R = crate::BitReader;
#[doc = "Field `PVT_1_GLITCH_MODE` reader - Use to configure glitch mode"]
pub type PVT_1_GLITCH_MODE_R = crate::FieldReader;
#[doc = "Field `PMU_FLASH_POWER_SEL` reader - FLASH power select.\\\\ 1'b1: use 3.3V\\\\1'b0: use 1.8V"]
pub type PMU_FLASH_POWER_SEL_R = crate::BitReader;
#[doc = "Field `PMU_FLASH_POWER_SEL_EN` reader - FLASH power select enable signal. 1'b1 : validates EFUSE_PMU_FLASH_POWER_SEL 1'b0: invalidates EFUSE_PMU_FLASH_POWER_SEL"]
pub type PMU_FLASH_POWER_SEL_EN_R = crate::BitReader;
#[doc = "Field `POWER_GLITCH_EN` reader - set these bit enable power glitch enable"]
pub type POWER_GLITCH_EN_R = crate::FieldReader;
#[doc = "Field `ENA_XTS_SHADOW` reader - Represents whether to enable XTS-AES shadow core countermeasure against fault injection attacks. \\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
pub type ENA_XTS_SHADOW_R = crate::BitReader;
#[doc = "Field `ENA_SPI_BOOT_CRYPT_SCRAMBLER` reader - Represents whether to enable ciphertext scrambler for external memory . \\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
pub type ENA_SPI_BOOT_CRYPT_SCRAMBLER_R = crate::BitReader;
#[doc = "Field `RE_ENABLE_JTAG_SOURCE` reader - Represents which Crypto peripheral is selected for renabling JTAG. \\\\ 0: RMA\\\\ 1: HMAC\\\\"]
pub type RE_ENABLE_JTAG_SOURCE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:2 - Represents the number of times the RMA session has been entered."]
    #[inline(always)]
    pub fn rma_session_counter(&self) -> RMA_SESSION_COUNTER_R {
        RMA_SESSION_COUNTER_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Represents whether random number NONCE is used in RMA and whether the KM module is used to generate the NONCE\\\\. 2'bx0: No NONCE\\\\ 2'b1x: Use KM generate NONCE.\\\\"]
    #[inline(always)]
    pub fn rma_nonce_ena(&self) -> RMA_NONCE_ENA_R {
        RMA_NONCE_ENA_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bit 5 - Represents whether HUK_info is selected as the source for calculating CHIP_info in RMA.\\\\1: use HUK_info\\\\ 0: use UNIQ_id\\\\"]
    #[inline(always)]
    pub fn rma_chip_info_source(&self) -> RMA_CHIP_INFO_SOURCE_R {
        RMA_CHIP_INFO_SOURCE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents whether disable FAST_VEF in RMA session.\\\\1: disable\\\\0: enable\\\\"]
    #[inline(always)]
    pub fn rma_disable_fast_vef(&self) -> RMA_DISABLE_FAST_VEF_R {
        RMA_DISABLE_FAST_VEF_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
    #[inline(always)]
    pub fn pvt_0_glitch_en(&self) -> PVT_0_GLITCH_EN_R {
        PVT_0_GLITCH_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Use to configure glitch mode"]
    #[inline(always)]
    pub fn pvt_0_glitch_mode(&self) -> PVT_0_GLITCH_MODE_R {
        PVT_0_GLITCH_MODE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 10 - Represents whether to enable PVT power glitch monitor function.\\\\1:Enable. \\\\0:Disable"]
    #[inline(always)]
    pub fn pvt_1_glitch_en(&self) -> PVT_1_GLITCH_EN_R {
        PVT_1_GLITCH_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bits 11:12 - Use to configure glitch mode"]
    #[inline(always)]
    pub fn pvt_1_glitch_mode(&self) -> PVT_1_GLITCH_MODE_R {
        PVT_1_GLITCH_MODE_R::new(((self.bits >> 11) & 3) as u8)
    }
    #[doc = "Bit 13 - FLASH power select.\\\\ 1'b1: use 3.3V\\\\1'b0: use 1.8V"]
    #[inline(always)]
    pub fn pmu_flash_power_sel(&self) -> PMU_FLASH_POWER_SEL_R {
        PMU_FLASH_POWER_SEL_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FLASH power select enable signal. 1'b1 : validates EFUSE_PMU_FLASH_POWER_SEL 1'b0: invalidates EFUSE_PMU_FLASH_POWER_SEL"]
    #[inline(always)]
    pub fn pmu_flash_power_sel_en(&self) -> PMU_FLASH_POWER_SEL_EN_R {
        PMU_FLASH_POWER_SEL_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 15:18 - set these bit enable power glitch enable"]
    #[inline(always)]
    pub fn power_glitch_en(&self) -> POWER_GLITCH_EN_R {
        POWER_GLITCH_EN_R::new(((self.bits >> 15) & 0x0f) as u8)
    }
    #[doc = "Bit 19 - Represents whether to enable XTS-AES shadow core countermeasure against fault injection attacks. \\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
    #[inline(always)]
    pub fn ena_xts_shadow(&self) -> ENA_XTS_SHADOW_R {
        ENA_XTS_SHADOW_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether to enable ciphertext scrambler for external memory . \\\\ 0: Disabled\\\\ 1: Enabled\\\\"]
    #[inline(always)]
    pub fn ena_spi_boot_crypt_scrambler(&self) -> ENA_SPI_BOOT_CRYPT_SCRAMBLER_R {
        ENA_SPI_BOOT_CRYPT_SCRAMBLER_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Represents which Crypto peripheral is selected for renabling JTAG. \\\\ 0: RMA\\\\ 1: HMAC\\\\"]
    #[inline(always)]
    pub fn re_enable_jtag_source(&self) -> RE_ENABLE_JTAG_SOURCE_R {
        RE_ENABLE_JTAG_SOURCE_R::new(((self.bits >> 21) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA6")
            .field("rma_session_counter", &self.rma_session_counter())
            .field("rma_nonce_ena", &self.rma_nonce_ena())
            .field("rma_chip_info_source", &self.rma_chip_info_source())
            .field("rma_disable_fast_vef", &self.rma_disable_fast_vef())
            .field("pvt_0_glitch_en", &self.pvt_0_glitch_en())
            .field("pvt_0_glitch_mode", &self.pvt_0_glitch_mode())
            .field("pvt_1_glitch_en", &self.pvt_1_glitch_en())
            .field("pvt_1_glitch_mode", &self.pvt_1_glitch_mode())
            .field("pmu_flash_power_sel", &self.pmu_flash_power_sel())
            .field("pmu_flash_power_sel_en", &self.pmu_flash_power_sel_en())
            .field("power_glitch_en", &self.power_glitch_en())
            .field("ena_xts_shadow", &self.ena_xts_shadow())
            .field(
                "ena_spi_boot_crypt_scrambler",
                &self.ena_spi_boot_crypt_scrambler(),
            )
            .field("re_enable_jtag_source", &self.re_enable_jtag_source())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data6::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA6_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data6::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA6_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA6 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA6_SPEC {}
