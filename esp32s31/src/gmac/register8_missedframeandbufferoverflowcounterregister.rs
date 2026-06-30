#[doc = "Register `REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER` reader"]
pub type R = crate::R<REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>;
#[doc = "Register `REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER` writer"]
pub type W = crate::W<REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>;
#[doc = "Field `MISFRMCNT` reader - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
pub type MISFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `MISFRMCNT` writer - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
pub type MISFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `MISCNTOVF` reader - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type MISCNTOVF_R = crate::BitReader;
#[doc = "Field `MISCNTOVF` writer - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type MISCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVFFRMCNT` reader - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
pub type OVFFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `OVFFRMCNT` writer - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
pub type OVFFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `OVFCNTOVF` reader - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type OVFCNTOVF_R = crate::BitReader;
#[doc = "Field `OVFCNTOVF` writer - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type OVFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
    #[inline(always)]
    pub fn misfrmcnt(&self) -> MISFRMCNT_R {
        MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn miscntovf(&self) -> MISCNTOVF_R {
        MISCNTOVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
    #[inline(always)]
    pub fn ovffrmcnt(&self) -> OVFFRMCNT_R {
        OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn ovfcntovf(&self) -> OVFCNTOVF_R {
        OVFCNTOVF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER")
            .field("misfrmcnt", &self.misfrmcnt())
            .field("miscntovf", &self.miscntovf())
            .field("ovffrmcnt", &self.ovffrmcnt())
            .field("ovfcntovf", &self.ovfcntovf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
    #[inline(always)]
    pub fn misfrmcnt(
        &mut self,
    ) -> MISFRMCNT_W<'_, REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC> {
        MISFRMCNT_W::new(self, 0)
    }
    #[doc = "Bit 16 - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn miscntovf(
        &mut self,
    ) -> MISCNTOVF_W<'_, REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC> {
        MISCNTOVF_W::new(self, 16)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
    #[inline(always)]
    pub fn ovffrmcnt(
        &mut self,
    ) -> OVFFRMCNT_W<'_, REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC> {
        OVFFRMCNT_W::new(self, 17)
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn ovfcntovf(
        &mut self,
    ) -> OVFCNTOVF_W<'_, REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC> {
        OVFCNTOVF_W::new(self, 28)
    }
}
#[doc = "Contains the counters for discarded frames because no host Receive Descriptor was available or because of Receive FIFO Overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`register8_missedframeandbufferoverflowcounterregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register8_missedframeandbufferoverflowcounterregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register8_missedframeandbufferoverflowcounterregister::R`](R) reader structure"]
impl crate::Readable for REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register8_missedframeandbufferoverflowcounterregister::W`](W) writer structure"]
impl crate::Writable for REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER to value 0"]
impl crate::Resettable for REGISTER8_MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {}
