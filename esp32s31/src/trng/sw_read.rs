#[doc = "Register `SW_READ` reader"]
pub type R = crate::R<SW_READ_SPEC>;
#[doc = "Field `SW_RANDOM_DATA` reader - sw read random reg"]
pub type SW_RANDOM_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - sw read random reg"]
    #[inline(always)]
    pub fn sw_random_data(&self) -> SW_RANDOM_DATA_R {
        SW_RANDOM_DATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SW_READ")
            .field("sw_random_data", &self.sw_random_data())
            .finish()
    }
}
#[doc = "sw read random reg\n\nYou can [`read`](crate::Reg::read) this register and get [`sw_read::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SW_READ_SPEC;
impl crate::RegisterSpec for SW_READ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sw_read::R`](R) reader structure"]
impl crate::Readable for SW_READ_SPEC {}
#[doc = "`reset()` method sets SW_READ to value 0"]
impl crate::Resettable for SW_READ_SPEC {}
