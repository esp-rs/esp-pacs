///Register `DMA_OUT_PUSH` reader
pub type R = crate::R<DMA_OUT_PUSH_SPEC>;
///Register `DMA_OUT_PUSH` writer
pub type W = crate::W<DMA_OUT_PUSH_SPEC>;
///Field `OUTFIFO_WDATA` reader - This is the data need to be pushed into out link descriptor's fifo.
pub type OUTFIFO_WDATA_R = crate::FieldReader<u16>;
///Field `OUTFIFO_WDATA` writer - This is the data need to be pushed into out link descriptor's fifo.
pub type OUTFIFO_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
///Field `OUTFIFO_PUSH` reader - Set this bit to push data in out link descriptor's fifo.
pub type OUTFIFO_PUSH_R = crate::BitReader;
///Field `OUTFIFO_PUSH` writer - Set this bit to push data in out link descriptor's fifo.
pub type OUTFIFO_PUSH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bits 0:8 - This is the data need to be pushed into out link descriptor's fifo.
    #[inline(always)]
    pub fn outfifo_wdata(&self) -> OUTFIFO_WDATA_R {
        OUTFIFO_WDATA_R::new((self.bits & 0x01ff) as u16)
    }
    ///Bit 16 - Set this bit to push data in out link descriptor's fifo.
    #[inline(always)]
    pub fn outfifo_push(&self) -> OUTFIFO_PUSH_R {
        OUTFIFO_PUSH_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_PUSH")
            .field("outfifo_wdata", &self.outfifo_wdata())
            .field("outfifo_push", &self.outfifo_push())
            .finish()
    }
}
impl W {
    ///Bits 0:8 - This is the data need to be pushed into out link descriptor's fifo.
    #[inline(always)]
    #[must_use]
    pub fn outfifo_wdata(&mut self) -> OUTFIFO_WDATA_W<DMA_OUT_PUSH_SPEC> {
        OUTFIFO_WDATA_W::new(self, 0)
    }
    ///Bit 16 - Set this bit to push data in out link descriptor's fifo.
    #[inline(always)]
    #[must_use]
    pub fn outfifo_push(&mut self) -> OUTFIFO_PUSH_W<DMA_OUT_PUSH_SPEC> {
        OUTFIFO_PUSH_W::new(self, 16)
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_push::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_out_push::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_OUT_PUSH_SPEC;
impl crate::RegisterSpec for DMA_OUT_PUSH_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_out_push::R`](R) reader structure
impl crate::Readable for DMA_OUT_PUSH_SPEC {}
///`write(|w| ..)` method takes [`dma_out_push::W`](W) writer structure
impl crate::Writable for DMA_OUT_PUSH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets DMA_OUT_PUSH to value 0
impl crate::Resettable for DMA_OUT_PUSH_SPEC {
    const RESET_VALUE: u32 = 0;
}
