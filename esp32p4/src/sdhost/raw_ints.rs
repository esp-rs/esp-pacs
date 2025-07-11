#[doc = "Register `RAW_INTS` reader"]
pub type R = crate::R<RAW_INTS_SPEC>;
#[doc = "Field `RAW_INTS` reader - It indicates raw ints."]
pub type RAW_INTS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - It indicates raw ints."]
    #[inline(always)]
    pub fn raw_ints(&self) -> RAW_INTS_R {
        RAW_INTS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW_INTS")
            .field("raw_ints", &self.raw_ints())
            .finish()
    }
}
#[doc = "SDIO raw ints register.\n\nYou can [`read`](crate::Reg::read) this register and get [`raw_ints::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_INTS_SPEC;
impl crate::RegisterSpec for RAW_INTS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_ints::R`](R) reader structure"]
impl crate::Readable for RAW_INTS_SPEC {}
#[doc = "`reset()` method sets RAW_INTS to value 0"]
impl crate::Resettable for RAW_INTS_SPEC {}
