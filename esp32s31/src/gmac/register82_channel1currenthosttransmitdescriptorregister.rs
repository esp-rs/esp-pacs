#[doc = "Register `REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER` reader"]
pub type R = crate::R<REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC>;
#[doc = "Field `CH1_CURTDESAPTR` reader - Host Transmit Descriptor Address Pointer"]
pub type CH1_CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn ch1_curtdesaptr(&self) -> CH1_CURTDESAPTR_R {
        CH1_CURTDESAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER")
            .field("ch1_curtdesaptr", &self.ch1_curtdesaptr())
            .finish()
    }
}
#[doc = "Points to the start of current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register82_channel1currenthosttransmitdescriptorregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register82_channel1currenthosttransmitdescriptorregister::R`](R) reader structure"]
impl crate::Readable for REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER to value 0"]
impl crate::Resettable for REGISTER82_CHANNEL1CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {}
