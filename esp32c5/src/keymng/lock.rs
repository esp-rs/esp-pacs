#[doc = "Register `LOCK` reader"]
pub type R = crate::R<LOCK_SPEC>;
#[doc = "Register `LOCK` writer"]
pub type W = crate::W<LOCK_SPEC>;
#[doc = "Field `USE_EFUSE_KEY_LOCK` reader - Write 1 to lock reg_use_efuse_key. Each bit locks the corresponding bit of reg_use_efuse_key."]
pub type USE_EFUSE_KEY_LOCK_R = crate::FieldReader;
#[doc = "Field `USE_EFUSE_KEY_LOCK` writer - Write 1 to lock reg_use_efuse_key. Each bit locks the corresponding bit of reg_use_efuse_key."]
pub type USE_EFUSE_KEY_LOCK_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RND_SWITCH_CYCLE_LOCK` reader - Write 1 to lock reg_rnd_switch_cycle."]
pub type RND_SWITCH_CYCLE_LOCK_R = crate::BitReader;
#[doc = "Field `RND_SWITCH_CYCLE_LOCK` writer - Write 1 to lock reg_rnd_switch_cycle."]
pub type RND_SWITCH_CYCLE_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USE_SW_INIT_KEY_LOCK` reader - Write 1 to lock reg_use_sw_init_key."]
pub type USE_SW_INIT_KEY_LOCK_R = crate::BitReader;
#[doc = "Field `USE_SW_INIT_KEY_LOCK` writer - Write 1 to lock reg_use_sw_init_key."]
pub type USE_SW_INIT_KEY_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_KEY_LEN_LOCK` reader - Write 1 to lock reg_flash_key_len."]
pub type FLASH_KEY_LEN_LOCK_R = crate::BitReader;
#[doc = "Field `FLASH_KEY_LEN_LOCK` writer - Write 1 to lock reg_flash_key_len."]
pub type FLASH_KEY_LEN_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_KEY_LEN_LOCK` reader - Write 1 to lock reg_psram_key_len."]
pub type PSRAM_KEY_LEN_LOCK_R = crate::BitReader;
#[doc = "Field `PSRAM_KEY_LEN_LOCK` writer - Write 1 to lock reg_psram_key_len."]
pub type PSRAM_KEY_LEN_LOCK_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Write 1 to lock reg_use_efuse_key. Each bit locks the corresponding bit of reg_use_efuse_key."]
    #[inline(always)]
    pub fn use_efuse_key_lock(&self) -> USE_EFUSE_KEY_LOCK_R {
        USE_EFUSE_KEY_LOCK_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - Write 1 to lock reg_rnd_switch_cycle."]
    #[inline(always)]
    pub fn rnd_switch_cycle_lock(&self) -> RND_SWITCH_CYCLE_LOCK_R {
        RND_SWITCH_CYCLE_LOCK_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write 1 to lock reg_use_sw_init_key."]
    #[inline(always)]
    pub fn use_sw_init_key_lock(&self) -> USE_SW_INIT_KEY_LOCK_R {
        USE_SW_INIT_KEY_LOCK_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write 1 to lock reg_flash_key_len."]
    #[inline(always)]
    pub fn flash_key_len_lock(&self) -> FLASH_KEY_LEN_LOCK_R {
        FLASH_KEY_LEN_LOCK_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to lock reg_psram_key_len."]
    #[inline(always)]
    pub fn psram_key_len_lock(&self) -> PSRAM_KEY_LEN_LOCK_R {
        PSRAM_KEY_LEN_LOCK_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOCK")
            .field("use_efuse_key_lock", &self.use_efuse_key_lock())
            .field("rnd_switch_cycle_lock", &self.rnd_switch_cycle_lock())
            .field("use_sw_init_key_lock", &self.use_sw_init_key_lock())
            .field("flash_key_len_lock", &self.flash_key_len_lock())
            .field("psram_key_len_lock", &self.psram_key_len_lock())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Write 1 to lock reg_use_efuse_key. Each bit locks the corresponding bit of reg_use_efuse_key."]
    #[inline(always)]
    pub fn use_efuse_key_lock(&mut self) -> USE_EFUSE_KEY_LOCK_W<'_, LOCK_SPEC> {
        USE_EFUSE_KEY_LOCK_W::new(self, 0)
    }
    #[doc = "Bit 5 - Write 1 to lock reg_rnd_switch_cycle."]
    #[inline(always)]
    pub fn rnd_switch_cycle_lock(&mut self) -> RND_SWITCH_CYCLE_LOCK_W<'_, LOCK_SPEC> {
        RND_SWITCH_CYCLE_LOCK_W::new(self, 5)
    }
    #[doc = "Bit 6 - Write 1 to lock reg_use_sw_init_key."]
    #[inline(always)]
    pub fn use_sw_init_key_lock(&mut self) -> USE_SW_INIT_KEY_LOCK_W<'_, LOCK_SPEC> {
        USE_SW_INIT_KEY_LOCK_W::new(self, 6)
    }
    #[doc = "Bit 7 - Write 1 to lock reg_flash_key_len."]
    #[inline(always)]
    pub fn flash_key_len_lock(&mut self) -> FLASH_KEY_LEN_LOCK_W<'_, LOCK_SPEC> {
        FLASH_KEY_LEN_LOCK_W::new(self, 7)
    }
    #[doc = "Bit 8 - Write 1 to lock reg_psram_key_len."]
    #[inline(always)]
    pub fn psram_key_len_lock(&mut self) -> PSRAM_KEY_LEN_LOCK_W<'_, LOCK_SPEC> {
        PSRAM_KEY_LEN_LOCK_W::new(self, 8)
    }
}
#[doc = "Key Manager static configuration locker register\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LOCK_SPEC;
impl crate::RegisterSpec for LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lock::R`](R) reader structure"]
impl crate::Readable for LOCK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lock::W`](W) writer structure"]
impl crate::Writable for LOCK_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LOCK to value 0"]
impl crate::Resettable for LOCK_SPEC {}
