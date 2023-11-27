#[doc = "Register `DMA_REQ_INTERVAL` reader"]
pub type R = crate::R<DMA_REQ_INTERVAL_SPEC>;
#[doc = "Register `DMA_REQ_INTERVAL` writer"]
pub type W = crate::W<DMA_REQ_INTERVAL_SPEC>;
#[doc = "Field `DMA_REQ_INTERVAL` reader - 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle."]
pub type DMA_REQ_INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `DMA_REQ_INTERVAL` writer - 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle."]
pub type DMA_REQ_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle."]
    #[inline(always)]
    pub fn dma_req_interval(&self) -> DMA_REQ_INTERVAL_R {
        DMA_REQ_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_REQ_INTERVAL")
            .field(
                "dma_req_interval",
                &format_args!("{}", self.dma_req_interval().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_REQ_INTERVAL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:15 - 16'b1: 1 cycle. 16'b11: 2 cycle. ... ... 16'hFFFF: 16 cycle."]
    #[inline(always)]
    #[must_use]
    pub fn dma_req_interval(&mut self) -> DMA_REQ_INTERVAL_W<DMA_REQ_INTERVAL_SPEC> {
        DMA_REQ_INTERVAL_W::new(self, 0)
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
#[doc = "DMA interval configuration.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dma_req_interval::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`dma_req_interval::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_REQ_INTERVAL_SPEC;
impl crate::RegisterSpec for DMA_REQ_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_interval::R`](R) reader structure"]
impl crate::Readable for DMA_REQ_INTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_req_interval::W`](W) writer structure"]
impl crate::Writable for DMA_REQ_INTERVAL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DMA_REQ_INTERVAL to value 0x01"]
impl crate::Resettable for DMA_REQ_INTERVAL_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
