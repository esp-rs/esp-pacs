#[doc = "Register `CONF` reader"]
pub type R = crate::R<CONF_SPEC>;
#[doc = "Register `CONF` writer"]
pub type W = crate::W<CONF_SPEC>;
#[doc = "Field `KGEN_MODE` reader - Set this field to choose the key generator deployment mode. 0: random mode. 1: AES mode. 2: ECDH0 mode. 3: ECDH1 mode. 4: recover mode. 5: export mode. 6-7: reserved."]
pub type KGEN_MODE_R = crate::FieldReader;
#[doc = "Field `KGEN_MODE` writer - Set this field to choose the key generator deployment mode. 0: random mode. 1: AES mode. 2: ECDH0 mode. 3: ECDH1 mode. 4: recover mode. 5: export mode. 6-7: reserved."]
pub type KGEN_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `KEY_PURPOSE` reader - Set this field to choose the key purpose. 1: ecdsa_key 2: flash_256_1_key. 3: flash_256_2_key. 4: flash_128_key. 6: hmac_key. 7: ds_key. 8: psram_256_1_key. 9: psram_256_2_key. 10: psram_128_key. Others: reserved."]
pub type KEY_PURPOSE_R = crate::FieldReader;
#[doc = "Field `KEY_PURPOSE` writer - Set this field to choose the key purpose. 1: ecdsa_key 2: flash_256_1_key. 3: flash_256_2_key. 4: flash_128_key. 6: hmac_key. 7: ds_key. 8: psram_256_1_key. 9: psram_256_2_key. 10: psram_128_key. Others: reserved."]
pub type KEY_PURPOSE_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:2 - Set this field to choose the key generator deployment mode. 0: random mode. 1: AES mode. 2: ECDH0 mode. 3: ECDH1 mode. 4: recover mode. 5: export mode. 6-7: reserved."]
    #[inline(always)]
    pub fn kgen_mode(&self) -> KGEN_MODE_R {
        KGEN_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:6 - Set this field to choose the key purpose. 1: ecdsa_key 2: flash_256_1_key. 3: flash_256_2_key. 4: flash_128_key. 6: hmac_key. 7: ds_key. 8: psram_256_1_key. 9: psram_256_2_key. 10: psram_128_key. Others: reserved."]
    #[inline(always)]
    pub fn key_purpose(&self) -> KEY_PURPOSE_R {
        KEY_PURPOSE_R::new(((self.bits >> 3) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF")
            .field("kgen_mode", &self.kgen_mode())
            .field("key_purpose", &self.key_purpose())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - Set this field to choose the key generator deployment mode. 0: random mode. 1: AES mode. 2: ECDH0 mode. 3: ECDH1 mode. 4: recover mode. 5: export mode. 6-7: reserved."]
    #[inline(always)]
    pub fn kgen_mode(&mut self) -> KGEN_MODE_W<CONF_SPEC> {
        KGEN_MODE_W::new(self, 0)
    }
    #[doc = "Bits 3:6 - Set this field to choose the key purpose. 1: ecdsa_key 2: flash_256_1_key. 3: flash_256_2_key. 4: flash_128_key. 6: hmac_key. 7: ds_key. 8: psram_256_1_key. 9: psram_256_2_key. 10: psram_128_key. Others: reserved."]
    #[inline(always)]
    pub fn key_purpose(&mut self) -> KEY_PURPOSE_W<CONF_SPEC> {
        KEY_PURPOSE_W::new(self, 3)
    }
}
#[doc = "Key Manager configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF_SPEC;
impl crate::RegisterSpec for CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf::R`](R) reader structure"]
impl crate::Readable for CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf::W`](W) writer structure"]
impl crate::Writable for CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF to value 0"]
impl crate::Resettable for CONF_SPEC {}
