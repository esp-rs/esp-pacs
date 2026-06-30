#[doc = "Register `REGISTER88_CHANNEL1CBSCONTROLREGISTER` reader"]
pub type R = crate::R<REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC>;
#[doc = "Register `REGISTER88_CHANNEL1CBSCONTROLREGISTER` writer"]
pub type W = crate::W<REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC>;
#[doc = "Field `CH1_CBSD` reader - CreditBased Shaper Disable When set, the MAC disables the creditbased shaper algorithm for Channel 1 traffic and makes the traffic management algorithm to strict priority for Channel 1 over Channel 0 When reset, the creditbased shaper algorithm schedules the traffic in Channel 1 for transmission"]
pub type CH1_CBSD_R = crate::BitReader;
#[doc = "Field `CH1_CBSD` writer - CreditBased Shaper Disable When set, the MAC disables the creditbased shaper algorithm for Channel 1 traffic and makes the traffic management algorithm to strict priority for Channel 1 over Channel 0 When reset, the creditbased shaper algorithm schedules the traffic in Channel 1 for transmission"]
pub type CH1_CBSD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_CC` reader - Credit Control When reset, the accumulated credit parameter in the creditbased shaper algorithm logic is set to zero when there is positive credit and no frame to transmit in Channel 1 When there is no frame waiting in Channel 1 and other channel is transmitting, no credit is accumulated When set, the accumulated credit parameter in the creditbased shaper algorithm logic is not reset to zero when there is positive credit and no frame to transmit in Channel 1 The credit accumulates even when there is no frame waiting in Channel 1 and another channel is transmitting"]
pub type CH1_CC_R = crate::BitReader;
#[doc = "Field `CH1_CC` writer - Credit Control When reset, the accumulated credit parameter in the creditbased shaper algorithm logic is set to zero when there is positive credit and no frame to transmit in Channel 1 When there is no frame waiting in Channel 1 and other channel is transmitting, no credit is accumulated When set, the accumulated credit parameter in the creditbased shaper algorithm logic is not reset to zero when there is positive credit and no frame to transmit in Channel 1 The credit accumulates even when there is no frame waiting in Channel 1 and another channel is transmitting"]
pub type CH1_CC_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CH1_SLC` reader - Slot Count The software can program the number of slots _of duration 125 microsec_ over which the average transmitted bits per slot _provided in the CBS Status register_ need to be computed for Channel 1 when the creditbased shaper algorithm is enabled The encoding is as follows: 3'b000: 1 Slot 3'b001: 2 Slots 3'b010: 4 Slots 3'b011: 8 Slots 3'b100: 16 Slots 3'b1013'b111: Reserved"]
pub type CH1_SLC_R = crate::FieldReader;
#[doc = "Field `CH1_SLC` writer - Slot Count The software can program the number of slots _of duration 125 microsec_ over which the average transmitted bits per slot _provided in the CBS Status register_ need to be computed for Channel 1 when the creditbased shaper algorithm is enabled The encoding is as follows: 3'b000: 1 Slot 3'b001: 2 Slots 3'b010: 4 Slots 3'b011: 8 Slots 3'b100: 16 Slots 3'b1013'b111: Reserved"]
pub type CH1_SLC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CH1_ABPSSIE` reader - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts an interrupt _sbd_intr_o or mci_intr_o_ when the average bits per slot status is updated _Bit 17 _ABSU_ in Register 89_ for Channel 1 When this bit is cleared, interrupt is not asserted for such an event"]
pub type CH1_ABPSSIE_R = crate::BitReader;
#[doc = "Field `CH1_ABPSSIE` writer - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts an interrupt _sbd_intr_o or mci_intr_o_ when the average bits per slot status is updated _Bit 17 _ABSU_ in Register 89_ for Channel 1 When this bit is cleared, interrupt is not asserted for such an event"]
pub type CH1_ABPSSIE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CreditBased Shaper Disable When set, the MAC disables the creditbased shaper algorithm for Channel 1 traffic and makes the traffic management algorithm to strict priority for Channel 1 over Channel 0 When reset, the creditbased shaper algorithm schedules the traffic in Channel 1 for transmission"]
    #[inline(always)]
    pub fn ch1_cbsd(&self) -> CH1_CBSD_R {
        CH1_CBSD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Credit Control When reset, the accumulated credit parameter in the creditbased shaper algorithm logic is set to zero when there is positive credit and no frame to transmit in Channel 1 When there is no frame waiting in Channel 1 and other channel is transmitting, no credit is accumulated When set, the accumulated credit parameter in the creditbased shaper algorithm logic is not reset to zero when there is positive credit and no frame to transmit in Channel 1 The credit accumulates even when there is no frame waiting in Channel 1 and another channel is transmitting"]
    #[inline(always)]
    pub fn ch1_cc(&self) -> CH1_CC_R {
        CH1_CC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Slot Count The software can program the number of slots _of duration 125 microsec_ over which the average transmitted bits per slot _provided in the CBS Status register_ need to be computed for Channel 1 when the creditbased shaper algorithm is enabled The encoding is as follows: 3'b000: 1 Slot 3'b001: 2 Slots 3'b010: 4 Slots 3'b011: 8 Slots 3'b100: 16 Slots 3'b1013'b111: Reserved"]
    #[inline(always)]
    pub fn ch1_slc(&self) -> CH1_SLC_R {
        CH1_SLC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 17 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts an interrupt _sbd_intr_o or mci_intr_o_ when the average bits per slot status is updated _Bit 17 _ABSU_ in Register 89_ for Channel 1 When this bit is cleared, interrupt is not asserted for such an event"]
    #[inline(always)]
    pub fn ch1_abpssie(&self) -> CH1_ABPSSIE_R {
        CH1_ABPSSIE_R::new(((self.bits >> 17) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER88_CHANNEL1CBSCONTROLREGISTER")
            .field("ch1_cbsd", &self.ch1_cbsd())
            .field("ch1_cc", &self.ch1_cc())
            .field("ch1_slc", &self.ch1_slc())
            .field("ch1_abpssie", &self.ch1_abpssie())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - CreditBased Shaper Disable When set, the MAC disables the creditbased shaper algorithm for Channel 1 traffic and makes the traffic management algorithm to strict priority for Channel 1 over Channel 0 When reset, the creditbased shaper algorithm schedules the traffic in Channel 1 for transmission"]
    #[inline(always)]
    pub fn ch1_cbsd(&mut self) -> CH1_CBSD_W<'_, REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC> {
        CH1_CBSD_W::new(self, 0)
    }
    #[doc = "Bit 1 - Credit Control When reset, the accumulated credit parameter in the creditbased shaper algorithm logic is set to zero when there is positive credit and no frame to transmit in Channel 1 When there is no frame waiting in Channel 1 and other channel is transmitting, no credit is accumulated When set, the accumulated credit parameter in the creditbased shaper algorithm logic is not reset to zero when there is positive credit and no frame to transmit in Channel 1 The credit accumulates even when there is no frame waiting in Channel 1 and another channel is transmitting"]
    #[inline(always)]
    pub fn ch1_cc(&mut self) -> CH1_CC_W<'_, REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC> {
        CH1_CC_W::new(self, 1)
    }
    #[doc = "Bits 4:6 - Slot Count The software can program the number of slots _of duration 125 microsec_ over which the average transmitted bits per slot _provided in the CBS Status register_ need to be computed for Channel 1 when the creditbased shaper algorithm is enabled The encoding is as follows: 3'b000: 1 Slot 3'b001: 2 Slots 3'b010: 4 Slots 3'b011: 8 Slots 3'b100: 16 Slots 3'b1013'b111: Reserved"]
    #[inline(always)]
    pub fn ch1_slc(&mut self) -> CH1_SLC_W<'_, REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC> {
        CH1_SLC_W::new(self, 4)
    }
    #[doc = "Bit 17 - Average Bits Per Slot Interrupt Enable When this bit is set, the MAC asserts an interrupt _sbd_intr_o or mci_intr_o_ when the average bits per slot status is updated _Bit 17 _ABSU_ in Register 89_ for Channel 1 When this bit is cleared, interrupt is not asserted for such an event"]
    #[inline(always)]
    pub fn ch1_abpssie(&mut self) -> CH1_ABPSSIE_W<'_, REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC> {
        CH1_ABPSSIE_W::new(self, 17)
    }
}
#[doc = "Controls the Channel 1 credit shaping operation on the transmit path\n\nYou can [`read`](crate::Reg::read) this register and get [`register88_channel1cbscontrolregister::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`register88_channel1cbscontrolregister::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register88_channel1cbscontrolregister::R`](R) reader structure"]
impl crate::Readable for REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`register88_channel1cbscontrolregister::W`](W) writer structure"]
impl crate::Writable for REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGISTER88_CHANNEL1CBSCONTROLREGISTER to value 0"]
impl crate::Resettable for REGISTER88_CHANNEL1CBSCONTROLREGISTER_SPEC {}
