#[doc = "Register `RES2` reader"]
pub type R = crate::R<RES2_SPEC>;
#[doc = "Field `DATA` reader - Second result data from calculate"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Second result data from calculate"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES2").field("data", &self.data()).finish()
    }
}
#[doc = "Cordic result 2 receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`res2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RES2_SPEC;
impl crate::RegisterSpec for RES2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res2::R`](R) reader structure"]
impl crate::Readable for RES2_SPEC {}
#[doc = "`reset()` method sets RES2 to value 0"]
impl crate::Resettable for RES2_SPEC {}
