#[doc = "Register `AHB_DMA_DATE` reader"]
pub type R = crate::R<AHB_DMA_DATE_SPEC>;
#[doc = "Register `AHB_DMA_DATE` writer"]
pub type W = crate::W<AHB_DMA_DATE_SPEC>;
#[doc = "Field `AHB_DMA_DATE` reader - Version control register."]
pub type AHB_DMA_DATE_R = crate::FieldReader<u32>;
#[doc = "Field `AHB_DMA_DATE` writer - Version control register."]
pub type AHB_DMA_DATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Version control register."]
    #[inline(always)]
    pub fn ahb_dma_date(&self) -> AHB_DMA_DATE_R {
        AHB_DMA_DATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_DATE")
            .field("ahb_dma_date", &self.ahb_dma_date())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Version control register."]
    #[inline(always)]
    pub fn ahb_dma_date(&mut self) -> AHB_DMA_DATE_W<'_, AHB_DMA_DATE_SPEC> {
        AHB_DMA_DATE_W::new(self, 0)
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_date::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_date::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_DATE_SPEC;
impl crate::RegisterSpec for AHB_DMA_DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_date::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_DATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_date::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_DATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_DATE to value 0x0250_5230"]
impl crate::Resettable for AHB_DMA_DATE_SPEC {
    const RESET_VALUE: u32 = 0x0250_5230;
}
