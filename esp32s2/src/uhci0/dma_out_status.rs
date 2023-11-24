#[doc = "Register `DMA_OUT_STATUS` reader"]
pub type R = crate::R<DMA_OUT_STATUS_SPEC>;
#[doc = "Field `OUT_FULL` reader - 1: DMA TX FIFO is full."]
pub type OUT_FULL_R = crate::BitReader;
#[doc = "Field `OUT_EMPTY` reader - 1: DMA TX FIFO is empty."]
pub type OUT_EMPTY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - 1: DMA TX FIFO is full."]
    #[inline(always)]
    pub fn out_full(&self) -> OUT_FULL_R {
        OUT_FULL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: DMA TX FIFO is empty."]
    #[inline(always)]
    pub fn out_empty(&self) -> OUT_EMPTY_R {
        OUT_EMPTY_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_OUT_STATUS")
            .field("out_full", &format_args!("{}", self.out_full().bit()))
            .field("out_empty", &format_args!("{}", self.out_empty().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_OUT_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "DMA data-output status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_out_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_OUT_STATUS_SPEC;
impl crate::RegisterSpec for DMA_OUT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_out_status::R`](R) reader structure"]
impl crate::Readable for DMA_OUT_STATUS_SPEC {}
#[doc = "`reset()` method sets DMA_OUT_STATUS to value 0x02"]
impl crate::Resettable for DMA_OUT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
