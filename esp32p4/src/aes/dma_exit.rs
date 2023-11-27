#[doc = "Register `DMA_EXIT` writer"]
pub type W = crate::W<DMA_EXIT_SPEC>;
#[doc = "Field `DMA_EXIT` writer - Set this register to leave calculation done stage. Recommend to use it after software finishes reading DMA's output buffer."]
pub type DMA_EXIT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_EXIT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this register to leave calculation done stage. Recommend to use it after software finishes reading DMA's output buffer."]
    #[inline(always)]
    #[must_use]
    pub fn dma_exit(&mut self) -> DMA_EXIT_W<DMA_EXIT_SPEC> {
        DMA_EXIT_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "AES-DMA exit config\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_exit::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_EXIT_SPEC;
impl crate::RegisterSpec for DMA_EXIT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dma_exit::W`](W) writer structure"]
impl crate::Writable for DMA_EXIT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_EXIT to value 0"]
impl crate::Resettable for DMA_EXIT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
