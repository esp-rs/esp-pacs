#[doc = "Register `PRO_INTR_STATUS_1` reader"]
pub type R = crate::R<PRO_INTR_STATUS_1_SPEC>;
#[doc = "Field `PRO_INTR_STATUS_1` reader - This register stores the status of the second 32 input interrupt sources."]
pub type PRO_INTR_STATUS_1_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the status of the second 32 input interrupt sources."]
    #[inline(always)]
    pub fn pro_intr_status_1(&self) -> PRO_INTR_STATUS_1_R {
        PRO_INTR_STATUS_1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTR_STATUS_1")
            .field("pro_intr_status_1", &self.pro_intr_status_1())
            .finish()
    }
}
#[doc = "Interrupt status register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_INTR_STATUS_1_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_intr_status_1::R`](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_1_SPEC {}
#[doc = "`reset()` method sets PRO_INTR_STATUS_1 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_1_SPEC {
    const RESET_VALUE: u32 = 0;
}
