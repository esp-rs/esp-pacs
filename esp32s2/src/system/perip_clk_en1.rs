#[doc = "Register `PERIP_CLK_EN1` reader"]
pub struct R(crate::R<PERIP_CLK_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_CLK_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_CLK_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_CLK_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_CLK_EN1` writer"]
pub struct W(crate::W<PERIP_CLK_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_CLK_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<PERIP_CLK_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_CLK_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO_AES_CLK_EN` reader - Set this bit to enable clock of cryptography AES."]
pub type CRYPTO_AES_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_AES_CLK_EN` writer - Set this bit to enable clock of cryptography AES."]
pub type CRYPTO_AES_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - Set this bit to enable clock of cryptography SHA."]
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - Set this bit to enable clock of cryptography SHA."]
pub type CRYPTO_SHA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_RSA_CLK_EN` reader - Set this bit to enable clock of cryptography RSA."]
pub type CRYPTO_RSA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_RSA_CLK_EN` writer - Set this bit to enable clock of cryptography RSA."]
pub type CRYPTO_RSA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_DS_CLK_EN` reader - Set this bit to enable clock of cryptography Digital Signature."]
pub type CRYPTO_DS_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DS_CLK_EN` writer - Set this bit to enable clock of cryptography Digital Signature."]
pub type CRYPTO_DS_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` reader - Set this bit to enable clock of cryptography HMAC."]
pub type CRYPTO_HMAC_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_HMAC_CLK_EN` writer - Set this bit to enable clock of cryptography HMAC."]
pub type CRYPTO_HMAC_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
#[doc = "Field `CRYPTO_DMA_CLK_EN` reader - Set this bit to enable clock of cryptography DMA."]
pub type CRYPTO_DMA_CLK_EN_R = crate::BitReader;
#[doc = "Field `CRYPTO_DMA_CLK_EN` writer - Set this bit to enable clock of cryptography DMA."]
pub type CRYPTO_DMA_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, PERIP_CLK_EN1_SPEC, O>;
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
            .field(
                "crypto_aes_clk_en",
                &format_args!("{}", self.crypto_aes_clk_en().bit()),
            )
            .field(
                "crypto_sha_clk_en",
                &format_args!("{}", self.crypto_sha_clk_en().bit()),
            )
            .field(
                "crypto_rsa_clk_en",
                &format_args!("{}", self.crypto_rsa_clk_en().bit()),
            )
            .field(
                "crypto_ds_clk_en",
                &format_args!("{}", self.crypto_ds_clk_en().bit()),
            )
            .field(
                "crypto_hmac_clk_en",
                &format_args!("{}", self.crypto_hmac_clk_en().bit()),
            )
            .field(
                "crypto_dma_clk_en",
                &format_args!("{}", self.crypto_dma_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERIP_CLK_EN1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to enable clock of cryptography AES."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_aes_clk_en(&mut self) -> CRYPTO_AES_CLK_EN_W<1> {
        CRYPTO_AES_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to enable clock of cryptography SHA."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W<2> {
        CRYPTO_SHA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to enable clock of cryptography RSA."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_rsa_clk_en(&mut self) -> CRYPTO_RSA_CLK_EN_W<3> {
        CRYPTO_RSA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to enable clock of cryptography Digital Signature."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ds_clk_en(&mut self) -> CRYPTO_DS_CLK_EN_W<4> {
        CRYPTO_DS_CLK_EN_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to enable clock of cryptography HMAC."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_hmac_clk_en(&mut self) -> CRYPTO_HMAC_CLK_EN_W<5> {
        CRYPTO_HMAC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to enable clock of cryptography DMA."]
    #[inline(always)]
    #[must_use]
    pub fn crypto_dma_clk_en(&mut self) -> CRYPTO_DMA_CLK_EN_W<6> {
        CRYPTO_DMA_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "System peripheral clock (for hardware accelerators) enable register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en1](index.html) module"]
pub struct PERIP_CLK_EN1_SPEC;
impl crate::RegisterSpec for PERIP_CLK_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_clk_en1::R](R) reader structure"]
impl crate::Readable for PERIP_CLK_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_clk_en1::W](W) writer structure"]
impl crate::Writable for PERIP_CLK_EN1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERIP_CLK_EN1 to value 0"]
impl crate::Resettable for PERIP_CLK_EN1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
