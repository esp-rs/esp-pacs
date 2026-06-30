#[doc = "Register `REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER` reader"]
pub type R = crate::R<REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC>;
#[doc = "Field `CURTDESAPTR` reader - Host Transmit Descriptor Address Pointer"]
pub type CURTDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Transmit Descriptor Address Pointer"]
    #[inline(always)]
    pub fn curtdesaptr(&self) -> CURTDESAPTR_R {
        CURTDESAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER")
            .field("curtdesaptr", &self.curtdesaptr())
            .finish()
    }
}
#[doc = "Points to the start of current Transmit Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register18_currenthosttransmitdescriptorregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register18_currenthosttransmitdescriptorregister::R`](R) reader structure"]
impl crate::Readable for REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER to value 0"]
impl crate::Resettable for REGISTER18_CURRENTHOSTTRANSMITDESCRIPTORREGISTER_SPEC {}
