#[doc = "Register `PRO_INTR_STATUS_2` reader"]
pub type R = crate::R<PRO_INTR_STATUS_2_SPEC>;
#[doc = "Field `INTR_STATUS_2` reader - this register store the status of the first 32 interrupt source"]
pub type INTR_STATUS_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this register store the status of the first 32 interrupt source"]
    #[inline(always)]
    pub fn intr_status_2(&self) -> INTR_STATUS_2_R {
        INTR_STATUS_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTR_STATUS_2")
            .field("intr_status_2", &self.intr_status_2())
            .finish()
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_intr_status_2::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_INTR_STATUS_2_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_intr_status_2::R`](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_2_SPEC {}
#[doc = "`reset()` method sets PRO_INTR_STATUS_2 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_2_SPEC {}
