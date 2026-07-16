#[doc = "Register `CORE_0_INTR_STATUS5` reader"]
pub type R = crate::R<CORE_0_INTR_STATUS5_SPEC>;
#[doc = "Field `INT_STATUS_5` reader - "]
pub type INT_STATUS_5_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:8"]
    #[inline(always)]
    pub fn int_status_5(&self) -> INT_STATUS_5_R {
        INT_STATUS_5_R::new((self.bits & 0x01ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE_0_INTR_STATUS5")
            .field("int_status_5", &self.int_status_5())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`core_0_intr_status5::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CORE_0_INTR_STATUS5_SPEC;
impl crate::RegisterSpec for CORE_0_INTR_STATUS5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`core_0_intr_status5::R`](R) reader structure"]
impl crate::Readable for CORE_0_INTR_STATUS5_SPEC {}
#[doc = "`reset()` method sets CORE_0_INTR_STATUS5 to value 0"]
impl crate::Resettable for CORE_0_INTR_STATUS5_SPEC {}
