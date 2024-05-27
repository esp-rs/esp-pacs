///Register `DMA` reader
pub type R = crate::R<DMA_SPEC>;
///Register `DMA` writer
pub type W = crate::W<DMA_SPEC>;
///Field `DMAADDR` reader -
pub type DMAADDR_R = crate::FieldReader<u32>;
///Field `DMAADDR` writer -
pub type DMAADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn dmaaddr(&self) -> DMAADDR_R {
        DMAADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA")
            .field("dmaaddr", &self.dmaaddr())
            .finish()
    }
}
impl W {
    ///Bits 0:31
    #[inline(always)]
    #[must_use]
    pub fn dmaaddr(&mut self) -> DMAADDR_W<DMA_SPEC> {
        DMAADDR_W::new(self, 0)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dma::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_SPEC;
impl crate::RegisterSpec for DMA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma::R`](R) reader structure
impl crate::Readable for DMA_SPEC {}
///`write(|w| ..)` method takes [`dma::W`](W) writer structure
impl crate::Writable for DMA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA to value 0
impl crate::Resettable for DMA_SPEC {
    const RESET_VALUE: u32 = 0;
}
