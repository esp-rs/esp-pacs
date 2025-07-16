#[doc = "Register `DATE` reader"]
pub type R = crate::R<DATE_SPEC>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{}", self.bits())
    }
}
#[doc = "Version control register\n\nYou can [`read`](crate::Reg::read) this register and get [`date::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DATE_SPEC;
impl crate::RegisterSpec for DATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`date::R`](R) reader structure"]
impl crate::Readable for DATE_SPEC {}
#[doc = "`reset()` method sets DATE to value 0x0024_c6ec"]
impl crate::Resettable for DATE_SPEC {
    const RESET_VALUE: u32 = 0x0024_c6ec;
}
