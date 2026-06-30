#[doc = "Register `HCDMAB9` reader"]
pub type R = crate::R<HCDMAB9_SPEC>;
#[doc = "Field `HCDMAB` reader - Holds the current buffer address. This register is updated as and when the data transfer for the corresponding end point is in progress. This register is present only in Scatter/Gather DMA mode. Otherwise this field is reserved."]
pub type HCDMAB_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Holds the current buffer address. This register is updated as and when the data transfer for the corresponding end point is in progress. This register is present only in Scatter/Gather DMA mode. Otherwise this field is reserved."]
    #[inline(always)]
    pub fn hcdmab(&self) -> HCDMAB_R {
        HCDMAB_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HCDMAB9")
            .field("hcdmab", &self.hcdmab())
            .finish()
    }
}
#[doc = "This register is present only in case of Scatter/Gather DMA. It is implemented in RAM instead of flop-based implementation. This register holds the current buffer address.\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdmab9::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HCDMAB9_SPEC;
impl crate::RegisterSpec for HCDMAB9_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hcdmab9::R`](R) reader structure"]
impl crate::Readable for HCDMAB9_SPEC {}
#[doc = "`reset()` method sets HCDMAB9 to value 0"]
impl crate::Resettable for HCDMAB9_SPEC {}
