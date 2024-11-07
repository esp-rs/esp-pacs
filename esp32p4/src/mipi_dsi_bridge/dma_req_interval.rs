#[doc = "Register `DMA_REQ_INTERVAL` reader"]
pub type R = crate::R<DMA_REQ_INTERVAL_SPEC>;
#[doc = "Register `DMA_REQ_INTERVAL` writer"]
pub type W = crate::W<DMA_REQ_INTERVAL_SPEC>;
#[doc = "Field `DMA_REQ_INTERVAL` reader - this field configures the interval between dma req events"]
pub type DMA_REQ_INTERVAL_R = crate::FieldReader<u16>;
#[doc = "Field `DMA_REQ_INTERVAL` writer - this field configures the interval between dma req events"]
pub type DMA_REQ_INTERVAL_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - this field configures the interval between dma req events"]
    #[inline(always)]
    pub fn dma_req_interval(&self) -> DMA_REQ_INTERVAL_R {
        DMA_REQ_INTERVAL_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_REQ_INTERVAL")
            .field("dma_req_interval", &self.dma_req_interval())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - this field configures the interval between dma req events"]
    #[inline(always)]
    pub fn dma_req_interval(&mut self) -> DMA_REQ_INTERVAL_W<DMA_REQ_INTERVAL_SPEC> {
        DMA_REQ_INTERVAL_W::new(self, 0)
    }
}
#[doc = "dsi bridge dma req interval control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dma_req_interval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dma_req_interval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMA_REQ_INTERVAL_SPEC;
impl crate::RegisterSpec for DMA_REQ_INTERVAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dma_req_interval::R`](R) reader structure"]
impl crate::Readable for DMA_REQ_INTERVAL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`dma_req_interval::W`](W) writer structure"]
impl crate::Writable for DMA_REQ_INTERVAL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DMA_REQ_INTERVAL to value 0x01"]
impl crate::Resettable for DMA_REQ_INTERVAL_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
