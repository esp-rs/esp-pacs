#[doc = "Register `PRO_INTR_STATUS_2` reader"]
pub type R = crate::R<PRO_INTR_STATUS_2_SPEC>;
#[doc = "Field `PRO_INTR_STATUS_2` reader - This register stores the status of the last 31 input interrupt sources."]
pub type PRO_INTR_STATUS_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the status of the last 31 input interrupt sources."]
    #[inline(always)]
    pub fn pro_intr_status_2(&self) -> PRO_INTR_STATUS_2_R {
        PRO_INTR_STATUS_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTR_STATUS_2")
            .field("pro_intr_status_2", &self.pro_intr_status_2().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_INTR_STATUS_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Interrupt status register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_intr_status_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_INTR_STATUS_2_SPEC;
impl crate::RegisterSpec for PRO_INTR_STATUS_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_intr_status_2::R`](R) reader structure"]
impl crate::Readable for PRO_INTR_STATUS_2_SPEC {}
#[doc = "`reset()` method sets PRO_INTR_STATUS_2 to value 0"]
impl crate::Resettable for PRO_INTR_STATUS_2_SPEC {
    const RESET_VALUE: u32 = 0;
}
