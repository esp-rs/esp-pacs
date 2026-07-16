#[doc = "Register `RD_REPEAT_DATA1` reader"]
pub type R = crate::R<RD_REPEAT_DATA1_SPEC>;
#[doc = "Register `RD_REPEAT_DATA1` writer"]
pub type W = crate::W<RD_REPEAT_DATA1_SPEC>;
#[doc = "Field `KM_RND_SWITCH_CYCLE` reader - Represents the control of key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles"]
pub type KM_RND_SWITCH_CYCLE_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_65` reader - "]
pub type RD_RESERVE_0_65_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_65` writer - "]
pub type RD_RESERVE_0_65_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `KM_DISABLE_DEPLOY_MODE` reader - Represents whether the deploy mode of key manager is disable or not. \\\\ 1: disabled \\\\ 0: enabled.\\\\ bit 0: ecsda, bit 1: flash & spi boot srambler, bit2: hmac & aes, bit3: ds & rma nonce, bit4: psram"]
pub type KM_DISABLE_DEPLOY_MODE_R = crate::FieldReader;
#[doc = "Field `KM_DEPLOY_ONLY_ONCE` reader - Represents whether corresponding key can only be deployed once. 1 is true, 0 is false. \\\\ 0: ecsda\\\\ 1: flash & spi boot srambler\\\\ 2: hmac & aes\\\\ 3: ds & rma nonce\\\\ 4: psram\\\\"]
pub type KM_DEPLOY_ONLY_ONCE_R = crate::FieldReader;
#[doc = "Field `FORCE_USE_KEY_MANAGER_KEY` reader - Represents whether corresponding key must come from key manager. 1 is true, 0 is false.\\\\ 0: ecsda\\\\ 1: flash\\\\ 2: reserved\\\\ 3: reserved\\\\ 4: psram\\\\"]
pub type FORCE_USE_KEY_MANAGER_KEY_R = crate::FieldReader;
#[doc = "Field `FORCE_DISABLE_SW_INIT_KEY` reader - Represents whether to disable software written init key, and force use efuse_init_key."]
pub type FORCE_DISABLE_SW_INIT_KEY_R = crate::BitReader;
#[doc = "Field `KM_XTS_KEY_LENGTH_256` reader - Represents whether to configure flash encryption use xts-128 key. else use xts-256 key. \\\\ 0: 128-bit key \\\\ 1: 256-bit key\\\\"]
pub type KM_XTS_KEY_LENGTH_256_R = crate::BitReader;
#[doc = "Field `WDT_DELAY_SEL` reader - Represents the threshold level of the RTC watchdog STG0 timeout.\\\\0: Original threshold configuration value of STG0 *2 \\\\1: Original threshold configuration value of STG0 *4 \\\\2: Original threshold configuration value of STG0 *8 \\\\3: Original threshold configuration value of STG0 *16 \\\\"]
pub type WDT_DELAY_SEL_R = crate::BitReader;
#[doc = "Field `DIS_SM_CRYPT` reader - Represents whether to disable all the SM crypto functions, including SM2, SM3.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
pub type DIS_SM_CRYPT_R = crate::BitReader;
#[doc = "Field `SPI_BOOT_CRYPT_CNT` reader - Represents whether SPI boot encrypt/decrypt is disabled or enabled.\\\\ Odd number of 1: enabled\\\\ Even number of 1: disabled\\\\"]
pub type SPI_BOOT_CRYPT_CNT_R = crate::FieldReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE0` reader - Represents whether revoking first secure boot key is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_KEY_REVOKE0_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE1` reader - Represents whether revoking second secure boot key is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_KEY_REVOKE1_R = crate::BitReader;
#[doc = "Field `SECURE_BOOT_KEY_REVOKE2` reader - Represents whether revoking third secure boot key is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
pub type SECURE_BOOT_KEY_REVOKE2_R = crate::BitReader;
#[doc = "Field `RD_RESERVE_0_91` reader - "]
pub type RD_RESERVE_0_91_R = crate::FieldReader;
#[doc = "Field `RD_RESERVE_0_91` writer - "]
pub type RD_RESERVE_0_91_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bit 0 - Represents the control of key manager random number switch cycle. 0: control by register. 1: 8 km clk cycles. 2: 16 km cycles. 3: 32 km cycles"]
    #[inline(always)]
    pub fn km_rnd_switch_cycle(&self) -> KM_RND_SWITCH_CYCLE_R {
        KM_RND_SWITCH_CYCLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rd_reserve_0_65(&self) -> RD_RESERVE_0_65_R {
        RD_RESERVE_0_65_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:6 - Represents whether the deploy mode of key manager is disable or not. \\\\ 1: disabled \\\\ 0: enabled.\\\\ bit 0: ecsda, bit 1: flash & spi boot srambler, bit2: hmac & aes, bit3: ds & rma nonce, bit4: psram"]
    #[inline(always)]
    pub fn km_disable_deploy_mode(&self) -> KM_DISABLE_DEPLOY_MODE_R {
        KM_DISABLE_DEPLOY_MODE_R::new(((self.bits >> 2) & 0x1f) as u8)
    }
    #[doc = "Bits 7:11 - Represents whether corresponding key can only be deployed once. 1 is true, 0 is false. \\\\ 0: ecsda\\\\ 1: flash & spi boot srambler\\\\ 2: hmac & aes\\\\ 3: ds & rma nonce\\\\ 4: psram\\\\"]
    #[inline(always)]
    pub fn km_deploy_only_once(&self) -> KM_DEPLOY_ONLY_ONCE_R {
        KM_DEPLOY_ONLY_ONCE_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
    #[doc = "Bits 12:16 - Represents whether corresponding key must come from key manager. 1 is true, 0 is false.\\\\ 0: ecsda\\\\ 1: flash\\\\ 2: reserved\\\\ 3: reserved\\\\ 4: psram\\\\"]
    #[inline(always)]
    pub fn force_use_key_manager_key(&self) -> FORCE_USE_KEY_MANAGER_KEY_R {
        FORCE_USE_KEY_MANAGER_KEY_R::new(((self.bits >> 12) & 0x1f) as u8)
    }
    #[doc = "Bit 17 - Represents whether to disable software written init key, and force use efuse_init_key."]
    #[inline(always)]
    pub fn force_disable_sw_init_key(&self) -> FORCE_DISABLE_SW_INIT_KEY_R {
        FORCE_DISABLE_SW_INIT_KEY_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Represents whether to configure flash encryption use xts-128 key. else use xts-256 key. \\\\ 0: 128-bit key \\\\ 1: 256-bit key\\\\"]
    #[inline(always)]
    pub fn km_xts_key_length_256(&self) -> KM_XTS_KEY_LENGTH_256_R {
        KM_XTS_KEY_LENGTH_256_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Represents the threshold level of the RTC watchdog STG0 timeout.\\\\0: Original threshold configuration value of STG0 *2 \\\\1: Original threshold configuration value of STG0 *4 \\\\2: Original threshold configuration value of STG0 *8 \\\\3: Original threshold configuration value of STG0 *16 \\\\"]
    #[inline(always)]
    pub fn wdt_delay_sel(&self) -> WDT_DELAY_SEL_R {
        WDT_DELAY_SEL_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Represents whether to disable all the SM crypto functions, including SM2, SM3.\\\\ 1: disabled\\\\ 0: enabled\\\\"]
    #[inline(always)]
    pub fn dis_sm_crypt(&self) -> DIS_SM_CRYPT_R {
        DIS_SM_CRYPT_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Represents whether SPI boot encrypt/decrypt is disabled or enabled.\\\\ Odd number of 1: enabled\\\\ Even number of 1: disabled\\\\"]
    #[inline(always)]
    pub fn spi_boot_crypt_cnt(&self) -> SPI_BOOT_CRYPT_CNT_R {
        SPI_BOOT_CRYPT_CNT_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Represents whether revoking first secure boot key is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_key_revoke0(&self) -> SECURE_BOOT_KEY_REVOKE0_R {
        SECURE_BOOT_KEY_REVOKE0_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Represents whether revoking second secure boot key is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_key_revoke1(&self) -> SECURE_BOOT_KEY_REVOKE1_R {
        SECURE_BOOT_KEY_REVOKE1_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Represents whether revoking third secure boot key is enabled or disabled.\\\\ 1: enabled\\\\ 0: disabled\\\\"]
    #[inline(always)]
    pub fn secure_boot_key_revoke2(&self) -> SECURE_BOOT_KEY_REVOKE2_R {
        SECURE_BOOT_KEY_REVOKE2_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rd_reserve_0_91(&self) -> RD_RESERVE_0_91_R {
        RD_RESERVE_0_91_R::new(((self.bits >> 27) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_DATA1")
            .field("km_rnd_switch_cycle", &self.km_rnd_switch_cycle())
            .field("km_disable_deploy_mode", &self.km_disable_deploy_mode())
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
            .field("wdt_delay_sel", &self.wdt_delay_sel())
            .field("dis_sm_crypt", &self.dis_sm_crypt())
            .field("spi_boot_crypt_cnt", &self.spi_boot_crypt_cnt())
            .field("secure_boot_key_revoke0", &self.secure_boot_key_revoke0())
            .field("secure_boot_key_revoke1", &self.secure_boot_key_revoke1())
            .field("secure_boot_key_revoke2", &self.secure_boot_key_revoke2())
            .field("rd_reserve_0_65", &self.rd_reserve_0_65())
            .field("rd_reserve_0_91", &self.rd_reserve_0_91())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn rd_reserve_0_65(&mut self) -> RD_RESERVE_0_65_W<'_, RD_REPEAT_DATA1_SPEC> {
        RD_RESERVE_0_65_W::new(self, 1)
    }
    #[doc = "Bits 27:31"]
    #[inline(always)]
    pub fn rd_reserve_0_91(&mut self) -> RD_RESERVE_0_91_W<'_, RD_REPEAT_DATA1_SPEC> {
        RD_RESERVE_0_91_W::new(self, 27)
    }
}
#[doc = "Represents rd_repeat_data\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_repeat_data1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_repeat_data1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_REPEAT_DATA1_SPEC;
impl crate::RegisterSpec for RD_REPEAT_DATA1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_repeat_data1::R`](R) reader structure"]
impl crate::Readable for RD_REPEAT_DATA1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_repeat_data1::W`](W) writer structure"]
impl crate::Writable for RD_REPEAT_DATA1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_REPEAT_DATA1 to value 0"]
impl crate::Resettable for RD_REPEAT_DATA1_SPEC {}
