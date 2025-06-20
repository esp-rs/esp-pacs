#[doc = "Register `STATIC` reader"]
pub type R = crate::R<STATIC_SPEC>;
#[doc = "Register `STATIC` writer"]
pub type W = crate::W<STATIC_SPEC>;
#[doc = "Field `USE_EFUSE_KEY` reader - Set each bit to choose efuse key instead of key manager deployed key. Each bit stands for a key type:bit 4 for psram_key; bit 3 for ds_key; bit 2 for hmac_key; bit 1 for flash_key; bit 0 for ecdsa_key"]
pub type USE_EFUSE_KEY_R = crate::FieldReader;
#[doc = "Field `USE_EFUSE_KEY` writer - Set each bit to choose efuse key instead of key manager deployed key. Each bit stands for a key type:bit 4 for psram_key; bit 3 for ds_key; bit 2 for hmac_key; bit 1 for flash_key; bit 0 for ecdsa_key"]
pub type USE_EFUSE_KEY_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `RND_SWITCH_CYCLE` reader - The core clock cycle number to sample one rng input data. Please set it bigger than the clock cycle ratio: T_rng/T_km"]
pub type RND_SWITCH_CYCLE_R = crate::FieldReader;
#[doc = "Field `RND_SWITCH_CYCLE` writer - The core clock cycle number to sample one rng input data. Please set it bigger than the clock cycle ratio: T_rng/T_km"]
pub type RND_SWITCH_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `USE_SW_INIT_KEY` reader - Set this bit to use software written init key instead of efuse_init_key."]
pub type USE_SW_INIT_KEY_R = crate::BitReader;
#[doc = "Field `USE_SW_INIT_KEY` writer - Set this bit to use software written init key instead of efuse_init_key."]
pub type USE_SW_INIT_KEY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASH_KEY_LEN` reader - Set this bit to choose flash crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
pub type FLASH_KEY_LEN_R = crate::BitReader;
#[doc = "Field `FLASH_KEY_LEN` writer - Set this bit to choose flash crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
pub type FLASH_KEY_LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSRAM_KEY_LEN` reader - Set this bit to choose psram crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
pub type PSRAM_KEY_LEN_R = crate::BitReader;
#[doc = "Field `PSRAM_KEY_LEN` writer - Set this bit to choose psram crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
pub type PSRAM_KEY_LEN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Set each bit to choose efuse key instead of key manager deployed key. Each bit stands for a key type:bit 4 for psram_key; bit 3 for ds_key; bit 2 for hmac_key; bit 1 for flash_key; bit 0 for ecdsa_key"]
    #[inline(always)]
    pub fn use_efuse_key(&self) -> USE_EFUSE_KEY_R {
        USE_EFUSE_KEY_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 5:9 - The core clock cycle number to sample one rng input data. Please set it bigger than the clock cycle ratio: T_rng/T_km"]
    #[inline(always)]
    pub fn rnd_switch_cycle(&self) -> RND_SWITCH_CYCLE_R {
        RND_SWITCH_CYCLE_R::new(((self.bits >> 5) & 0x1f) as u8)
    }
    #[doc = "Bit 10 - Set this bit to use software written init key instead of efuse_init_key."]
    #[inline(always)]
    pub fn use_sw_init_key(&self) -> USE_SW_INIT_KEY_R {
        USE_SW_INIT_KEY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Set this bit to choose flash crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
    #[inline(always)]
    pub fn flash_key_len(&self) -> FLASH_KEY_LEN_R {
        FLASH_KEY_LEN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to choose psram crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
    #[inline(always)]
    pub fn psram_key_len(&self) -> PSRAM_KEY_LEN_R {
        PSRAM_KEY_LEN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATIC")
            .field("use_efuse_key", &self.use_efuse_key())
            .field("rnd_switch_cycle", &self.rnd_switch_cycle())
            .field("use_sw_init_key", &self.use_sw_init_key())
            .field("flash_key_len", &self.flash_key_len())
            .field("psram_key_len", &self.psram_key_len())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:4 - Set each bit to choose efuse key instead of key manager deployed key. Each bit stands for a key type:bit 4 for psram_key; bit 3 for ds_key; bit 2 for hmac_key; bit 1 for flash_key; bit 0 for ecdsa_key"]
    #[inline(always)]
    pub fn use_efuse_key(&mut self) -> USE_EFUSE_KEY_W<STATIC_SPEC> {
        USE_EFUSE_KEY_W::new(self, 0)
    }
    #[doc = "Bits 5:9 - The core clock cycle number to sample one rng input data. Please set it bigger than the clock cycle ratio: T_rng/T_km"]
    #[inline(always)]
    pub fn rnd_switch_cycle(&mut self) -> RND_SWITCH_CYCLE_W<STATIC_SPEC> {
        RND_SWITCH_CYCLE_W::new(self, 5)
    }
    #[doc = "Bit 10 - Set this bit to use software written init key instead of efuse_init_key."]
    #[inline(always)]
    pub fn use_sw_init_key(&mut self) -> USE_SW_INIT_KEY_W<STATIC_SPEC> {
        USE_SW_INIT_KEY_W::new(self, 10)
    }
    #[doc = "Bit 11 - Set this bit to choose flash crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
    #[inline(always)]
    pub fn flash_key_len(&mut self) -> FLASH_KEY_LEN_W<STATIC_SPEC> {
        FLASH_KEY_LEN_W::new(self, 11)
    }
    #[doc = "Bit 12 - Set this bit to choose psram crypt using xts-aes-256 or xts-aes-128. 1: use xts-aes-256. 0: use xts-aes-128."]
    #[inline(always)]
    pub fn psram_key_len(&mut self) -> PSRAM_KEY_LEN_W<STATIC_SPEC> {
        PSRAM_KEY_LEN_W::new(self, 12)
    }
}
#[doc = "Key Manager static configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`static_::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`static_::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATIC_SPEC;
impl crate::RegisterSpec for STATIC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`static_::R`](R) reader structure"]
impl crate::Readable for STATIC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`static_::W`](W) writer structure"]
impl crate::Writable for STATIC_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets STATIC to value 0x01e0"]
impl crate::Resettable for STATIC_SPEC {
    const RESET_VALUE: u32 = 0x01e0;
}
