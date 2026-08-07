#[doc = "Register `AXI_MST_PRI` reader"]
pub type R = crate::R<AXI_MST_PRI_SPEC>;
#[doc = "Register `AXI_MST_PRI` writer"]
pub type W = crate::W<AXI_MST_PRI_SPEC>;
#[doc = "Field `DMA_PRIORITY` reader - AHB-DMA arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
pub type DMA_PRIORITY_R = crate::BitReader;
#[doc = "Field `DMA_PRIORITY` writer - AHB-DMA arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
pub type DMA_PRIORITY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CACHE_PRIORITY` reader - CACHE arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
pub type CACHE_PRIORITY_R = crate::BitReader;
#[doc = "Field `CACHE_PRIORITY` writer - CACHE arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
pub type CACHE_PRIORITY_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - AHB-DMA arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
    #[inline(always)]
    pub fn dma_priority(&self) -> DMA_PRIORITY_R {
        DMA_PRIORITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CACHE arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
    #[inline(always)]
    pub fn cache_priority(&self) -> CACHE_PRIORITY_R {
        CACHE_PRIORITY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AXI_MST_PRI")
            .field("dma_priority", &self.dma_priority())
            .field("cache_priority", &self.cache_priority())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - AHB-DMA arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
    #[inline(always)]
    pub fn dma_priority(&mut self) -> DMA_PRIORITY_W<'_, AXI_MST_PRI_SPEC> {
        DMA_PRIORITY_W::new(self, 0)
    }
    #[doc = "Bit 1 - CACHE arbitration priority for command channels between masters connected to ext_mem_DW_axi"]
    #[inline(always)]
    pub fn cache_priority(&mut self) -> CACHE_PRIORITY_W<'_, AXI_MST_PRI_SPEC> {
        CACHE_PRIORITY_W::new(self, 1)
    }
}
#[doc = "AXI mst priority configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_mst_pri::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_mst_pri::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AXI_MST_PRI_SPEC;
impl crate::RegisterSpec for AXI_MST_PRI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`axi_mst_pri::R`](R) reader structure"]
impl crate::Readable for AXI_MST_PRI_SPEC {}
#[doc = "`write(|w| ..)` method takes [`axi_mst_pri::W`](W) writer structure"]
impl crate::Writable for AXI_MST_PRI_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AXI_MST_PRI to value 0"]
impl crate::Resettable for AXI_MST_PRI_SPEC {}
