#[doc = "Register `AHB_DMA_WEIGHT_EN` reader"]
pub type R = crate::R<AHB_DMA_WEIGHT_EN_SPEC>;
#[doc = "Register `AHB_DMA_WEIGHT_EN` writer"]
pub type W = crate::W<AHB_DMA_WEIGHT_EN_SPEC>;
#[doc = "Field `AHB_DMA_WEIGHT_EN` reader - Configures whether to enable weight arbitration.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_WEIGHT_EN_R = crate::BitReader;
#[doc = "Field `AHB_DMA_WEIGHT_EN` writer - Configures whether to enable weight arbitration.\\\\0: Disable\\\\1: Enable\\\\"]
pub type AHB_DMA_WEIGHT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether to enable weight arbitration.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_weight_en(&self) -> AHB_DMA_WEIGHT_EN_R {
        AHB_DMA_WEIGHT_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_WEIGHT_EN")
            .field("ahb_dma_weight_en", &self.ahb_dma_weight_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether to enable weight arbitration.\\\\0: Disable\\\\1: Enable\\\\"]
    #[inline(always)]
    pub fn ahb_dma_weight_en(&mut self) -> AHB_DMA_WEIGHT_EN_W<'_, AHB_DMA_WEIGHT_EN_SPEC> {
        AHB_DMA_WEIGHT_EN_W::new(self, 0)
    }
}
#[doc = "TX weight arbitration enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_weight_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_weight_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_WEIGHT_EN_SPEC;
impl crate::RegisterSpec for AHB_DMA_WEIGHT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_weight_en::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_WEIGHT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_weight_en::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_WEIGHT_EN_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_WEIGHT_EN to value 0"]
impl crate::Resettable for AHB_DMA_WEIGHT_EN_SPEC {}
