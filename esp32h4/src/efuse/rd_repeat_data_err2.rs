#[doc = "Register `RD_REPEAT_DATA_ERR2` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR2_SPEC>;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL_ERR` reader - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
pub type XTS_DPA_PSEUDO_LEVEL_ERR_R = crate::FieldReader;
#[doc = "Field `XTS_DPA_CLK_ENABLE_ERR` reader - Represents the programming error of EFUSE_XTS_DPA_CLK_ENABLE"]
pub type XTS_DPA_CLK_ENABLE_ERR_R = crate::BitReader;
#[doc = "Field `ECC_FORCE_CONST_TIME_ERR` reader - Represents the programming error of EFUSE_ECC_FORCE_CONST_TIME"]
pub type ECC_FORCE_CONST_TIME_ERR_R = crate::BitReader;
#[doc = "Field `ECDSA_P384_ENABLE_ERR` reader - Represents the programming error of EFUSE_ECDSA_P384_ENABLE"]
pub type ECDSA_P384_ENABLE_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_EN"]
pub type SECURE_BOOT_EN_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE"]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE_ERR` reader - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE"]
pub type KM_DISABLE_DEPLOY_MODE_ERR_R = crate::FieldReader;
#[doc = "Field `KM_RND_SWITCH_CYCLE_ERR` reader - Represents the programming error of EFUSE_KM_RND_SWITCH_CYCLE"]
pub type KM_RND_SWITCH_CYCLE_ERR_R = crate::FieldReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE_ERR` reader - Represents the programming error of EFUSE_KM_DEPLOY_ONLY_ONCE"]
pub type KM_DEPLOY_ONLY_ONCE_ERR_R = crate::FieldReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY_ERR` reader - Represents the programming error of EFUSE_FORCE_USE_KEY_MANAGER_KEY"]
pub type FORCE_USE_KEY_MANAGER_KEY_ERR_R = crate::FieldReader;
#[doc = "Field `FORCE_DISABLE_SW_INIT_KEY_ERR` reader - Represents the programming error of EFUSE_FORCE_DISABLE_SW_INIT_KEY"]
pub type FORCE_DISABLE_SW_INIT_KEY_ERR_R = crate::BitReader;
#[doc = "Field `KM_XTS_KEY_LENGTH_256_ERR` reader - Represents the programming error of EFUSE_KM_XTS_KEY_LENGTH_256"]
pub type KM_XTS_KEY_LENGTH_256_ERR_R = crate::BitReader;
#[doc = "Field `LOCK_KM_KEY_ERR` reader - Represents the programming error of EFUSE_LOCK_KM_KEY"]
pub type LOCK_KM_KEY_ERR_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW_ERR` reader - Represents the programming error of EFUSE_FLASH_TPUW"]
pub type FLASH_TPUW_ERR_R = crate::FieldReader;
#[doc = "Field `DIS_DOWNLOAD_MODE_ERR` reader - Represents the programming error of EFUSE_DIS_DOWNLOAD_MODE"]
pub type DIS_DOWNLOAD_MODE_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Represents the programming error of EFUSE_XTS_DPA_PSEUDO_LEVEL"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level_err(&self) -> XTS_DPA_PSEUDO_LEVEL_ERR_R {
        XTS_DPA_PSEUDO_LEVEL_ERR_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents the programming error of EFUSE_XTS_DPA_CLK_ENABLE"]
    #[inline(always)]
    pub fn xts_dpa_clk_enable_err(&self) -> XTS_DPA_CLK_ENABLE_ERR_R {
        XTS_DPA_CLK_ENABLE_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents the programming error of EFUSE_ECC_FORCE_CONST_TIME"]
    #[inline(always)]
    pub fn ecc_force_const_time_err(&self) -> ECC_FORCE_CONST_TIME_ERR_R {
        ECC_FORCE_CONST_TIME_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents the programming error of EFUSE_ECDSA_P384_ENABLE"]
    #[inline(always)]
    pub fn ecdsa_p384_enable_err(&self) -> ECDSA_P384_ENABLE_ERR_R {
        ECDSA_P384_ENABLE_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents the programming error of EFUSE_SECURE_BOOT_EN"]
    #[inline(always)]
    pub fn secure_boot_en_err(&self) -> SECURE_BOOT_EN_ERR_R {
        SECURE_BOOT_EN_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents the programming error of EFUSE_SECURE_BOOT_AGGRESSIVE_REVOKE"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke_err(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_ERR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_err(&self) -> KM_DISABLE_DEPLOY_MODE_ERR_R {
        KM_DISABLE_DEPLOY_MODE_ERR_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:13 - Represents the programming error of EFUSE_KM_RND_SWITCH_CYCLE"]
    #[inline(always)]
    pub fn km_rnd_switch_cycle_err(&self) -> KM_RND_SWITCH_CYCLE_ERR_R {
        KM_RND_SWITCH_CYCLE_ERR_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:18 - Represents the programming error of EFUSE_KM_DEPLOY_ONLY_ONCE"]
    #[inline(always)]
    pub fn km_deploy_only_once_err(&self) -> KM_DEPLOY_ONLY_ONCE_ERR_R {
        KM_DEPLOY_ONLY_ONCE_ERR_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Represents the programming error of EFUSE_FORCE_USE_KEY_MANAGER_KEY"]
    #[inline(always)]
    pub fn force_use_key_manager_key_err(&self) -> FORCE_USE_KEY_MANAGER_KEY_ERR_R {
        FORCE_USE_KEY_MANAGER_KEY_ERR_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Represents the programming error of EFUSE_FORCE_DISABLE_SW_INIT_KEY"]
    #[inline(always)]
    pub fn force_disable_sw_init_key_err(&self) -> FORCE_DISABLE_SW_INIT_KEY_ERR_R {
        FORCE_DISABLE_SW_INIT_KEY_ERR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents the programming error of EFUSE_KM_XTS_KEY_LENGTH_256"]
    #[inline(always)]
    pub fn km_xts_key_length_256_err(&self) -> KM_XTS_KEY_LENGTH_256_ERR_R {
        KM_XTS_KEY_LENGTH_256_ERR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents the programming error of EFUSE_LOCK_KM_KEY"]
    #[inline(always)]
    pub fn lock_km_key_err(&self) -> LOCK_KM_KEY_ERR_R {
        LOCK_KM_KEY_ERR_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Represents the programming error of EFUSE_FLASH_TPUW"]
    #[inline(always)]
    pub fn flash_tpuw_err(&self) -> FLASH_TPUW_ERR_R {
        FLASH_TPUW_ERR_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 31 - Represents the programming error of EFUSE_DIS_DOWNLOAD_MODE"]
    #[inline(always)]
    pub fn dis_download_mode_err(&self) -> DIS_DOWNLOAD_MODE_ERR_R {
        DIS_DOWNLOAD_MODE_ERR_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR2")
            .field("xts_dpa_pseudo_level_err", &self.xts_dpa_pseudo_level_err())
            .field("xts_dpa_clk_enable_err", &self.xts_dpa_clk_enable_err())
            .field("ecc_force_const_time_err", &self.ecc_force_const_time_err())
            .field("ecdsa_p384_enable_err", &self.ecdsa_p384_enable_err())
            .field("secure_boot_en_err", &self.secure_boot_en_err())
            .field(
                "secure_boot_aggressive_revoke_err",
                &self.secure_boot_aggressive_revoke_err(),
            )
            .field(
                "km_disable_deploy_mode_err",
                &self.km_disable_deploy_mode_err(),
            )
            .field("km_rnd_switch_cycle_err", &self.km_rnd_switch_cycle_err())
            .field("km_deploy_only_once_err", &self.km_deploy_only_once_err())
            .field(
                "force_use_key_manager_key_err",
                &self.force_use_key_manager_key_err(),
            )
            .field(
                "force_disable_sw_init_key_err",
                &self.force_disable_sw_init_key_err(),
            )
            .field(
                "km_xts_key_length_256_err",
                &self.km_xts_key_length_256_err(),
            )
            .field("lock_km_key_err", &self.lock_km_key_err())
            .field("flash_tpuw_err", &self.flash_tpuw_err())
            .field("dis_download_mode_err", &self.dis_download_mode_err())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR2_SPEC {}
