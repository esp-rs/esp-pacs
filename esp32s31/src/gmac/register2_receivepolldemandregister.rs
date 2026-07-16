#[doc = "Register `REGISTER2_RECEIVEPOLLDEMANDREGISTER` reader"]
pub type R = crate::R<REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC>;
#[doc = "Field `RPD` reader - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 _Current Host Receive Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the reception returns to the Suspended state and Bit 7 _RU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the Rx DMA returns to the active state"]
pub type RPD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 _Current Host Receive Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the reception returns to the Suspended state and Bit 7 _RU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the Rx DMA returns to the active state"]
    #[inline(always)]
    pub fn rpd(&self) -> RPD_R {
        RPD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER2_RECEIVEPOLLDEMANDREGISTER")
            .field("rpd", &self.rpd())
            .finish()
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register2_receivepolldemandregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register2_receivepolldemandregister::R`](R) reader structure"]
impl crate::Readable for REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER2_RECEIVEPOLLDEMANDREGISTER to value 0"]
impl crate::Resettable for REGISTER2_RECEIVEPOLLDEMANDREGISTER_SPEC {}
