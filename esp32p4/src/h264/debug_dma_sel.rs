#[doc = "Register `DEBUG_DMA_SEL` reader"]
pub type R = crate::R<DEBUG_DMA_SEL_SPEC>;
#[doc = "Register `DEBUG_DMA_SEL` writer"]
pub type W = crate::W<DEBUG_DMA_SEL_SPEC>;
#[doc = "Field `DBG_DMA_SEL` reader - Every bit represents a dma in h264"]
pub type DBG_DMA_SEL_R = crate::FieldReader;
#[doc = "Field `DBG_DMA_SEL` writer - Every bit represents a dma in h264"]
pub type DBG_DMA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Every bit represents a dma in h264"]
    #[inline(always)]
    pub fn dbg_dma_sel(&self) -> DBG_DMA_SEL_R {
        DBG_DMA_SEL_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DEBUG_DMA_SEL")
            .field("dbg_dma_sel", &self.dbg_dma_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Every bit represents a dma in h264"]
    #[inline(always)]
    pub fn dbg_dma_sel(&mut self) -> DBG_DMA_SEL_W<DEBUG_DMA_SEL_SPEC> {
        DBG_DMA_SEL_W::new(self, 0)
    }
}
#[doc = "Debug H264 DMA select register\n\nYou can [`read`](crate::Reg::read) this register and get [`debug_dma_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`debug_dma_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DEBUG_DMA_SEL_SPEC;
impl crate::RegisterSpec for DEBUG_DMA_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`debug_dma_sel::R`](R) reader structure"]
impl crate::Readable for DEBUG_DMA_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`debug_dma_sel::W`](W) writer structure"]
impl crate::Writable for DEBUG_DMA_SEL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DEBUG_DMA_SEL to value 0"]
impl crate::Resettable for DEBUG_DMA_SEL_SPEC {
    const RESET_VALUE: u32 = 0;
}
