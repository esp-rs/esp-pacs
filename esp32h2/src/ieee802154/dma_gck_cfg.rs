#[doc = "Register `DMA_GCK_CFG` reader"]
pub type R = crate::R<DMA_GCK_CFG_SPEC>;
#[doc = "Register `DMA_GCK_CFG` writer"]
pub type W = crate::W<DMA_GCK_CFG_SPEC>;
#[doc = "Field `DMA_GCK_CFG` reader - "]
pub type DMA_GCK_CFG_R = crate::BitReader;
#[doc = "Field `DMA_GCK_CFG` writer - "]
pub type DMA_GCK_CFG_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dma_gck_cfg(&self) -> DMA_GCK_CFG_R {
        DMA_GCK_CFG_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_GCK_CFG")
            .field("dma_gck_cfg", &format_args!("{}", self.dma_gck_cfg().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_GCK_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn dma_gck_cfg(&mut self) -> DMA_GCK_CFG_W<DMA_GCK_CFG_SPEC> {
        DMA_GCK_CFG_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_gck_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_gck_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_GCK_CFG_SPEC;
impl crate::RegisterSpec for DMA_GCK_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_gck_cfg::R`](R) reader structure"]
impl crate::Readable for DMA_GCK_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_gck_cfg::W`](W) writer structure"]
impl crate::Writable for DMA_GCK_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_GCK_CFG to value 0"]
impl crate::Resettable for DMA_GCK_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
