#[doc = "Register `DOEPDMAB11` reader"]
pub type R = crate::R<DOEPDMAB11_SPEC>;
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
        f.debug_struct("DOEPDMAB11")
            .field("dmabufferaddr", &self.dmabufferaddr())
            .finish()
    }
}
#[doc = "This register contains the DMA Buffer Address for the OUT Endpoint 11 of the Device controller.\n\nYou can [`read`](crate::Reg::read) this register and get [`doepdmab11::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DOEPDMAB11_SPEC;
impl crate::RegisterSpec for DOEPDMAB11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`doepdmab11::R`](R) reader structure"]
impl crate::Readable for DOEPDMAB11_SPEC {}
#[doc = "`reset()` method sets DOEPDMAB11 to value 0"]
impl crate::Resettable for DOEPDMAB11_SPEC {}
