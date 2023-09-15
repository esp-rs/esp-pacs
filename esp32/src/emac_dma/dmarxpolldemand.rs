#[doc = "Register `DMARXPOLLDEMAND` reader"]
pub type R = crate::R<DMARXPOLLDEMAND_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMARXPOLLDEMAND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "When these bits are written with any value the DMA reads the current descriptor to which the Current Host Receive Descriptor Register is pointing. If that descriptor is not available (owned by the Host) the reception returns to the Suspended state and Bit\\[7\\] (RU) of Status Register is asserted. If the descriptor is available the Rx DMA returns to the active state.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmarxpolldemand::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXPOLLDEMAND_SPEC;
impl crate::RegisterSpec for DMARXPOLLDEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxpolldemand::R`](R) reader structure"]
impl crate::Readable for DMARXPOLLDEMAND_SPEC {}
#[doc = "`reset()` method sets DMARXPOLLDEMAND to value 0"]
impl crate::Resettable for DMARXPOLLDEMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
