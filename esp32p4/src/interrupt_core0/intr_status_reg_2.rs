#[doc = "Register `INTR_STATUS_REG_2` reader"]
pub type R = crate::R<INTR_STATUS_REG_2_SPEC>;
#[doc = "Field `CORE0_INTR_STATUS_2` reader - NA"]
pub type CORE0_INTR_STATUS_2_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn core0_intr_status_2(&self) -> CORE0_INTR_STATUS_2_R {
        CORE0_INTR_STATUS_2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTR_STATUS_REG_2")
            .field(
                "core0_intr_status_2",
                &format_args!("{}", self.core0_intr_status_2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTR_STATUS_REG_2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`intr_status_reg_2::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTR_STATUS_REG_2_SPEC;
impl crate::RegisterSpec for INTR_STATUS_REG_2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intr_status_reg_2::R`](R) reader structure"]
impl crate::Readable for INTR_STATUS_REG_2_SPEC {}
#[doc = "`reset()` method sets INTR_STATUS_REG_2 to value 0"]
impl crate::Resettable for INTR_STATUS_REG_2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
