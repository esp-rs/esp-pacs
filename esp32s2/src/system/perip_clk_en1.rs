#[doc = "Register `PERIP_CLK_EN1` reader"]
pub type R = crate::R<PERIP_CLK_EN1_SPEC>;
#[doc = "Register `PERIP_CLK_EN1` writer"]
pub type W = crate::W<PERIP_CLK_EN1_SPEC>;
#[doc = "Field `CRYPTO_AES_CLK_EN` reader - Set this bit to enable clock of cryptography AES."]
pub type CRYPTO_AES_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_CLK_EN` writer - Set this bit to enable clock of cryptography AES."]
pub type CRYPTO_AES_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - Set this bit to enable clock of cryptography SHA."]
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - Set this bit to enable clock of cryptography SHA."]
pub type CRYPTO_SHA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_RSA_CLK_EN` reader - Set this bit to enable clock of cryptography RSA."]
pub type CRYPTO_RSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_CLK_EN` writer - Set this bit to enable clock of cryptography RSA."]
pub type CRYPTO_RSA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DS_CLK_EN` reader - Set this bit to enable clock of cryptography Digital Signature."]
pub type CRYPTO_DS_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_CLK_EN` writer - Set this bit to enable clock of cryptography Digital Signature."]
pub type CRYPTO_DS_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` reader - Set this bit to enable clock of cryptography HMAC."]
pub type CRYPTO_HMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` writer - Set this bit to enable clock of cryptography HMAC."]
pub type CRYPTO_HMAC_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_DMA_CLK_EN` reader - Set this bit to enable clock of cryptography DMA."]
pub type CRYPTO_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DMA_CLK_EN` writer - Set this bit to enable clock of cryptography DMA."]
pub type CRYPTO_DMA_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to enable clock of cryptography AES."]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&self) -> CRYPTO_AES_CLK_EN_R {
        CRYPTO_AES_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable clock of cryptography SHA."]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable clock of cryptography RSA."]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&self) -> CRYPTO_RSA_CLK_EN_R {
        CRYPTO_RSA_CLK_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable clock of cryptography Digital Signature."]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&self) -> CRYPTO_DS_CLK_EN_R {
        CRYPTO_DS_CLK_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable clock of cryptography HMAC."]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&self) -> CRYPTO_HMAC_CLK_EN_R {
        CRYPTO_HMAC_CLK_EN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable clock of cryptography DMA."]
    #[inline(always)]
    pub fn crypto_dma_clk_en(&self) -> CRYPTO_DMA_CLK_EN_R {
        CRYPTO_DMA_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_CLK_EN1")
            .field("crypto_aes_clk_en", &self.crypto_aes_clk_en())
            .field("crypto_sha_clk_en", &self.crypto_sha_clk_en())
            .field("crypto_rsa_clk_en", &self.crypto_rsa_clk_en())
            .field("crypto_ds_clk_en", &self.crypto_ds_clk_en())
            .field("crypto_hmac_clk_en", &self.crypto_hmac_clk_en())
            .field("crypto_dma_clk_en", &self.crypto_dma_clk_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to enable clock of cryptography AES."]
    #[inline(always)]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_AES_CLK_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to enable clock of cryptography SHA."]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_SHA_CLK_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to enable clock of cryptography RSA."]
    #[inline(always)]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_RSA_CLK_EN_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to enable clock of cryptography Digital Signature."]
    #[inline(always)]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_DS_CLK_EN_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to enable clock of cryptography HMAC."]
    #[inline(always)]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_HMAC_CLK_EN_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to enable clock of cryptography DMA."]
    #[inline(always)]
    pub fn crypto_dma_clk_en(&mut self) -> CRYPTO_DMA_CLK_EN_W<PERIP_CLK_EN1_SPEC> {
        CRYPTO_DMA_CLK_EN_W::new(self, 6)
    }
}
#[doc = "System peripheral clock (for hardware accelerators) enable register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`perip_clk_en1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`perip_clk_en1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_CLK_EN1_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_clk_en1::R`](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_clk_en1::W`](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIP_CLK_EN1 to value 0"]
impl crate::Resettable for PERIP_CLK_EN1_SPEC {
    const RESET_VALUE: u32 = 0;
}
