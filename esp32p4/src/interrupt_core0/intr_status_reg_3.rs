#[doc = "Register `INTR_STATUS_REG_3` reader"]
pub type R = crate::R<INTR_STATUS_REG_3_SPEC>;
#[doc = "Field `CORE0_INTR_STATUS_3` reader - NA"]
pub type CORE0_INTR_STATUS_3_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core0_intr_status_3(&self) -> CORE0_INTR_STATUS_3_R {
        CORE0_INTR_STATUS_3_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_STATUS_REG_3")
            .field("core0_intr_status_3", &self.core0_intr_status_3().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_STATUS_REG_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_STATUS_REG_3_SPEC;
impl crate::RegisterSpec for INTR_STATUS_REG_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status_reg_3::R`](R) reader structure"]
impl crate::Readable for INTR_STATUS_REG_3_SPEC {}
#[doc = "`reset()` method sets INTR_STATUS_REG_3 to value 0"]
impl crate::Resettable for INTR_STATUS_REG_3_SPEC {
    const RESET_VALUE: u32 = 0;
}
