#[doc = "Register `PERIP_RST_EN1` reader"]
pub type R = crate::R<PERIP_RST_EN1_SPEC>;
#[doc = "Register `PERIP_RST_EN1` writer"]
pub type W = crate::W<PERIP_RST_EN1_SPEC>;
#[doc = "Field `CRYPTO_ECC_RST` reader - Set 1 to let CRYPTO_ECC reset"]
pub type CRYPTO_ECC_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_ECC_RST` writer - Set 1 to let CRYPTO_ECC reset"]
pub type CRYPTO_ECC_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CRYPTO_SHA_RST` reader - Set 1 to let CRYPTO_SHA reset"]
pub type CRYPTO_SHA_RST_R = crate::BitReader;
#[doc = "Field `CRYPTO_SHA_RST` writer - Set 1 to let CRYPTO_SHA reset"]
pub type CRYPTO_SHA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_RST` reader - Set 1 to let DMA reset"]
pub type DMA_RST_R = crate::BitReader;
#[doc = "Field `DMA_RST` writer - Set 1 to let DMA reset"]
pub type DMA_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENS_RST` reader - Set 1 to let TSENS reset"]
pub type TSENS_RST_R = crate::BitReader;
#[doc = "Field `TSENS_RST` writer - Set 1 to let TSENS reset"]
pub type TSENS_RST_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERIP_RST_EN1")
            .field("crypto_ecc_rst", &self.crypto_ecc_rst())
            .field("crypto_sha_rst", &self.crypto_sha_rst())
            .field("dma_rst", &self.dma_rst())
            .field("tsens_rst", &self.tsens_rst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set 1 to let CRYPTO_ECC reset"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_ecc_rst(&mut self) -> CRYPTO_ECC_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_ECC_RST_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set 1 to let CRYPTO_SHA reset"]
    #[inline(always)]
    #[must_use]
    pub fn crypto_sha_rst(&mut self) -> CRYPTO_SHA_RST_W<PERIP_RST_EN1_SPEC> {
        CRYPTO_SHA_RST_W::new(self, 2)
    }
    #[doc = "Bit 6 - Set 1 to let DMA reset"]
    #[inline(always)]
    #[must_use]
    pub fn dma_rst(&mut self) -> DMA_RST_W<PERIP_RST_EN1_SPEC> {
        DMA_RST_W::new(self, 6)
    }
    #[doc = "Bit 10 - Set 1 to let TSENS reset"]
    #[inline(always)]
    #[must_use]
    pub fn tsens_rst(&mut self) -> TSENS_RST_W<PERIP_RST_EN1_SPEC> {
        TSENS_RST_W::new(self, 10)
    }
}
#[doc = "peripheral reset register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`perip_rst_en1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`perip_rst_en1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PERIP_RST_EN1_SPEC;
impl crate::RegisterSpec for PERIP_RST_EN1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`perip_rst_en1::R`](R) reader structure"]
impl crate::Readable for PERIP_RST_EN1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`perip_rst_en1::W`](W) writer structure"]
impl crate::Writable for PERIP_RST_EN1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PERIP_RST_EN1 to value 0x46"]
impl crate::Resettable for PERIP_RST_EN1_SPEC {
    const RESET_VALUE: u32 = 0x46;
}
