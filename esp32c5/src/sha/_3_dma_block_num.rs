#[doc = "Register `_3_DMA_BLOCK_NUM` reader"]
pub type R = crate::R<_3_DMA_BLOCK_NUM_SPEC>;
#[doc = "Register `_3_DMA_BLOCK_NUM` writer"]
pub type W = crate::W<_3_DMA_BLOCK_NUM_SPEC>;
#[doc = "Field `_3_DMA_BLOCK_NUM` reader - DMA-SHA3 block number."]
pub type _3_DMA_BLOCK_NUM_R = crate::FieldReader;
#[doc = "Field `_3_DMA_BLOCK_NUM` writer - DMA-SHA3 block number."]
pub type _3_DMA_BLOCK_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - DMA-SHA3 block number."]
    #[inline(always)]
    pub fn _3_dma_block_num(&self) -> _3_DMA_BLOCK_NUM_R {
        _3_DMA_BLOCK_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_3_DMA_BLOCK_NUM")
            .field("_3_dma_block_num", &self._3_dma_block_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:5 - DMA-SHA3 block number."]
    #[inline(always)]
    pub fn _3_dma_block_num(&mut self) -> _3_DMA_BLOCK_NUM_W<_3_DMA_BLOCK_NUM_SPEC> {
        _3_DMA_BLOCK_NUM_W::new(self, 0)
    }
}
#[doc = "DMA configuration register 0.\n\nYou can [`read`](crate::Reg::read) this register and get [`_3_dma_block_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_3_dma_block_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _3_DMA_BLOCK_NUM_SPEC;
impl crate::RegisterSpec for _3_DMA_BLOCK_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_3_dma_block_num::R`](R) reader structure"]
impl crate::Readable for _3_DMA_BLOCK_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_3_dma_block_num::W`](W) writer structure"]
impl crate::Writable for _3_DMA_BLOCK_NUM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets _3_DMA_BLOCK_NUM to value 0"]
impl crate::Resettable for _3_DMA_BLOCK_NUM_SPEC {
    const RESET_VALUE: u32 = 0;
}
