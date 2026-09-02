#[doc = "Register `CH%sDATA` reader"]
pub type R = crate::R<CHDATA_SPEC>;
#[doc = "Field `DATA` reader - Read and write data for channel %s via APB FIFO."]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Read and write data for channel %s via APB FIFO."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHDATA")
            .field("data", &self.data())
            .finish()
    }
}
#[doc = "The read and write data register for CHANNEL%s by apb fifo access.\n\nYou can [`read`](crate::Reg::read) this register and get [`chdata::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHDATA_SPEC;
impl crate::RegisterSpec for CHDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chdata::R`](R) reader structure"]
impl crate::Readable for CHDATA_SPEC {}
#[doc = "`reset()` method sets CH%sDATA to value 0"]
impl crate::Resettable for CHDATA_SPEC {}
