#[doc = "Register `DMA_IN_DSCR` reader"]
pub type R = crate::R<DMA_IN_DSCR_SPEC>;
#[doc = "Field `INLINK_DSCR` reader - The content of current in link descriptor's third dword"]
pub type INLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The content of current in link descriptor's third dword"]
    #[inline(always)]
    pub fn inlink_dscr(&self) -> INLINK_DSCR_R {
        INLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_IN_DSCR")
            .field(
                "inlink_dscr",
                &format_args!("{}", self.inlink_dscr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_IN_DSCR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_in_dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_IN_DSCR_SPEC;
impl crate::RegisterSpec for DMA_IN_DSCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_in_dscr::R`](R) reader structure"]
impl crate::Readable for DMA_IN_DSCR_SPEC {}
#[doc = "`reset()` method sets DMA_IN_DSCR to value 0"]
impl crate::Resettable for DMA_IN_DSCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
