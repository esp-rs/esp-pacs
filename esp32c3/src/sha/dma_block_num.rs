#[doc = "Register `DMA_BLOCK_NUM` reader"]
pub type R = crate::R<DMA_BLOCK_NUM_SPEC>;
#[doc = "Register `DMA_BLOCK_NUM` writer"]
pub type W = crate::W<DMA_BLOCK_NUM_SPEC>;
#[doc = "Field `DMA_BLOCK_NUM` reader - Dma-sha block number."]
pub type DMA_BLOCK_NUM_R = crate::FieldReader;
#[doc = "Field `DMA_BLOCK_NUM` writer - Dma-sha block number."]
pub type DMA_BLOCK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Dma-sha block number."]
    #[inline(always)]
    pub fn dma_block_num(&self) -> DMA_BLOCK_NUM_R {
        DMA_BLOCK_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_BLOCK_NUM")
            .field(
                "dma_block_num",
                &format_args!("{}", self.dma_block_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_BLOCK_NUM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:5 - Dma-sha block number."]
    #[inline(always)]
    #[must_use]
    pub fn dma_block_num(&mut self) -> DMA_BLOCK_NUM_W<DMA_BLOCK_NUM_SPEC> {
        DMA_BLOCK_NUM_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 0.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_block_num::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_block_num::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_BLOCK_NUM_SPEC;
impl crate::RegisterSpec for DMA_BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_block_num::R`](R) reader structure"]
impl crate::Readable for DMA_BLOCK_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_block_num::W`](W) writer structure"]
impl crate::Writable for DMA_BLOCK_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_BLOCK_NUM to value 0"]
impl crate::Resettable for DMA_BLOCK_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
