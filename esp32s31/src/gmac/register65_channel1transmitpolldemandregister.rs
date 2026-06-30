#[doc = "Register `REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER` reader"]
pub type R = crate::R<REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER_SPEC>;
#[doc = "Field `CH1_TPD` reader - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 _Current Host Transmit Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the transmission returns to the Suspend state and Bit 2 _TU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the transmission resumes"]
pub type CH1_TPD_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Transmit Poll Demand When these bits are written with any value, the DMA reads the current descriptor to which the Register 18 _Current Host Transmit Descriptor Register_ is pointing If that descriptor is not available _owned by the Host_, the transmission returns to the Suspend state and Bit 2 _TU_ of Register 5 _Status Register_ is asserted If the descriptor is available, the transmission resumes"]
    #[inline(always)]
    pub fn ch1_tpd(&self) -> CH1_TPD_R {
        CH1_TPD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER")
            .field("ch1_tpd", &self.ch1_tpd())
            .finish()
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Transmit Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`register65_channel1transmitpolldemandregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register65_channel1transmitpolldemandregister::R`](R) reader structure"]
impl crate::Readable for REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER to value 0"]
impl crate::Resettable for REGISTER65_CHANNEL1TRANSMITPOLLDEMANDREGISTER_SPEC {}
