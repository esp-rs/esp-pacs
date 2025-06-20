#[doc = "Register `RD_REPEAT_DATA_ERR1` reader"]
pub type R = crate::R<RD_REPEAT_DATA_ERR1_SPEC>;
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
#[doc = "Field `WDT_DELAY_SEL_ERR` reader - Represents the programming error of EFUSE_WDT_DELAY_SEL"]
pub type WDT_DELAY_SEL_ERR_R = crate::FieldReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT_ERR` reader - Represents the programming error of EFUSE_SPI_BOOT_CRYPT_CNT"]
pub type SPI_BOOT_CRYPT_CNT_ERR_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_KEY_REVOKE0"]
pub type SECURE_BOOT_KEY_REVOKE0_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_KEY_REVOKE1"]
pub type SECURE_BOOT_KEY_REVOKE1_ERR_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2_ERR` reader - Represents the programming error of EFUSE_SECURE_BOOT_KEY_REVOKE2"]
pub type SECURE_BOOT_KEY_REVOKE2_ERR_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_0"]
pub type KEY_PURPOSE_0_ERR_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1_ERR` reader - Represents the programming error of EFUSE_KEY_PURPOSE_1"]
pub type KEY_PURPOSE_1_ERR_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:3 - Represents the programming error of EFUSE_KM_DISABLE_DEPLOY_MODE"]
    #[inline(always)]
    pub fn km_disable_deploy_mode_err(&self) -> KM_DISABLE_DEPLOY_MODE_ERR_R {
        KM_DISABLE_DEPLOY_MODE_ERR_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:5 - Represents the programming error of EFUSE_KM_RND_SWITCH_CYCLE"]
    #[inline(always)]
    pub fn km_rnd_switch_cycle_err(&self) -> KM_RND_SWITCH_CYCLE_ERR_R {
        KM_RND_SWITCH_CYCLE_ERR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:9 - Represents the programming error of EFUSE_KM_DEPLOY_ONLY_ONCE"]
    #[inline(always)]
    pub fn km_deploy_only_once_err(&self) -> KM_DEPLOY_ONLY_ONCE_ERR_R {
        KM_DEPLOY_ONLY_ONCE_ERR_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
    #[doc = "Bits 10:13 - Represents the programming error of EFUSE_FORCE_USE_KEY_MANAGER_KEY"]
    #[inline(always)]
    pub fn force_use_key_manager_key_err(&self) -> FORCE_USE_KEY_MANAGER_KEY_ERR_R {
        FORCE_USE_KEY_MANAGER_KEY_ERR_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 14 - Represents the programming error of EFUSE_FORCE_DISABLE_SW_INIT_KEY"]
    #[inline(always)]
    pub fn force_disable_sw_init_key_err(&self) -> FORCE_DISABLE_SW_INIT_KEY_ERR_R {
        FORCE_DISABLE_SW_INIT_KEY_ERR_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Represents the programming error of EFUSE_WDT_DELAY_SEL"]
    #[inline(always)]
    pub fn wdt_delay_sel_err(&self) -> WDT_DELAY_SEL_ERR_R {
        WDT_DELAY_SEL_ERR_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Represents the programming error of EFUSE_SPI_BOOT_CRYPT_CNT"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt_err(&self) -> SPI_BOOT_CRYPT_CNT_ERR_R {
        SPI_BOOT_CRYPT_CNT_ERR_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Represents the programming error of EFUSE_SECURE_BOOT_KEY_REVOKE0"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0_err(&self) -> SECURE_BOOT_KEY_REVOKE0_ERR_R {
        SECURE_BOOT_KEY_REVOKE0_ERR_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents the programming error of EFUSE_SECURE_BOOT_KEY_REVOKE1"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1_err(&self) -> SECURE_BOOT_KEY_REVOKE1_ERR_R {
        SECURE_BOOT_KEY_REVOKE1_ERR_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents the programming error of EFUSE_SECURE_BOOT_KEY_REVOKE2"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2_err(&self) -> SECURE_BOOT_KEY_REVOKE2_ERR_R {
        SECURE_BOOT_KEY_REVOKE2_ERR_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Represents the programming error of EFUSE_KEY_PURPOSE_0"]
    #[inline(always)]
    pub fn key_purpose_0_err(&self) -> KEY_PURPOSE_0_ERR_R {
        KEY_PURPOSE_0_ERR_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Represents the programming error of EFUSE_KEY_PURPOSE_1"]
    #[inline(always)]
    pub fn key_purpose_1_err(&self) -> KEY_PURPOSE_1_ERR_R {
        KEY_PURPOSE_1_ERR_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA_ERR1")
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
#[doc = "Represents rd_repeat_data_err\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data_err1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA_ERR1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA_ERR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data_err1::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA_ERR1_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA_ERR1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA_ERR1_SPEC {
    const RESET_VALUE: u32 = 0;
}
