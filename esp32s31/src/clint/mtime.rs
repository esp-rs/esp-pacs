#[doc = "Register `MTIME` reader"]
pub type R = crate::R<MTIME_SPEC>;
#[doc = "Field `MTIME` reader - Represents the current value of the system counter."]
pub type MTIME_R = crate::FieldReader<u64>;
impl R {
    #[doc = "Bits 0:63 - Represents the current value of the system counter."]
    #[inline(always)]
    pub fn mtime(&self) -> MTIME_R {
        MTIME_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MTIME")
            .field("mtime", &self.mtime())
            .finish()
    }
}
#[doc = "Core-local system counter value\n\nYou can [`read`](crate::Reg::read) this register and get [`mtime::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MTIME_SPEC;
impl crate::RegisterSpec for MTIME_SPEC {
    type Ux = u64;
}
#[doc = "`read()` method returns [`mtime::R`](R) reader structure"]
impl crate::Readable for MTIME_SPEC {}
#[doc = "`reset()` method sets MTIME to value 0"]
impl crate::Resettable for MTIME_SPEC {}
