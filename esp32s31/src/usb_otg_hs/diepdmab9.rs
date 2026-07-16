#[doc = "Register `DIEPDMAB9` reader"]
pub type R = crate::R<DIEPDMAB9_SPEC>;
#[doc = "Field `DMABUFFERADDR` reader - Holds the current buffer address.This register is updated as and when the data transfer for the corresponding end point is in progress. This register is present only in Scatter/Gather DMA mode. Otherwise this field is reserved."]
pub type DMABUFFERADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Holds the current buffer address.This register is updated as and when the data transfer for the corresponding end point is in progress. This register is present only in Scatter/Gather DMA mode. Otherwise this field is reserved."]
    #[inline(always)]
    pub fn dmabufferaddr(&self) -> DMABUFFERADDR_R {
        DMABUFFERADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DIEPDMAB9")
            .field("dmabufferaddr", &self.dmabufferaddr())
            .finish()
    }
}
#[doc = "This register contains the DMA Buffer Address of the IN Endpoint 9 of the Device controller. Note: This register exists for an endpoint i if the OTG_EP_DIR_i parameter is 0 or 1 for that endpoint.\n\nYou can [`read`](crate::Reg::read) this register and get [`diepdmab9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DIEPDMAB9_SPEC;
impl crate::RegisterSpec for DIEPDMAB9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`diepdmab9::R`](R) reader structure"]
impl crate::Readable for DIEPDMAB9_SPEC {}
#[doc = "`reset()` method sets DIEPDMAB9 to value 0"]
impl crate::Resettable for DIEPDMAB9_SPEC {}
