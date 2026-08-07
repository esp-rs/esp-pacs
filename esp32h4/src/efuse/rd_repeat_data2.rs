#[doc = "Register `RD_REPEAT_DATA2` reader"]
pub type R = crate::R<RD_REPEAT_DATA2_SPEC>;
#[doc = "Field `XTS_DPA_PSEUDO_LEVEL` reader - Represents the pseudo round level of xts-aes anti-dpa attack.\\\\ 3: High.\\\\ 2: Moderate 1. Low\\\\ 0: Disabled\\\\"]
pub type XTS_DPA_PSEUDO_LEVEL_R = crate::FieldReader;
#[doc = "Field `XTS_DPA_CLK_ENABLE` reader - Represents whether xts-aes anti-dpa attack clock is enabled.\\\\ 1. Enable.\\\\ 0: Disable.\\\\"]
pub type XTS_DPA_CLK_ENABLE_R = crate::BitReader;
#[doc = "Field `ECC_FORCE_CONST_TIME` reader - Represents whether to force ecc to use const-time calculation mode. \\\\ 1: Enable. \\\\ 0: Disable."]
pub type ECC_FORCE_CONST_TIME_R = crate::BitReader;
#[doc = "Field `ECDSA_P384_ENABLE` reader - Represents if the chip supports ECDSA P384"]
pub type ECDSA_P384_ENABLE_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_EN` reader - Represents whether secure boot is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_EN_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_AGGRESSIVE_REVOKE` reader - Represents whether revoking aggressive secure boot is enabled or disabled.\\\\ 1: enabled.\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_AGGRESSIVE_REVOKE_R = crate::BitReader;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE` reader - Represents whether the new key deployment of key manager is disabled. \\\\Bit0: Represents whether the new ECDSA key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\Bit1: Represents whether the new XTS-AES (flash and PSRAM) key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\Bit2: Represents whether the new HMAC key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\Bit3: Represents whether the new DS key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\"]
pub type KM_DISABLE_DEPLOY_MODE_R = crate::FieldReader;
#[doc = "Field `KM_RND_SWITCH_CYCLE` reader - Represents the cycle at which the Key Manager switches random numbers.\\\\0: Controlled by the \\hyperref\\[fielddesc:KEYMNGRNDSWITCHCYCLE\\]{KEYMNG\\_RND\\_SWITCH\\_CYCLE} register. For more information, please refer to Chapter \\ref{mod:keymng} \\textit{\nameref{mod:keymng}}\\\\1: 8 Key Manager clock cycles\\\\2: 16 Key Manager clock cycles\\\\3: 32 Key Manager clock cycles\\\\"]
pub type KM_RND_SWITCH_CYCLE_R = crate::FieldReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE` reader - Represents whether the corresponding key can be deployed only once.\\\\Bit0: Represents whether the ECDSA key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\Bit1: Represents whether the XTS-AES (flash and PSRAM) key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\Bit2: Represents whether the HMAC key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\Bit3: Represents whether the DS key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\"]
pub type KM_DEPLOY_ONLY_ONCE_R = crate::FieldReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY` reader - Represents whether the corresponding key must come from Key Manager. \\\\Bit0: Represents whether the ECDSA key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\Bit1: Represents whether the XTS-AES (flash and PSRAM) key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\Bit2: Represents whether the HMAC key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\Bit3: Represents whether the DS key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\"]
pub type FORCE_USE_KEY_MANAGER_KEY_R = crate::FieldReader;
#[doc = "Field `FORCE_DISABLE_SW_INIT_KEY` reader - Represents whether to disable the use of the initialization key written by software and instead force use efuse\\_init\\_key.\\\\0: Enable\\\\1: Disable\\\\"]
pub type FORCE_DISABLE_SW_INIT_KEY_R = crate::BitReader;
#[doc = "Field `KM_XTS_KEY_LENGTH_256` reader - Represents which key flash encryption uses.\\\\0: XTS-AES-256 key\\\\1: XTS-AES-128 key\\\\"]
pub type KM_XTS_KEY_LENGTH_256_R = crate::BitReader;
#[doc = "Field `LOCK_KM_KEY` reader - Represents whether the keys in the Key Manager are locked after deployment.\\\\0: Not locked\\\\1: Locked\\\\"]
pub type LOCK_KM_KEY_R = crate::BitReader;
#[doc = "Field `FLASH_TPUW` reader - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
pub type FLASH_TPUW_R = crate::FieldReader;
#[doc = "Field `DIS_DOWNLOAD_MODE` reader - Represents whether Download mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_DOWNLOAD_MODE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - Represents the pseudo round level of xts-aes anti-dpa attack.\\\\ 3: High.\\\\ 2: Moderate 1. Low\\\\ 0: Disabled\\\\"]
    #[inline(always)]
    pub fn xts_dpa_pseudo_level(&self) -> XTS_DPA_PSEUDO_LEVEL_R {
        XTS_DPA_PSEUDO_LEVEL_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Represents whether xts-aes anti-dpa attack clock is enabled.\\\\ 1. Enable.\\\\ 0: Disable.\\\\"]
    #[inline(always)]
    pub fn xts_dpa_clk_enable(&self) -> XTS_DPA_CLK_ENABLE_R {
        XTS_DPA_CLK_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Represents whether to force ecc to use const-time calculation mode. \\\\ 1: Enable. \\\\ 0: Disable."]
    #[inline(always)]
    pub fn ecc_force_const_time(&self) -> ECC_FORCE_CONST_TIME_R {
        ECC_FORCE_CONST_TIME_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Represents if the chip supports ECDSA P384"]
    #[inline(always)]
    pub fn ecdsa_p384_enable(&self) -> ECDSA_P384_ENABLE_R {
        ECDSA_P384_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Represents whether secure boot is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_en(&self) -> SECURE_BOOT_EN_R {
        SECURE_BOOT_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Represents whether revoking aggressive secure boot is enabled or disabled.\\\\ 1: enabled.\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_aggressive_revoke(&self) -> SECURE_BOOT_AGGRESSIVE_REVOKE_R {
        SECURE_BOOT_AGGRESSIVE_REVOKE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:11 - Represents whether the new key deployment of key manager is disabled. \\\\Bit0: Represents whether the new ECDSA key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\Bit1: Represents whether the new XTS-AES (flash and PSRAM) key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\Bit2: Represents whether the new HMAC key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\Bit3: Represents whether the new DS key deployment is disabled\\\\0: Enabled\\\\1: Disabled\\\\"]
    #[inline(always)]
    pub fn km_disable_deploy_mode(&self) -> KM_DISABLE_DEPLOY_MODE_R {
        KM_DISABLE_DEPLOY_MODE_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:13 - Represents the cycle at which the Key Manager switches random numbers.\\\\0: Controlled by the \\hyperref\\[fielddesc:KEYMNGRNDSWITCHCYCLE\\]{KEYMNG\\_RND\\_SWITCH\\_CYCLE} register. For more information, please refer to Chapter \\ref{mod:keymng} \\textit{\nameref{mod:keymng}}\\\\1: 8 Key Manager clock cycles\\\\2: 16 Key Manager clock cycles\\\\3: 32 Key Manager clock cycles\\\\"]
    #[inline(always)]
    pub fn km_rnd_switch_cycle(&self) -> KM_RND_SWITCH_CYCLE_R {
        KM_RND_SWITCH_CYCLE_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:18 - Represents whether the corresponding key can be deployed only once.\\\\Bit0: Represents whether the ECDSA key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\Bit1: Represents whether the XTS-AES (flash and PSRAM) key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\Bit2: Represents whether the HMAC key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\Bit3: Represents whether the DS key can be deployed only once\\\\0: The key can be deployed multiple times\\\\1: The key can be deployed only once\\\\"]
    #[inline(always)]
    pub fn km_deploy_only_once(&self) -> KM_DEPLOY_ONLY_ONCE_R {
        KM_DEPLOY_ONLY_ONCE_R::new(((self.bits >> 14) & 0x1f) as u8)
    }
    #[doc = "Bits 19:23 - Represents whether the corresponding key must come from Key Manager. \\\\Bit0: Represents whether the ECDSA key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\Bit1: Represents whether the XTS-AES (flash and PSRAM) key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\Bit2: Represents whether the HMAC key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\Bit3: Represents whether the DS key must come from Key Manager.\\\\0: The key does not need to come from Key Manager\\\\1: The key must come from Key Manager\\\\"]
    #[inline(always)]
    pub fn force_use_key_manager_key(&self) -> FORCE_USE_KEY_MANAGER_KEY_R {
        FORCE_USE_KEY_MANAGER_KEY_R::new(((self.bits >> 19) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Represents whether to disable the use of the initialization key written by software and instead force use efuse\\_init\\_key.\\\\0: Enable\\\\1: Disable\\\\"]
    #[inline(always)]
    pub fn force_disable_sw_init_key(&self) -> FORCE_DISABLE_SW_INIT_KEY_R {
        FORCE_DISABLE_SW_INIT_KEY_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents which key flash encryption uses.\\\\0: XTS-AES-256 key\\\\1: XTS-AES-128 key\\\\"]
    #[inline(always)]
    pub fn km_xts_key_length_256(&self) -> KM_XTS_KEY_LENGTH_256_R {
        KM_XTS_KEY_LENGTH_256_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents whether the keys in the Key Manager are locked after deployment.\\\\0: Not locked\\\\1: Locked\\\\"]
    #[inline(always)]
    pub fn lock_km_key(&self) -> LOCK_KM_KEY_R {
        LOCK_KM_KEY_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:29 - Represents the flash waiting time after power-up, in unit of ms. When the value less than 15, the waiting time is the programmed value. Otherwise, the waiting time is 2 times the programmed value."]
    #[inline(always)]
    pub fn flash_tpuw(&self) -> FLASH_TPUW_R {
        FLASH_TPUW_R::new(((self.bits >> 27) & 7) as u8)
    }
    #[doc = "Bit 31 - Represents whether Download mode is disabled or enabled.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_download_mode(&self) -> DIS_DOWNLOAD_MODE_R {
        DIS_DOWNLOAD_MODE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA2")
            .field("xts_dpa_pseudo_level", &self.xts_dpa_pseudo_level())
            .field("xts_dpa_clk_enable", &self.xts_dpa_clk_enable())
            .field("ecc_force_const_time", &self.ecc_force_const_time())
            .field("ecdsa_p384_enable", &self.ecdsa_p384_enable())
            .field("secure_boot_en", &self.secure_boot_en())
            .field(
                "secure_boot_aggressive_revoke",
                &self.secure_boot_aggressive_revoke(),
            )
            .field("km_disable_deploy_mode", &self.km_disable_deploy_mode())
            .field("km_rnd_switch_cycle", &self.km_rnd_switch_cycle())
            .field("km_deploy_only_once", &self.km_deploy_only_once())
            .field(
                "force_use_key_manager_key",
                &self.force_use_key_manager_key(),
            )
            .field(
                "force_disable_sw_init_key",
                &self.force_disable_sw_init_key(),
            )
            .field("km_xts_key_length_256", &self.km_xts_key_length_256())
            .field("lock_km_key", &self.lock_km_key())
            .field("flash_tpuw", &self.flash_tpuw())
            .field("dis_download_mode", &self.dis_download_mode())
            .finish()
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA2_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data2::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA2_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA2 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA2_SPEC {}
