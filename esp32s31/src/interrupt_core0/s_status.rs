#[doc = "Register `S_STATUS` reader"]
pub type R = crate::R<S_STATUS_SPEC>;
#[doc = "Field `INT_S_STATUS` reader - "]
pub type INT_S_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_s_status(&self) -> INT_S_STATUS_R {
        INT_S_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("S_STATUS")
            .field("int_s_status", &self.int_s_status())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`s_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct S_STATUS_SPEC;
impl crate::RegisterSpec for S_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`s_status::R`](R) reader structure"]
impl crate::Readable for S_STATUS_SPEC {}
#[doc = "`reset()` method sets S_STATUS to value 0"]
impl crate::Resettable for S_STATUS_SPEC {}
