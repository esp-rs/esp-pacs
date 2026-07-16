#[doc = "Register `REGISTER1_TRANSMITPOLLDEMANDREGISTER` reader"]
pub type R = crate::R<REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC>;
#[doc = "Field `TPD` reader - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 _Current Host Transmit Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the transmission returns to the Suspend state and Bit 2 _TU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the transmission resumes"]
pub type TPD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 _Current Host Transmit Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the transmission returns to the Suspend state and Bit 2 _TU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the transmission resumes"]
    #[inline(always)]
    pub fn tpd(&self) -> TPD_R {
        TPD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER1_TRANSMITPOLLDEMANDREGISTER")
            .field("tpd", &self.tpd())
            .finish()
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register1_transmitpolldemandregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register1_transmitpolldemandregister::R`](R) reader structure"]
impl crate::Readable for REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER1_TRANSMITPOLLDEMANDREGISTER to value 0"]
impl crate::Resettable for REGISTER1_TRANSMITPOLLDEMANDREGISTER_SPEC {}
