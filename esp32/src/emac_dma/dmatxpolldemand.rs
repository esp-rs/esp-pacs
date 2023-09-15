#[doc = "Register `DMATXPOLLDEMAND` reader"]
pub type R = crate::R<DMATXPOLLDEMAND_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMATXPOLLDEMAND_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "When these bits are written with any value the DMA reads the current descriptor to which the Register (Current Host Transmit Descriptor Register) is pointing. If that descriptor is not available (owned by the Host) the transmission returns to the suspend state and Bit\\[2\\] (TU) of Status Register is asserted. If the descriptor is available the transmission resumes.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`dmatxpolldemand::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMATXPOLLDEMAND_SPEC;
impl crate::RegisterSpec for DMATXPOLLDEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmatxpolldemand::R`](R) reader structure"]
impl crate::Readable for DMATXPOLLDEMAND_SPEC {}
#[doc = "`reset()` method sets DMATXPOLLDEMAND to value 0"]
impl crate::Resettable for DMATXPOLLDEMAND_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
