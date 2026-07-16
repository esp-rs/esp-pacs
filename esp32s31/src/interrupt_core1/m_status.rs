#[doc = "Register `M_STATUS` reader"]
pub type R = crate::R<M_STATUS_SPEC>;
#[doc = "Field `INT_M_STATUS` reader - "]
pub type INT_M_STATUS_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn int_m_status(&self) -> INT_M_STATUS_R {
        INT_M_STATUS_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M_STATUS")
            .field("int_m_status", &self.int_m_status())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`m_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M_STATUS_SPEC;
impl crate::RegisterSpec for M_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m_status::R`](R) reader structure"]
impl crate::Readable for M_STATUS_SPEC {}
#[doc = "`reset()` method sets M_STATUS to value 0"]
impl crate::Resettable for M_STATUS_SPEC {}
