#[doc = "Register `RD_REPEAT_DATA1` reader"]
pub type R = crate::R<RD_REPEAT_DATA1_SPEC>;
#[doc = "Field `KM_HUK_GEN_STATE_HIGH` reader - Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
pub type KM_HUK_GEN_STATE_HIGH_R = crate::FieldReader;
#[doc = "Field `KM_RND_SWITCH_CYCLE` reader - Set bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles."]
pub type KM_RND_SWITCH_CYCLE_R = crate::FieldReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE` reader - Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds."]
pub type KM_DEPLOY_ONLY_ONCE_R = crate::FieldReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY` reader - Set each bit to control whether corresponding key must come from key manager.. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds."]
pub type FORCE_USE_KEY_MANAGER_KEY_R = crate::FieldReader;
#[doc = "Field `FORCE_DISABLE_SW_INIT_KEY` reader - Set this bit to disable software written init key, and force use efuse_init_key."]
pub type FORCE_DISABLE_SW_INIT_KEY_R = crate::BitReader;
#[doc = "Field `XTS_KEY_LENGTH_256` reader - Set this bit to configure flash encryption use xts-128 key, else use xts-256 key."]
pub type XTS_KEY_LENGTH_256_R = crate::BitReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Represents whether RTC watchdog timeout threshold is selected at startup. 1: selected. 0: not selected."]
pub type WDT_DELAY_SEL_R = crate::FieldReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - Represents whether SPI boot encrypt/decrypt is disabled or enabled. Odd number of 1: enabled. Even number of 1: disabled."]
pub type SPI_BOOT_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - Represents whether revoking first secure boot key is enabled or disabled. 1: enabled. 0: disabled."]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - Represents whether revoking second secure boot key is enabled or disabled. 1: enabled. 0: disabled."]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - Represents whether revoking third secure boot key is enabled or disabled. 1: enabled. 0: disabled."]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::BitReader;
#[doc = "Field `KEY_PURPOSE_0` reader - Represents the purpose of Key0."]
pub type KEY_PURPOSE_0_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE_1` reader - Represents the purpose of Key1."]
pub type KEY_PURPOSE_1_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - Set this bit to control validation of HUK generate mode. Odd of 1 is invalid, even of 1 is valid."]
    #[inline(always)]
    pub fn km_huk_gen_state_high(&self) -> KM_HUK_GEN_STATE_HIGH_R {
        KM_HUK_GEN_STATE_HIGH_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:4 - Set bits to control key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles."]
    #[inline(always)]
    pub fn km_rnd_switch_cycle(&self) -> KM_RND_SWITCH_CYCLE_R {
        KM_RND_SWITCH_CYCLE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:8 - Set each bit to control whether corresponding key can only be deployed once. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds."]
    #[inline(always)]
    pub fn km_deploy_only_once(&self) -> KM_DEPLOY_ONLY_ONCE_R {
        KM_DEPLOY_ONLY_ONCE_R::new(((self.bits >> 5) & 0x0f) as u8)
    }
    #[doc = "Bits 9:12 - Set each bit to control whether corresponding key must come from key manager.. 1 is true, 0 is false. Bit0: ecdsa. Bit1: xts. Bit2: hmac. Bit3: ds."]
    #[inline(always)]
    pub fn force_use_key_manager_key(&self) -> FORCE_USE_KEY_MANAGER_KEY_R {
        FORCE_USE_KEY_MANAGER_KEY_R::new(((self.bits >> 9) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - Set this bit to disable software written init key, and force use efuse_init_key."]
    #[inline(always)]
    pub fn force_disable_sw_init_key(&self) -> FORCE_DISABLE_SW_INIT_KEY_R {
        FORCE_DISABLE_SW_INIT_KEY_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to configure flash encryption use xts-128 key, else use xts-256 key."]
    #[inline(always)]
    pub fn xts_key_length_256(&self) -> XTS_KEY_LENGTH_256_R {
        XTS_KEY_LENGTH_256_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Represents whether RTC watchdog timeout threshold is selected at startup. 1: selected. 0: not selected."]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Represents whether SPI boot encrypt/decrypt is disabled or enabled. Odd number of 1: enabled. Even number of 1: disabled."]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 21 - Represents whether revoking first secure boot key is enabled or disabled. 1: enabled. 0: disabled."]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Represents whether revoking second secure boot key is enabled or disabled. 1: enabled. 0: disabled."]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Represents whether revoking third secure boot key is enabled or disabled. 1: enabled. 0: disabled."]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:27 - Represents the purpose of Key0."]
    #[inline(always)]
    pub fn key_purpose_0(&self) -> KEY_PURPOSE_0_R {
        KEY_PURPOSE_0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 28:31 - Represents the purpose of Key1."]
    #[inline(always)]
    pub fn key_purpose_1(&self) -> KEY_PURPOSE_1_R {
        KEY_PURPOSE_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA1")
            .field("km_huk_gen_state_high", &self.km_huk_gen_state_high())
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
            .field("xts_key_length_256", &self.xts_key_length_256())
            .field("wdt_delay_sel", &self.wdt_delay_sel())
            .field("spi_boot_crypt_cnt", &self.spi_boot_crypt_cnt())
            .field("secure_boot_key_revoke0", &self.secure_boot_key_revoke0())
            .field("secure_boot_key_revoke1", &self.secure_boot_key_revoke1())
            .field("secure_boot_key_revoke2", &self.secure_boot_key_revoke2())
            .field("key_purpose_0", &self.key_purpose_0())
            .field("key_purpose_1", &self.key_purpose_1())
            .finish()
    }
}
#[doc = "BLOCK0 data register 2.\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data1::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA1_SPEC {}
#[doc = "`reset()` method sets RD_REPEAT_DATA1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA1_SPEC {}
