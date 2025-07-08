#[doc = "Register `OUT_PUSH_CH1` reader"]
pub type R = crate::R<OUT_PUSH_CH1_SPEC>;
#[doc = "Register `OUT_PUSH_CH1` writer"]
pub type W = crate::W<OUT_PUSH_CH1_SPEC>;
#[doc = "Field `OUTFIFO_WDATA_CH1` reader - Configures whether to push data into AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Push\\\\"]
pub type OUTFIFO_WDATA_CH1_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA_CH1` writer - Configures whether to push data into AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Push\\\\"]
pub type OUTFIFO_WDATA_CH1_W<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `OUTFIFO_PUSH_CH1` writer - Configures the data that need to be pushed into AHB_DMA FIFO."]
pub type OUTFIFO_PUSH_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - Configures whether to push data into AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Push\\\\"]
    #[inline(always)]
    pub fn outfifo_wdata_ch1(&self) -> OUTFIFO_WDATA_CH1_R {
        OUTFIFO_WDATA_CH1_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PUSH_CH1")
            .field("outfifo_wdata_ch1", &self.outfifo_wdata_ch1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:8 - Configures whether to push data into AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Push\\\\"]
    #[inline(always)]
    pub fn outfifo_wdata_ch1(&mut self) -> OUTFIFO_WDATA_CH1_W<OUT_PUSH_CH1_SPEC> {
        OUTFIFO_WDATA_CH1_W::new(self, 0)
    }
    #[doc = "Bit 9 - Configures the data that need to be pushed into AHB_DMA FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch1(&mut self) -> OUTFIFO_PUSH_CH1_W<OUT_PUSH_CH1_SPEC> {
        OUTFIFO_PUSH_CH1_W::new(self, 9)
    }
}
#[doc = "Push control register of TX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PUSH_CH1_SPEC;
impl crate::RegisterSpec for OUT_PUSH_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_push_ch1::R`](R) reader structure"]
impl crate::Readable for OUT_PUSH_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_push_ch1::W`](W) writer structure"]
impl crate::Writable for OUT_PUSH_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_PUSH_CH1 to value 0"]
impl crate::Resettable for OUT_PUSH_CH1_SPEC {}
