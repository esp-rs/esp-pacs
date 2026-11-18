#[doc = "Register `OUT_PUSH_CH%s` reader"]
pub type R = crate::R<OUT_PUSH_CH_SPEC>;
#[doc = "Register `OUT_PUSH_CH%s` writer"]
pub type W = crate::W<OUT_PUSH_CH_SPEC>;
#[doc = "Field `OUTFIFO_WDATA_CH` reader - This register stores the data that need to be pushed into DMA Tx FIFO."]
pub type OUTFIFO_WDATA_CH_R = crate::FieldReader<u16>;
#[doc = "Field `OUTFIFO_WDATA_CH` writer - This register stores the data that need to be pushed into DMA Tx FIFO."]
pub type OUTFIFO_WDATA_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
#[doc = "Field `OUTFIFO_PUSH_CH` reader - Set this bit to push data into DMA Tx FIFO."]
pub type OUTFIFO_PUSH_CH_R = crate::BitReader;
#[doc = "Field `OUTFIFO_PUSH_CH` writer - Set this bit to push data into DMA Tx FIFO."]
pub type OUTFIFO_PUSH_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:9 - This register stores the data that need to be pushed into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch(&self) -> OUTFIFO_WDATA_CH_R {
        OUTFIFO_WDATA_CH_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Set this bit to push data into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch(&self) -> OUTFIFO_PUSH_CH_R {
        OUTFIFO_PUSH_CH_R::new(((self.bits >> 10) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_PUSH_CH")
            .field("outfifo_wdata_ch", &self.outfifo_wdata_ch())
            .field("outfifo_push_ch", &self.outfifo_push_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - This register stores the data that need to be pushed into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_wdata_ch(&mut self) -> OUTFIFO_WDATA_CH_W<'_, OUT_PUSH_CH_SPEC> {
        OUTFIFO_WDATA_CH_W::new(self, 0)
    }
    #[doc = "Bit 10 - Set this bit to push data into DMA Tx FIFO."]
    #[inline(always)]
    pub fn outfifo_push_ch(&mut self) -> OUTFIFO_PUSH_CH_W<'_, OUT_PUSH_CH_SPEC> {
        OUTFIFO_PUSH_CH_W::new(self, 10)
    }
}
#[doc = "Configures the tx fifo of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_push_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_push_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_PUSH_CH_SPEC;
impl crate::RegisterSpec for OUT_PUSH_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_push_ch::R`](R) reader structure"]
impl crate::Readable for OUT_PUSH_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_push_ch::W`](W) writer structure"]
impl crate::Writable for OUT_PUSH_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_PUSH_CH%s to value 0"]
impl crate::Resettable for OUT_PUSH_CH_SPEC {}
