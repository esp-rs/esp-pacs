#[doc = "Register `PERIP_RST_EN1` reader"]
pub struct R(crate::R<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERIP_RST_EN1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERIP_RST_EN1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERIP_RST_EN1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERIP_RST_EN1` writer"]
pub struct W(crate::W<PERIP_RST_EN1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERIP_RST_EN1_SPEC>;
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
impl From<crate::W<PERIP_RST_EN1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERIP_RST_EN1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CRYPTO_ECC_RST` reader - Set 1 to let CRYPTO_ECC reset"]
pub type CRYPTO_ECC_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_ECC_RST` writer - Set 1 to let CRYPTO_ECC reset"]
pub type CRYPTO_ECC_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 1>;
#[doc = "Field `CRYPTO_SHA_RST` reader - Set 1 to let CRYPTO_SHA reset"]
pub type CRYPTO_SHA_RST_R = crate::BitReader<bool>;
#[doc = "Field `CRYPTO_SHA_RST` writer - Set 1 to let CRYPTO_SHA reset"]
pub type CRYPTO_SHA_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 2>;
#[doc = "Field `DMA_RST` reader - Set 1 to let DMA reset"]
pub type DMA_RST_R = crate::BitReader<bool>;
#[doc = "Field `DMA_RST` writer - Set 1 to let DMA reset"]
pub type DMA_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 6>;
#[doc = "Field `TSENS_RST` reader - Set 1 to let TSENS reset"]
pub type TSENS_RST_R = crate::BitReader<bool>;
#[doc = "Field `TSENS_RST` writer - Set 1 to let TSENS reset"]
pub type TSENS_RST_W<'a> = crate::BitWriter<'a, u32, PERIP_RST_EN1_SPEC, bool, 10>;
impl R {
    #[doc = "Bit 1 - Set 1 to let CRYPTO_ECC reset"]
    #[inline(always)]
    pub fn crypto_ecc_rst(&self) -> CRYPTO_ECC_RST_R {
        CRYPTO_ECC_RST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set 1 to let CRYPTO_SHA reset"]
    #[inline(always)]
    pub fn crypto_sha_rst(&self) -> CRYPTO_SHA_RST_R {
        CRYPTO_SHA_RST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 6 - Set 1 to let DMA reset"]
    #[inline(always)]
    pub fn dma_rst(&self) -> DMA_RST_R {
        DMA_RST_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 10 - Set 1 to let TSENS reset"]
    #[inline(always)]
    pub fn tsens_rst(&self) -> TSENS_RST_R {
        TSENS_RST_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Set 1 to let CRYPTO_ECC reset"]
    #[inline(always)]
    pub fn crypto_ecc_rst(&mut self) -> CRYPTO_ECC_RST_W {
        CRYPTO_ECC_RST_W::new(self)
    }
    #[doc = "Bit 2 - Set 1 to let CRYPTO_SHA reset"]
    #[inline(always)]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W {
        CRYPTO_SHA_RST_W::new(self)
    }
    #[doc = "Bit 6 - Set 1 to let DMA reset"]
    #[inline(always)]
    pub fn dma_rst(&mut self) -> DMA_RST_W {
        DMA_RST_W::new(self)
    }
    #[doc = "Bit 10 - Set 1 to let TSENS reset"]
    #[inline(always)]
    pub fn tsens_rst(&mut self) -> TSENS_RST_W {
        TSENS_RST_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "peripheral reset register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [perip_rst_en1](index.html) module"]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [perip_rst_en1::R](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [perip_rst_en1::W](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0x46"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x46
    }
}
