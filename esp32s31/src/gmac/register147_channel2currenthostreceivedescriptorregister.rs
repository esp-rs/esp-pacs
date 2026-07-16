#[doc = "Register `REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER` reader"]
pub type R = crate::R<REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC>;
#[doc = "Field `CH2_CURRDESAPTR` reader - Host Receive Descriptor Address Pointer"]
pub type CH2_CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn ch2_currdesaptr(&self) -> CH2_CURRDESAPTR_R {
        CH2_CURRDESAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER")
            .field("ch2_currdesaptr", &self.ch2_currdesaptr())
            .finish()
    }
}
#[doc = "Points to the start of current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register147_channel2currenthostreceivedescriptorregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register147_channel2currenthostreceivedescriptorregister::R`](R) reader structure"]
impl crate::Readable for REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER to value 0"]
impl crate::Resettable for REGISTER147_CHANNEL2CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {}
