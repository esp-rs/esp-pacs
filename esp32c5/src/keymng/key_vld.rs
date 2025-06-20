#[doc = "Register `KEY_VLD` reader"]
pub type R = crate::R<KEY_VLD_SPEC>;
#[doc = "Field `KEY_ECDSA_VLD` reader - The status bit for key_ecdsa. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
pub type KEY_ECDSA_VLD_R = crate::BitReader;
#[doc = "Field `KEY_FLASH_VLD` reader - The status bit for key_flash. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
pub type KEY_FLASH_VLD_R = crate::BitReader;
#[doc = "Field `KEY_HMAC_VLD` reader - The status bit for key_hmac. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
pub type KEY_HMAC_VLD_R = crate::BitReader;
#[doc = "Field `KEY_DS_VLD` reader - The status bit for key_ds. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
pub type KEY_DS_VLD_R = crate::BitReader;
#[doc = "Field `KEY_PSRAM_VLD` reader - The status bit for key_psram. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
pub type KEY_PSRAM_VLD_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The status bit for key_ecdsa. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
    #[inline(always)]
    pub fn key_ecdsa_vld(&self) -> KEY_ECDSA_VLD_R {
        KEY_ECDSA_VLD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The status bit for key_flash. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
    #[inline(always)]
    pub fn key_flash_vld(&self) -> KEY_FLASH_VLD_R {
        KEY_FLASH_VLD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The status bit for key_hmac. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
    #[inline(always)]
    pub fn key_hmac_vld(&self) -> KEY_HMAC_VLD_R {
        KEY_HMAC_VLD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The status bit for key_ds. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
    #[inline(always)]
    pub fn key_ds_vld(&self) -> KEY_DS_VLD_R {
        KEY_DS_VLD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The status bit for key_psram. 1: The key has been deployed correctly. 0: The key has not been deployed yet."]
    #[inline(always)]
    pub fn key_psram_vld(&self) -> KEY_PSRAM_VLD_R {
        KEY_PSRAM_VLD_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("KEY_VLD")
            .field("key_ecdsa_vld", &self.key_ecdsa_vld())
            .field("key_flash_vld", &self.key_flash_vld())
            .field("key_hmac_vld", &self.key_hmac_vld())
            .field("key_ds_vld", &self.key_ds_vld())
            .field("key_psram_vld", &self.key_psram_vld())
            .finish()
    }
}
#[doc = "Key Manager key status register\n\nYou can [`read`](crate::Reg::read) this register and get [`key_vld::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct KEY_VLD_SPEC;
impl crate::RegisterSpec for KEY_VLD_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`key_vld::R`](R) reader structure"]
impl crate::Readable for KEY_VLD_SPEC {}
#[doc = "`reset()` method sets KEY_VLD to value 0"]
impl crate::Resettable for KEY_VLD_SPEC {
    const RESET_VALUE: u32 = 0;
}
