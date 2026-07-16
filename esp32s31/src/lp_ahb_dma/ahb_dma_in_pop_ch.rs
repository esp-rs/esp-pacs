#[doc = "Register `AHB_DMA_IN_POP_CH%s` reader"]
pub type R = crate::R<AHB_DMA_IN_POP_CH_SPEC>;
#[doc = "Register `AHB_DMA_IN_POP_CH%s` writer"]
pub type W = crate::W<AHB_DMA_IN_POP_CH_SPEC>;
#[doc = "Field `AHB_DMA_INFIFO_RDATA_CH` reader - Represents the data popped from AHB_DMA FIFO."]
pub type AHB_DMA_INFIFO_RDATA_CH_R = crate::FieldReader<u16>;
#[doc = "Field `AHB_DMA_INFIFO_POP_CH` writer - Configures whether to pop data from AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Pop\\\\"]
pub type AHB_DMA_INFIFO_POP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Represents the data popped from AHB_DMA FIFO."]
    #[inline(always)]
    pub fn ahb_dma_infifo_rdata_ch(&self) -> AHB_DMA_INFIFO_RDATA_CH_R {
        AHB_DMA_INFIFO_RDATA_CH_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("AHB_DMA_IN_POP_CH")
            .field("ahb_dma_infifo_rdata_ch", &self.ahb_dma_infifo_rdata_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Configures whether to pop data from AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Pop\\\\"]
    #[inline(always)]
    pub fn ahb_dma_infifo_pop_ch(&mut self) -> AHB_DMA_INFIFO_POP_CH_W<'_, AHB_DMA_IN_POP_CH_SPEC> {
        AHB_DMA_INFIFO_POP_CH_W::new(self, 12)
    }
}
#[doc = "Pop control register of RX channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`ahb_dma_in_pop_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ahb_dma_in_pop_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct AHB_DMA_IN_POP_CH_SPEC;
impl crate::RegisterSpec for AHB_DMA_IN_POP_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ahb_dma_in_pop_ch::R`](R) reader structure"]
impl crate::Readable for AHB_DMA_IN_POP_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ahb_dma_in_pop_ch::W`](W) writer structure"]
impl crate::Writable for AHB_DMA_IN_POP_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets AHB_DMA_IN_POP_CH%s to value 0x0800"]
impl crate::Resettable for AHB_DMA_IN_POP_CH_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
