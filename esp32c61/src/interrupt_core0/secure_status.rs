#[doc = "Register `SECURE_STATUS` reader"]
pub type R = crate::R<SECURE_STATUS_SPEC>;
#[doc = "Field `INT_SECURE_STATUS` reader - reserved"]
pub type INT_SECURE_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - reserved"]
    #[inline(always)]
    pub fn int_secure_status(&self) -> INT_SECURE_STATUS_R {
        INT_SECURE_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SECURE_STATUS")
            .field("int_secure_status", &self.int_secure_status())
            .finish()
    }
}
#[doc = "reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`secure_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SECURE_STATUS_SPEC;
impl crate::RegisterSpec for SECURE_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`secure_status::R`](R) reader structure"]
impl crate::Readable for SECURE_STATUS_SPEC {}
#[doc = "`reset()` method sets SECURE_STATUS to value 0"]
impl crate::Resettable for SECURE_STATUS_SPEC {}
