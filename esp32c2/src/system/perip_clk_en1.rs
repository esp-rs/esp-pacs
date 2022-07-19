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
#[doc = "Field `CRYPTO_ECC_CLK_EN` reader - Set 1 to enable ECC clock"]
pub type CRYPTO_ECC_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_ECC_CLK_EN` writer - Set 1 to enable ECC clock"]
pub type CRYPTO_ECC_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN1_SPEC, bool, 1>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` reader - Set 1 to enable SHA clock"]
pub type CRYPTO_SHA_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_SHA_CLK_EN` writer - Set 1 to enable SHA clock"]
pub type CRYPTO_SHA_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN1_SPEC, bool, 2>;
#[doc = "Field `DMA_CLK_EN` reader - Set 1 to enable DMA clock"]
pub type DMA_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `DMA_CLK_EN` writer - Set 1 to enable DMA clock"]
pub type DMA_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN1_SPEC, bool, 6>;
#[doc = "Field `TSENS_CLK_EN` reader - Set 1 to enable TSENS clock"]
pub type TSENS_CLK_EN_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_CLK_EN` writer - Set 1 to enable TSENS clock"]
pub type TSENS_CLK_EN_W<'a> = crate::BitWriter<'a, u32, PERIP_CLK_EN1_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 1 - Set 1 to enable ECC clock"]
    #[inline(always)]
    pub fn crypto_ecc_clk_en(&self) -> CRYPTO_ECC_CLK_EN_R {
        CRYPTO_ECC_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to enable SHA clock"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&self) -> CRYPTO_SHA_CLK_EN_R {
        CRYPTO_SHA_CLK_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to enable DMA clock"]
    #[inline(always)]
    pub fn dma_clk_en(&self) -> DMA_CLK_EN_R {
        DMA_CLK_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Set 1 to enable TSENS clock"]
    #[inline(always)]
    pub fn tsens_clk_en(&self) -> TSENS_CLK_EN_R {
        TSENS_CLK_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set 1 to enable ECC clock"]
    #[inline(always)]
    pub fn crypto_ecc_clk_en(&mut self) -> CRYPTO_ECC_CLK_EN_W {
        CRYPTO_ECC_CLK_EN_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to enable SHA clock"]
    #[inline(always)]
    pub fn crypto_sha_clk_en(&mut self) -> CRYPTO_SHA_CLK_EN_W {
        CRYPTO_SHA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to enable DMA clock"]
    #[inline(always)]
    pub fn dma_clk_en(&mut self) -> DMA_CLK_EN_W {
        DMA_CLK_EN_W::new(self)
    }
    #[doc = "Bit 10 - Set 1 to enable TSENS clock"]
    #[inline(always)]
    pub fn tsens_clk_en(&mut self) -> TSENS_CLK_EN_W {
        TSENS_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral clock gating register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_clk_en1](index.html) module"]
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
}
#[doc = "`reset()` method sets PERIP_CLK_EN1 to value 0"]
impl crate::Resettable for PERIP_CLK_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
