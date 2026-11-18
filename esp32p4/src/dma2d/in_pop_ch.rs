#[doc = "Register `IN_POP_CH%s` reader"]
pub type R = crate::R<IN_POP_CH_SPEC>;
#[doc = "Register `IN_POP_CH%s` writer"]
pub type W = crate::W<IN_POP_CH_SPEC>;
#[doc = "Field `INFIFO_RDATA_CH` reader - This register stores the data popping from DMA Rx FIFO."]
pub type INFIFO_RDATA_CH_R = crate::FieldReader<u16>;
#[doc = "Field `INFIFO_POP_CH` reader - Set this bit to pop data from DMA Rx FIFO."]
pub type INFIFO_POP_CH_R = crate::BitReader;
#[doc = "Field `INFIFO_POP_CH` writer - Set this bit to pop data from DMA Rx FIFO."]
pub type INFIFO_POP_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:10 - This register stores the data popping from DMA Rx FIFO."]
    #[inline(always)]
    pub fn infifo_rdata_ch(&self) -> INFIFO_RDATA_CH_R {
        INFIFO_RDATA_CH_R::new((self.bits & 0x07ff) as u16)
    }
    #[doc = "Bit 11 - Set this bit to pop data from DMA Rx FIFO."]
    #[inline(always)]
    pub fn infifo_pop_ch(&self) -> INFIFO_POP_CH_R {
        INFIFO_POP_CH_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_POP_CH")
            .field("infifo_rdata_ch", &self.infifo_rdata_ch())
            .field("infifo_pop_ch", &self.infifo_pop_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 11 - Set this bit to pop data from DMA Rx FIFO."]
    #[inline(always)]
    pub fn infifo_pop_ch(&mut self) -> INFIFO_POP_CH_W<'_, IN_POP_CH_SPEC> {
        INFIFO_POP_CH_W::new(self, 11)
    }
}
#[doc = "Configures the rx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_pop_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_pop_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_POP_CH_SPEC;
impl crate::RegisterSpec for IN_POP_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_pop_ch::R`](R) reader structure"]
impl crate::Readable for IN_POP_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_pop_ch::W`](W) writer structure"]
impl crate::Writable for IN_POP_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_POP_CH%s to value 0x0400"]
impl crate::Resettable for IN_POP_CH_SPEC {
    const RESET_VALUE: u32 = 0x0400;
}
