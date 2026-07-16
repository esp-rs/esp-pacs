#[doc = "Register `REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER` reader"]
pub type R = crate::R<REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC>;
#[doc = "Field `CURRDESAPTR` reader - Host Receive Descriptor Address Pointer"]
pub type CURRDESAPTR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Host Receive Descriptor Address Pointer"]
    #[inline(always)]
    pub fn currdesaptr(&self) -> CURRDESAPTR_R {
        CURRDESAPTR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER")
            .field("currdesaptr", &self.currdesaptr())
            .finish()
    }
}
#[doc = "Points to the start of current Receive Descriptor read by the DMA\n\nYou can [`read`](crate::Reg::read) this register and get [`register19_currenthostreceivedescriptorregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register19_currenthostreceivedescriptorregister::R`](R) reader structure"]
impl crate::Readable for REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER to value 0"]
impl crate::Resettable for REGISTER19_CURRENTHOSTRECEIVEDESCRIPTORREGISTER_SPEC {}
