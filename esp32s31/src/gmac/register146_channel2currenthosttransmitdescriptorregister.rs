#[doc = "Register `REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER` reader"]
pub type R = crate::R<REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC>;
#[doc = "Field `CH2_CURTDESAPTR` reader - Host Transmit Descriptor Address Pointer"]
pub type CH2_CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn ch2_curtdesaptr(&self) -> CH2_CURTDESAPTR_R {
        CH2_CURTDESAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER")
            .field("ch2_curtdesaptr", &self.ch2_curtdesaptr())
            .finish()
    }
}
#[doc = "Points to the start of current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register146_channel2currenthosttransmitdescriptorregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register146_channel2currenthosttransmitdescriptorregister::R`](R) reader structure"]
impl crate::Readable for REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER to value 0"]
impl crate::Resettable for REGISTER146_CHANNEL2CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {}
