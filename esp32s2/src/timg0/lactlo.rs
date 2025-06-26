#[doc = "Register `LACTLO` reader"]
pub type R = crate::R<LACTLO_SPEC>;
#[doc = "Field `LO` reader - Reserved."]
pub type LO_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Reserved."]
    #[inline(always)]
    pub fn lo(&self) -> LO_R {
        LO_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LACTLO").field("lo", &self.lo()).finish()
    }
}
#[doc = "LACT low register\n\nYou can [`read`](crate::Reg::read) this register and get [`lactlo::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LACTLO_SPEC;
impl crate::RegisterSpec for LACTLO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lactlo::R`](R) reader structure"]
impl crate::Readable for LACTLO_SPEC {}
#[doc = "`reset()` method sets LACTLO to value 0"]
impl crate::Resettable for LACTLO_SPEC {}
