#[doc = "Register `PRO_INTR_STATUS_1` reader"]
pub type R = crate::R<PRO_INTR_STATUS_1_SPEC>;
#[doc = "Field `INTR_STATUS_1` reader - this register store the status of the first 32 interrupt source"]
pub type INTR_STATUS_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - this register store the status of the first 32 interrupt source"]
    #[inline(always)]
    pub fn intr_status_1(&self) -> INTR_STATUS_1_R {
        INTR_STATUS_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTR_STATUS_1")
            .field("intr_status_1", &self.intr_status_1())
            .finish()
    }
}
#[doc = "interrupt status register\n\nYou can [`read`](crate::Reg::read) this register and get [`pro_intr_status_1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_INTR_STATUS_1_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_intr_status_1::R`](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_1_SPEC {}
#[doc = "`reset()` method sets PRO_INTR_STATUS_1 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_1_SPEC {}
