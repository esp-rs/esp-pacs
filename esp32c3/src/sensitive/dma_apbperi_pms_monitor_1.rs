#[doc = "Register `DMA_APBPERI_PMS_MONITOR_1` reader"]
pub type R = crate::R<DMA_APBPERI_PMS_MONITOR_1_SPEC>;
#[doc = "Register `DMA_APBPERI_PMS_MONITOR_1` writer"]
pub type W = crate::W<DMA_APBPERI_PMS_MONITOR_1_SPEC>;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR` reader - dma_apbperi_pms_monitor_violate_clr"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR_R = crate::BitReader;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR` writer - dma_apbperi_pms_monitor_violate_clr"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_EN` reader - dma_apbperi_pms_monitor_violate_en"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_EN_R = crate::BitReader;
#[doc = "Field `DMA_APBPERI_PMS_MONITOR_VIOLATE_EN` writer - dma_apbperi_pms_monitor_violate_en"]
pub type DMA_APBPERI_PMS_MONITOR_VIOLATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - dma_apbperi_pms_monitor_violate_clr"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_clr(&self) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - dma_apbperi_pms_monitor_violate_en"]
    #[inline(always)]
    pub fn dma_apbperi_pms_monitor_violate_en(&self) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_EN_R {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_APBPERI_PMS_MONITOR_1")
            .field(
                "dma_apbperi_pms_monitor_violate_clr",
                &format_args!("{}", self.dma_apbperi_pms_monitor_violate_clr().bit()),
            )
            .field(
                "dma_apbperi_pms_monitor_violate_en",
                &format_args!("{}", self.dma_apbperi_pms_monitor_violate_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_APBPERI_PMS_MONITOR_1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - dma_apbperi_pms_monitor_violate_clr"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_pms_monitor_violate_clr(
        &mut self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR_W<DMA_APBPERI_PMS_MONITOR_1_SPEC> {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - dma_apbperi_pms_monitor_violate_en"]
    #[inline(always)]
    #[must_use]
    pub fn dma_apbperi_pms_monitor_violate_en(
        &mut self,
    ) -> DMA_APBPERI_PMS_MONITOR_VIOLATE_EN_W<DMA_APBPERI_PMS_MONITOR_1_SPEC> {
        DMA_APBPERI_PMS_MONITOR_VIOLATE_EN_W::new(self, 1)
    }
}
#[doc = "SENSITIVE_DMA_APBPERI_PMS_MONITOR_1_REG\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_apbperi_pms_monitor_1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_apbperi_pms_monitor_1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_APBPERI_PMS_MONITOR_1_SPEC;
impl crate::RegisterSpec for DMA_APBPERI_PMS_MONITOR_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_apbperi_pms_monitor_1::R`](R) reader structure"]
impl crate::Readable for DMA_APBPERI_PMS_MONITOR_1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_apbperi_pms_monitor_1::W`](W) writer structure"]
impl crate::Writable for DMA_APBPERI_PMS_MONITOR_1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_APBPERI_PMS_MONITOR_1 to value 0x03"]
impl crate::Resettable for DMA_APBPERI_PMS_MONITOR_1_SPEC {
    const RESET_VALUE: u32 = 0x03;
}
