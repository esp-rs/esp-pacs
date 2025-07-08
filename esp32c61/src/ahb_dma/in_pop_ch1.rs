#[doc = "Register `IN_POP_CH1` reader"]
pub type R = crate::R<IN_POP_CH1_SPEC>;
#[doc = "Register `IN_POP_CH1` writer"]
pub type W = crate::W<IN_POP_CH1_SPEC>;
#[doc = "Field `INFIFO_RDATA_CH1` reader - Represents the data popped from AHB_DMA FIFO."]
pub type INFIFO_RDATA_CH1_R = crate::FieldReader<u16>;
#[doc = "Field `INFIFO_POP_CH1` writer - Configures whether to pop data from AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Pop\\\\"]
pub type INFIFO_POP_CH1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:11 - Represents the data popped from AHB_DMA FIFO."]
    #[inline(always)]
    pub fn infifo_rdata_ch1(&self) -> INFIFO_RDATA_CH1_R {
        INFIFO_RDATA_CH1_R::new((self.bits & 0x0fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_POP_CH1")
            .field("infifo_rdata_ch1", &self.infifo_rdata_ch1())
            .finish()
    }
}
impl W {
    #[doc = "Bit 12 - Configures whether to pop data from AHB_DMA FIFO.\\\\0: Invalid. No effect\\\\1: Pop\\\\"]
    #[inline(always)]
    pub fn infifo_pop_ch1(&mut self) -> INFIFO_POP_CH1_W<IN_POP_CH1_SPEC> {
        INFIFO_POP_CH1_W::new(self, 12)
    }
}
#[doc = "Receive FIFO status of RX channel 1\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop_ch1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop_ch1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_POP_CH1_SPEC;
impl crate::RegisterSpec for IN_POP_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pop_ch1::R`](R) reader structure"]
impl crate::Readable for IN_POP_CH1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_pop_ch1::W`](W) writer structure"]
impl crate::Writable for IN_POP_CH1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_POP_CH1 to value 0x0800"]
impl crate::Resettable for IN_POP_CH1_SPEC {
    const RESET_VALUE: u32 = 0x0800;
}
