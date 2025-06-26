#[doc = "Register `PERIP_RST_EN1` reader"]
pub type R = crate::R<PERIP_RST_EN1_SPEC>;
#[doc = "Register `PERIP_RST_EN1` writer"]
pub type W = crate::W<PERIP_RST_EN1_SPEC>;
#[doc = "Field `CRYPTO_AES_RST` reader - Set this bit to reset cryptography AES."]
pub type CRYPTO_AES_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_RST` writer - Set this bit to reset cryptography AES."]
pub type CRYPTO_AES_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_RST` reader - Set this bit to reset cryptography SHA."]
pub type CRYPTO_SHA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_RST` writer - Set this bit to reset cryptography SHA."]
pub type CRYPTO_SHA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_RST` reader - Set this bit to reset cryptography RSA."]
pub type CRYPTO_RSA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_RST` writer - Set this bit to reset cryptography RSA."]
pub type CRYPTO_RSA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_RST` reader - Set this bit to reset cryptography digital signature."]
pub type CRYPTO_DS_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_RST` writer - Set this bit to reset cryptography digital signature."]
pub type CRYPTO_DS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_RST` reader - Set this bit to reset cryptography HMAC."]
pub type CRYPTO_HMAC_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_RST` writer - Set this bit to reset cryptography HMAC."]
pub type CRYPTO_HMAC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DMA_RST` reader - Set this bit to reset cryptography DMA."]
pub type CRYPTO_DMA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_DMA_RST` writer - Set this bit to reset cryptography DMA."]
pub type CRYPTO_DMA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to reset cryptography AES."]
    #[inline(always)]
    pub fn crypto_aes_rst(&self) -> CRYPTO_AES_RST_R {
        CRYPTO_AES_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset cryptography SHA."]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset cryptography RSA."]
    #[inline(always)]
    pub fn crypto_rsa_rst(&self) -> CRYPTO_RSA_RST_R {
        CRYPTO_RSA_RST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to reset cryptography digital signature."]
    #[inline(always)]
    pub fn crypto_ds_rst(&self) -> CRYPTO_DS_RST_R {
        CRYPTO_DS_RST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to reset cryptography HMAC."]
    #[inline(always)]
    pub fn crypto_hmac_rst(&self) -> CRYPTO_HMAC_RST_R {
        CRYPTO_HMAC_RST_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to reset cryptography DMA."]
    #[inline(always)]
    pub fn crypto_dma_rst(&self) -> CRYPTO_DMA_RST_R {
        CRYPTO_DMA_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN1")
            .field("crypto_aes_rst", &self.crypto_aes_rst())
            .field("crypto_sha_rst", &self.crypto_sha_rst())
            .field("crypto_rsa_rst", &self.crypto_rsa_rst())
            .field("crypto_ds_rst", &self.crypto_ds_rst())
            .field("crypto_hmac_rst", &self.crypto_hmac_rst())
            .field("crypto_dma_rst", &self.crypto_dma_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to reset cryptography AES."]
    #[inline(always)]
    pub fn crypto_aes_rst(&mut self) -> CRYPTO_AES_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_AES_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to reset cryptography SHA."]
    #[inline(always)]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_SHA_RST_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to reset cryptography RSA."]
    #[inline(always)]
    pub fn crypto_rsa_rst(&mut self) -> CRYPTO_RSA_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_RSA_RST_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to reset cryptography digital signature."]
    #[inline(always)]
    pub fn crypto_ds_rst(&mut self) -> CRYPTO_DS_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_DS_RST_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to reset cryptography HMAC."]
    #[inline(always)]
    pub fn crypto_hmac_rst(&mut self) -> CRYPTO_HMAC_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_HMAC_RST_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to reset cryptography DMA."]
    #[inline(always)]
    pub fn crypto_dma_rst(&mut self) -> CRYPTO_DMA_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_DMA_RST_W::new(self, 6)
    }
}
#[doc = "System peripheral (hardware accelerators) reset register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_rst_en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_rst_en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_rst_en1::R`](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_rst_en1::W`](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0x7e"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {
    const RESET_VALUE: u32 = 0x7e;
}
