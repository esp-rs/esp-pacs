///Register `DMA_OUT_STATUS` reader
pub type R = crate::R<DMA_OUT_STATUS_SPEC>;
///Field `OUT_FULL` reader - 1:DMA out link descriptor's fifo is full.
pub type OUT_FULL_R = crate::BitReader;
///Field `OUT_EMPTY` reader - 1:DMA in link descriptor's fifo is empty.
pub type OUT_EMPTY_R = crate::BitReader;
impl R {
    ///Bit 0 - 1:DMA out link descriptor's fifo is full.
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1:DMA in link descriptor's fifo is empty.
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_STATUS")
            .field("out_full", &self.out_full())
            .field("out_empty", &self.out_empty())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`dma_out_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DMA_OUT_STATUS_SPEC;
impl crate::RegisterSpec for DMA_OUT_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dma_out_status::R`](R) reader structure
impl crate::Readable for DMA_OUT_STATUS_SPEC {}
///`reset()` method sets DMA_OUT_STATUS to value 0x02
impl crate::Resettable for DMA_OUT_STATUS_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
