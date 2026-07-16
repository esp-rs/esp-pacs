#[doc = "Register `AHB_DMA_INTR_MEM_START_ADDR` reader"]
pub type R = crate::R<AHB_DMA_INTR_MEM_START_ADDR_SPEC>;
#[doc = "Register `AHB_DMA_INTR_MEM_START_ADDR` writer"]
pub type W = crate::W<AHB_DMA_INTR_MEM_START_ADDR_SPEC>;
#[doc = "Field `AHB_DMA_ACCESS_INTR_MEM_START_ADDR` reader - Configures the start address of accessible address space."]
pub type AHB_DMA_ACCESS_INTR_MEM_START_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `AHB_DMA_ACCESS_INTR_MEM_START_ADDR` writer - Configures the start address of accessible address space."]
pub type AHB_DMA_ACCESS_INTR_MEM_START_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the start address of accessible address space."]
    #[inline(always)]
    pub fn ahb_dma_access_intr_mem_start_addr(&self) -> AHB_DMA_ACCESS_INTR_MEM_START_ADDR_R {
        AHB_DMA_ACCESS_INTR_MEM_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_INTR_MEM_START_ADDR")
            .field(
                "ahb_dma_access_intr_mem_start_addr",
                &self.ahb_dma_access_intr_mem_start_addr(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the start address of accessible address space."]
    #[inline(always)]
    pub fn ahb_dma_access_intr_mem_start_addr(
        &mut self,
    ) -> AHB_DMA_ACCESS_INTR_MEM_START_ADDR_W<'_, AHB_DMA_INTR_MEM_START_ADDR_SPEC> {
        AHB_DMA_ACCESS_INTR_MEM_START_ADDR_W::new(self, 0)
    }
}
#[doc = "Accessible address space start address configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_intr_mem_start_addr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_intr_mem_start_addr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_INTR_MEM_START_ADDR_SPEC;
impl crate::RegisterSpec for AHB_DMA_INTR_MEM_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_intr_mem_start_addr::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_INTR_MEM_START_ADDR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_intr_mem_start_addr::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_INTR_MEM_START_ADDR_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_INTR_MEM_START_ADDR to value 0"]
impl crate::Resettable for AHB_DMA_INTR_MEM_START_ADDR_SPEC {}
