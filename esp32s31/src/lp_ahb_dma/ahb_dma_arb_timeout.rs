#[doc = "Register `AHB_DMA_ARB_TIMEOUT` reader"]
pub type R = crate::R<AHB_DMA_ARB_TIMEOUT_SPEC>;
#[doc = "Register `AHB_DMA_ARB_TIMEOUT` writer"]
pub type W = crate::W<AHB_DMA_ARB_TIMEOUT_SPEC>;
#[doc = "Field `AHB_DMA_ARB_TIMEOUT` reader - Configures the time slot. Measurement unit: AHB bus clock cycle."]
pub type AHB_DMA_ARB_TIMEOUT_R = crate::FieldReader<u16>;
#[doc = "Field `AHB_DMA_ARB_TIMEOUT` writer - Configures the time slot. Measurement unit: AHB bus clock cycle."]
pub type AHB_DMA_ARB_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the time slot. Measurement unit: AHB bus clock cycle."]
    #[inline(always)]
    pub fn ahb_dma_arb_timeout(&self) -> AHB_DMA_ARB_TIMEOUT_R {
        AHB_DMA_ARB_TIMEOUT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_ARB_TIMEOUT")
            .field("ahb_dma_arb_timeout", &self.ahb_dma_arb_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the time slot. Measurement unit: AHB bus clock cycle."]
    #[inline(always)]
    pub fn ahb_dma_arb_timeout(&mut self) -> AHB_DMA_ARB_TIMEOUT_W<'_, AHB_DMA_ARB_TIMEOUT_SPEC> {
        AHB_DMA_ARB_TIMEOUT_W::new(self, 0)
    }
}
#[doc = "TX arbitration timeout configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_arb_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_arb_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_ARB_TIMEOUT_SPEC;
impl crate::RegisterSpec for AHB_DMA_ARB_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_arb_timeout::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_ARB_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_arb_timeout::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_ARB_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_ARB_TIMEOUT to value 0"]
impl crate::Resettable for AHB_DMA_ARB_TIMEOUT_SPEC {}
