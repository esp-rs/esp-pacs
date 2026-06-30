#[doc = "Register `REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER` reader"]
pub type R = crate::R<REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>;
#[doc = "Register `REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER` writer"]
pub type W = crate::W<REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>;
#[doc = "Field `CH1_MISFRMCNT` reader - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
pub type CH1_MISFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `CH1_MISFRMCNT` writer - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
pub type CH1_MISFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CH1_MISCNTOVF` reader - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type CH1_MISCNTOVF_R = crate::BitReader;
#[doc = "Field `CH1_MISCNTOVF` writer - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type CH1_MISCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_OVFFRMCNT` reader - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
pub type CH1_OVFFRMCNT_R = crate::FieldReader<u16>;
#[doc = "Field `CH1_OVFFRMCNT` writer - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
pub type CH1_OVFFRMCNT_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
#[doc = "Field `CH1_OVFCNTOVF` reader - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type CH1_OVFCNTOVF_R = crate::BitReader;
#[doc = "Field `CH1_OVFCNTOVF` writer - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
pub type CH1_OVFCNTOVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
    #[inline(always)]
    pub fn ch1_misfrmcnt(&self) -> CH1_MISFRMCNT_R {
        CH1_MISFRMCNT_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn ch1_miscntovf(&self) -> CH1_MISCNTOVF_R {
        CH1_MISCNTOVF_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
    #[inline(always)]
    pub fn ch1_ovffrmcnt(&self) -> CH1_OVFFRMCNT_R {
        CH1_OVFFRMCNT_R::new(((self.bits >> 17) & 0x07ff) as u16)
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn ch1_ovfcntovf(&self) -> CH1_OVFCNTOVF_R {
        CH1_OVFCNTOVF_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER")
            .field("ch1_misfrmcnt", &self.ch1_misfrmcnt())
            .field("ch1_miscntovf", &self.ch1_miscntovf())
            .field("ch1_ovffrmcnt", &self.ch1_ovffrmcnt())
            .field("ch1_ovfcntovf", &self.ch1_ovfcntovf())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Missed Frame Counter This field indicates the number of frames missed by the controller because of the Host Receive Buffer being unavailable This counter is incremented each time the DMA discards an incoming frame The counter is cleared when this register is read with mci_be_i\\[0\\] at 1’b1"]
    #[inline(always)]
    pub fn ch1_misfrmcnt(
        &mut self,
    ) -> CH1_MISFRMCNT_W<'_, REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>
    {
        CH1_MISFRMCNT_W::new(self, 0)
    }
    #[doc = "Bit 16 - Overflow Bit for Missed Frame Counter This bit is set every time Missed Frame Counter _Bits\\[15:0\\]_ overflows, that is, the DMA discards an incoming frame because of the Host Receive Buffer being unavailable with the missed frame counter at maximum value In such a scenario, the Missed frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn ch1_miscntovf(
        &mut self,
    ) -> CH1_MISCNTOVF_W<'_, REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>
    {
        CH1_MISCNTOVF_W::new(self, 16)
    }
    #[doc = "Bits 17:27 - Overflow Frame Counter This field indicates the number of frames missed by the application This counter is incremented each time the MTL FIFO overflows The counter is cleared when this register is read with mci_be_i\\[2\\] at 1’b1"]
    #[inline(always)]
    pub fn ch1_ovffrmcnt(
        &mut self,
    ) -> CH1_OVFFRMCNT_W<'_, REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>
    {
        CH1_OVFFRMCNT_W::new(self, 17)
    }
    #[doc = "Bit 28 - Overflow Bit for FIFO Overflow Counter This bit is set every time the Overflow Frame Counter _Bits\\[27:17\\]_ overflows, that is, the Rx FIFO overflows with the overflow frame counter at maximum value In such a scenario, the overflow frame counter is reset to allzeros and this bit indicates that the rollover happened"]
    #[inline(always)]
    pub fn ch1_ovfcntovf(
        &mut self,
    ) -> CH1_OVFCNTOVF_W<'_, REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC>
    {
        CH1_OVFCNTOVF_W::new(self, 28)
    }
}
#[doc = "Contains the counters for discarded frames because no host Receive Descriptor was available, and discarded frames because of Receive FIFO Overflow\n\nYou can [`read`](crate::Reg::read) this register and get [`register72_channel1missedframeandbufferoverflowcounterregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register72_channel1missedframeandbufferoverflowcounterregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register72_channel1missedframeandbufferoverflowcounterregister::R`](R) reader structure"]
impl crate::Readable for REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register72_channel1missedframeandbufferoverflowcounterregister::W`](W) writer structure"]
impl crate::Writable for REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER to value 0"]
impl crate::Resettable for REGISTER72_CHANNEL1MISSEDFRAMEANDBUFFEROVERFLOWCOUNTERREGISTER_SPEC {}
