#[doc = "Register `REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER` reader"]
pub type R = crate::R<REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC>;
#[doc = "Field `CH1_CURRDESAPTR` reader - Host Receive Descriptor Address Pointer"]
pub type CH1_CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn ch1_currdesaptr(&self) -> CH1_CURRDESAPTR_R {
        CH1_CURRDESAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER")
            .field("ch1_currdesaptr", &self.ch1_currdesaptr())
            .finish()
    }
}
#[doc = "Points to the start of current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register83_channel1currenthostreceivedescriptorregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register83_channel1currenthostreceivedescriptorregister::R`](R) reader structure"]
impl crate::Readable for REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER to value 0"]
impl crate::Resettable for REGISTER83_CHANNEL1CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {}
