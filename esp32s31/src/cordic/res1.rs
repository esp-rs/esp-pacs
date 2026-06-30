#[doc = "Register `RES1` reader"]
pub type R = crate::R<RES1_SPEC>;
#[doc = "Field `DATA` reader - First result data from calculate"]
pub type DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - First result data from calculate"]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RES1").field("data", &self.data()).finish()
    }
}
#[doc = "Cordic result 1 receive register\n\nYou can [`read`](crate::Reg::read) this register and get [`res1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RES1_SPEC;
impl crate::RegisterSpec for RES1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`res1::R`](R) reader structure"]
impl crate::Readable for RES1_SPEC {}
#[doc = "`reset()` method sets RES1 to value 0"]
impl crate::Resettable for RES1_SPEC {}
