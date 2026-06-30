#[doc = "Register `REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER` reader"]
pub type R = crate::R<REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER_SPEC>;
#[doc = "Field `CH2_RPD` reader - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 _Current Host Receive Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the reception returns to the Suspended state and Bit 7 _RU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the Rx DMA returns to the active state"]
pub type CH2_RPD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Receive Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 19 _Current Host Receive Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the reception returns to the Suspended state and Bit 7 _RU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the Rx DMA returns to the active state"]
    #[inline(always)]
    pub fn ch2_rpd(&self) -> CH2_RPD_R {
        CH2_RPD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER")
            .field("ch2_rpd", &self.ch2_rpd())
            .finish()
    }
}
#[doc = "Used by the Host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register130_channel2receivepolldemandregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register130_channel2receivepolldemandregister::R`](R) reader structure"]
impl crate::Readable for REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER to value 0"]
impl crate::Resettable for REGISTER130_CHANNEL2RECEIVEPOLLDEMANDREGISTER_SPEC {}
