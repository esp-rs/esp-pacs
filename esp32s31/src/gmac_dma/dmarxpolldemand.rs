#[doc = "Register `DMARXPOLLDEMAND` reader"]
pub type R = crate::R<DMARXPOLLDEMAND_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Used by the host to instruct the DMA to poll the Receive Descriptor list\n\nYou can [`read`](crate::Reg::read) this register and get [`dmarxpolldemand::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DMARXPOLLDEMAND_SPEC;
impl crate::RegisterSpec for DMARXPOLLDEMAND_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmarxpolldemand::R`](R) reader structure"]
impl crate::Readable for DMARXPOLLDEMAND_SPEC {}
#[doc = "`reset()` method sets DMARXPOLLDEMAND to value 0"]
impl crate::Resettable for DMARXPOLLDEMAND_SPEC {}
