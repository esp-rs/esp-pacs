#[doc = "Register `PERI_RST_EN` reader"]
pub type R = crate::R<PERI_RST_EN_SPEC>;
#[doc = "Register `PERI_RST_EN` writer"]
pub type W = crate::W<PERI_RST_EN_SPEC>;
#[doc = "Field `CRYPTO_AES_RST` reader - Set the bit to reset AES module. Clear the bit to release AES module."]
pub type CRYPTO_AES_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_RST` writer - Set the bit to reset AES module. Clear the bit to release AES module."]
pub type CRYPTO_AES_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_RST` reader - Set the bit to reset SHA module. Clear the bit to release SHA module."]
pub type CRYPTO_SHA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_RST` writer - Set the bit to reset SHA module. Clear the bit to release SHA module."]
pub type CRYPTO_SHA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_RST` reader - Set the bit to reset RSA module. Clear the bit to release RSA module."]
pub type CRYPTO_RSA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_RST` writer - Set the bit to reset RSA module. Clear the bit to release RSA module."]
pub type CRYPTO_RSA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set the bit to reset AES module. Clear the bit to release AES module."]
    #[inline(always)]
    pub fn crypto_aes_rst(&self) -> CRYPTO_AES_RST_R {
        CRYPTO_AES_RST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set the bit to reset SHA module. Clear the bit to release SHA module."]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set the bit to reset RSA module. Clear the bit to release RSA module."]
    #[inline(always)]
    pub fn crypto_rsa_rst(&self) -> CRYPTO_RSA_RST_R {
        CRYPTO_RSA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_RST_EN")
            .field("crypto_aes_rst", &self.crypto_aes_rst())
            .field("crypto_sha_rst", &self.crypto_sha_rst())
            .field("crypto_rsa_rst", &self.crypto_rsa_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set the bit to reset AES module. Clear the bit to release AES module."]
    #[inline(always)]
    pub fn crypto_aes_rst(&mut self) -> CRYPTO_AES_RST_W<'_, PERI_RST_EN_SPEC> {
        CRYPTO_AES_RST_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set the bit to reset SHA module. Clear the bit to release SHA module."]
    #[inline(always)]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W<'_, PERI_RST_EN_SPEC> {
        CRYPTO_SHA_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set the bit to reset RSA module. Clear the bit to release RSA module."]
    #[inline(always)]
    pub fn crypto_rsa_rst(&mut self) -> CRYPTO_RSA_RST_W<'_, PERI_RST_EN_SPEC> {
        CRYPTO_RSA_RST_W::new(self, 2)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`peri_rst_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`peri_rst_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERI_RST_EN_SPEC;
impl crate::RegisterSpec for PERI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`peri_rst_en::R`](R) reader structure"]
impl crate::Readable for PERI_RST_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`peri_rst_en::W`](W) writer structure"]
impl crate::Writable for PERI_RST_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERI_RST_EN to value 0"]
impl crate::Resettable for PERI_RST_EN_SPEC {}
